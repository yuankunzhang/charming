use deno_core::{v8, JsRuntime, RuntimeOptions};
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

    pub fn render(mut self, chart: Chart) -> String {
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
}
