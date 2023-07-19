use std::io::Cursor;

use deno_core::{v8, JsRuntime, RuntimeOptions};
use handlebars::Handlebars;
use image::RgbaImage;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{self, TreeTextToPath},
};

use crate::{theme::Theme, Chart, EchartsError};

static CODE_TEMPLATE: &str = r#"
{{#if theme_source}}{{{ theme_source }}}{{/if}}
var chart = echarts.init(null, {{#if theme}}'{{ theme }}'{{else}}null{{/if}}, {
    renderer: 'svg',
    ssr: true,
    width: {{ width }},
    height: {{ height }}
});

chart.setOption({ animation: false });
chart.setOption({{{ chart_option }}});
chart.renderToSVGString();
"#;

pub use image::ImageFormat;

pub struct ImageRenderer {
    js_runtime: JsRuntime,
    fontdb: usvg::fontdb::Database,
    theme: Theme,
    width: u32,
    height: u32,
}

impl ImageRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut runtime = JsRuntime::new(RuntimeOptions::default());
        runtime
            .execute_script(
                "[runtime.js]",
                include_str!("../asset/runtime.js").to_string().into(),
            )
            .unwrap();
        runtime
            .execute_script(
                "[echarts.js]",
                include_str!("../asset/echarts-5.4.2.min.js")
                    .to_string()
                    .into(),
            )
            .unwrap();

        let mut fontdb = usvg::fontdb::Database::default();
        fontdb.load_system_fonts();

        #[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
        {
            set_default_fonts(&mut fontdb);
        }

        Self {
            js_runtime: runtime,
            fontdb,
            theme: Theme::Default,
            width,
            height,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Render chart to an SVG String
    pub fn render(&mut self, chart: &Chart) -> Result<String, EchartsError> {
        let (theme, theme_source) = self.theme.to_str();
        let code = Handlebars::new()
            .render_template(
                CODE_TEMPLATE,
                &serde_json::json!({
                    "theme": theme,
                    "theme_source": theme_source,
                    "width": self.width,
                    "height": self.height,
                    "chart_option": chart.to_string(),
                }),
            )
            .expect("Failed to render template");
        let result = self.js_runtime.execute_script("[anon]", code.into());

        match result {
            Ok(global) => {
                let scope = &mut self.js_runtime.handle_scope();
                let local = v8::Local::new(scope, global);
                let value = serde_v8::from_v8::<serde_json::Value>(scope, local);

                match value {
                    Ok(value) => Ok(value.as_str().unwrap().to_string()),
                    Err(error) => Err(EchartsError::JsRuntimeError(error.to_string())),
                }
            }
            Err(error) => Err(EchartsError::JsRuntimeError(error.to_string())),
        }
    }

    /// Render a chart to a given image format in bytes
    pub fn render_format(
        &mut self,
        image_format: ImageFormat,
        chart: &Chart,
    ) -> Result<Vec<u8>, EchartsError> {
        let svg = self.render(chart)?;

        let img = self.render_svg_to_buf(&svg)?;

        // give buf initial capacity of: width * height * num of channels for RGBA + room for headers/metadata
        let estimated_capacity = self.width * self.height * 4 + 1024;
        let mut buf = Vec::with_capacity(estimated_capacity as usize);
        img.write_to(&mut Cursor::new(&mut buf), image_format)
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))?;
        Ok(buf)
    }

    /// Given an svg str, render it into an [`image::ImageBuffer`]
    fn render_svg_to_buf(&mut self, svg: &str) -> Result<image::RgbaImage, EchartsError> {
        let mut pixels =
            Pixmap::new(self.width, self.height).ok_or(EchartsError::ImageRenderingError(
                "Rendered image cannot be greater than i32::MAX/4".to_string(),
            ))?;

        let mut tree: usvg::Tree =
            usvg::TreeParsing::from_data(svg.as_bytes(), &usvg::Options::default())
                .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))?;

        tree.convert_text(&self.fontdb);
        resvg::Tree::from_usvg(&tree).render(usvg::Transform::identity(), &mut pixels.as_mut());

        let img = RgbaImage::from_vec(self.width, self.height, pixels.take()).ok_or(
            EchartsError::ImageRenderingError(
                "Could not create ImageBuffer from bytes".to_string(),
            ),
        )?;

        Ok(img)
    }

    /// Render and save chart as an SVG
    pub fn save<P: AsRef<std::path::Path>>(
        &mut self,
        chart: &Chart,
        path: P,
    ) -> Result<(), EchartsError> {
        let svg = self.render(chart)?;
        std::fs::write(path, svg)
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))
    }

    /// Render and save chart as the given image format
    pub fn save_format<P: AsRef<std::path::Path>>(
        &mut self,
        image_format: ImageFormat,
        chart: &Chart,
        path: P,
    ) -> Result<(), EchartsError> {
        let svg = self.render(chart)?;
        let img = self.render_svg_to_buf(&svg)?;
        img.save_with_format(path, image_format)
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))
    }
}

#[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
fn set_default_fonts(fontdb: &mut usvg::fontdb::Database) {
    let sans_serif_fonts = vec![
        "DejaVu Sans",
        "FreeSans",
        "Liberation Sans",
        "Arimo",
        "Cantarell",
        "Nimbus Sans",
    ];

    let serif_fonts = vec![
        "DejaVu Serif",
        "FreeSerif",
        "Liberation Serif",
        "Tinos",
        "Nimbus Roman",
    ];

    let monospace_fonts = vec![
        "DejaVu Sans Mono",
        "FreeMono",
        "Liberation Mono",
        "Nimbus Mono",
    ];

    for font in sans_serif_fonts {
        if font_exists(fontdb, font) {
            fontdb.set_sans_serif_family(font);
            break;
        }
    }

    for font in serif_fonts {
        if font_exists(fontdb, font) {
            fontdb.set_serif_family(font);
            break;
        }
    }

    for font in monospace_fonts {
        if font_exists(fontdb, font) {
            fontdb.set_monospace_family(font);
            break;
        }
    }
}

#[cfg(all(unix, not(any(target_os = "macos", target_os = "android"))))]
fn font_exists(fontdb: &usvg::fontdb::Database, family: &str) -> bool {
    fontdb
        .query(&usvg::fontdb::Query {
            families: &[usvg::fontdb::Family::Name(family)],
            weight: usvg::fontdb::Weight(14),
            stretch: usvg::fontdb::Stretch::Normal,
            style: usvg::fontdb::Style::Normal,
        })
        .is_some()
}
