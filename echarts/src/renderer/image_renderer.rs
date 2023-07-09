use deno_core::{v8, JsRuntime, RuntimeOptions};
use gdk_pixbuf::traits::PixbufLoaderExt;
use handlebars::Handlebars;

use crate::Chart;

static CODE_TEMPLATE: &str = r#"
const chart = echarts.init(null, null, {
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
            width,
            height,
        }
    }

    pub fn render(&mut self, chart: &Chart) -> String {
        let code = Handlebars::new()
            .render_template(
                CODE_TEMPLATE,
                &serde_json::json!({
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
                    Ok(value) => value.as_str().unwrap().to_string(),
                    Err(error) => panic!("{}", error.to_string()),
                }
            }
            Err(error) => panic!("{}", error.to_string()),
        }
    }

    pub fn render_format(&mut self, chart: &Chart, fmt: ImageFormat) -> Vec<u8> {
        let svg = self.render(chart);

        let loader = gdk_pixbuf::PixbufLoader::with_mime_type("image/svg+xml").unwrap();
        loader.write(svg.as_bytes()).unwrap();
        loader.close().unwrap();

        let pixbuf = loader.pixbuf().unwrap();
        pixbuf.save_to_bufferv(&fmt.to_string(), &[]).unwrap()
    }

    pub fn save(&mut self, chart: &Chart, path: &str) {
        let svg = self.render(chart);
        std::fs::write(path, svg).unwrap();
    }

    pub fn save_format(&mut self, chart: &Chart, fmt: ImageFormat, path: &str) {
        let bytes = self.render_format(chart, fmt);
        std::fs::write(path, bytes).unwrap();
    }
}
