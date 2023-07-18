use std::io::Cursor;

use deno_core::{v8, JsRuntime, RuntimeOptions};
use handlebars::Handlebars;

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
#[derive(PartialEq)]
pub enum ImageFormat {
    SVG,
    Other(image::ImageFormat),
}

pub struct ImageRenderer {
    js_runtime: JsRuntime,
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

        Self {
            js_runtime: runtime,
            theme: Theme::Default,
            width,
            height,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

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

    pub fn render_format(
        &mut self,
        image_format: ImageFormat,
        chart: &Chart,
    ) -> Result<Vec<u8>, EchartsError> {
        let svg = self.render(chart)?;

        match image_format {
            ImageFormat::SVG => Ok(svg.as_bytes().to_vec()),
            ImageFormat::Other(format) => {
                let img = self.render_svg_to_buf(&svg)?;

                let mut buf = Vec::new();
                img.write_to(&mut Cursor::new(&mut buf), format)
                    .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))?;
                Ok(buf)
            }
        }
    }

    pub fn render_svg_to_buf(&mut self, svg: &str) -> Result<image::RgbaImage, EchartsError> {
        let mut pixels = resvg::tiny_skia::Pixmap::new(self.width, self.height)
            .expect("width smaller than i32::MAX/4");

        let tree: resvg::usvg::Tree =
            resvg::usvg::TreeParsing::from_data(svg.as_bytes(), &resvg::usvg::Options::default())
                .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))?;

        resvg::Tree::from_usvg(&tree).render(
            resvg::tiny_skia::Transform::identity(),
            &mut pixels.as_mut(),
        );

        let img = image::RgbaImage::from_vec(self.width, self.height, pixels.take()).ok_or(
            EchartsError::ImageRenderingError(
                "Could not create ImageBUffer from bytes".to_string(),
            ),
        )?;

        Ok(img)
    }

    pub fn save<P: AsRef<std::path::Path>>(
        &mut self,
        chart: &Chart,
        path: P,
    ) -> Result<(), EchartsError> {
        let svg = self.render(chart)?;
        std::fs::write(path, svg)
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))
    }

    pub fn save_format<P: AsRef<std::path::Path>>(
        &mut self,
        image_format: ImageFormat,
        chart: &Chart,
        path: P,
    ) -> Result<(), EchartsError> {
        let svg = self.render(chart)?;
        match image_format {
            ImageFormat::SVG => std::fs::write(path, svg)
                .map_err(|error| EchartsError::ImageRenderingError(error.to_string())),
            ImageFormat::Other(format) => {
                let img = self.render_svg_to_buf(&svg)?;
                img.save_with_format(path, format)
                    .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))
            }
        }
    }
}
