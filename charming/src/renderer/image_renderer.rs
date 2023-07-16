use deno_core::{v8, JsRuntime, RuntimeOptions};
use gdk_pixbuf::traits::PixbufLoaderExt;
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

pub enum ImageFormat {
    SVG,
    PNG,
    JPEG,
}

impl ToString for ImageFormat {
    fn to_string(&self) -> String {
        match self {
            ImageFormat::SVG => "svg".to_string(),
            ImageFormat::PNG => "png".to_string(),
            ImageFormat::JPEG => "jpeg".to_string(),
        }
    }
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

        if let ImageFormat::SVG = image_format {
            return Ok(svg.as_bytes().to_vec());
        }

        let loader = gdk_pixbuf::PixbufLoader::with_mime_type("image/svg+xml")
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))?;
        loader
            .write(svg.as_bytes())
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))?;
        loader
            .close()
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))?;

        let pixbuf = loader.pixbuf().ok_or_else(|| {
            EchartsError::ImageRenderingError("Failed to load pixbuf".to_string())
        })?;

        pixbuf
            .save_to_bufferv(&image_format.to_string(), &[])
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))
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
        let bytes = self.render_format(image_format, chart)?;
        std::fs::write(path, bytes)
            .map_err(|error| EchartsError::ImageRenderingError(error.to_string()))
    }
}
