use deno_core::{v8, JsRuntime, RuntimeOptions};
use handlebars::Handlebars;
use std::fmt;

static CODE_TEMPLATE: &str = r#"
const chart = echarts.init(null, null, {
    renderer: 'svg',
    ssr: true,
    width: {{ width }},
    height: {{ height }}
});

chart.setOption({
    title: {
        text: 'ECharts Getting Started Example'
    },
    tooltip: {},
    xAxis: {
        data: ['shirt', 'cardigan', 'chiffon', 'pants', 'heels', 'socks']
    },
    yAxis: {},
    series: [{
        name: 'sales',
        type: 'bar',
        data: [5, 20, 36, 10, 10, 20]
    }]
});

chart.renderToSVGString();
"#;

#[derive(Debug, Clone)]
pub struct RenderError(String);

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

pub fn render() -> Result<Vec<u8>, RenderError> {
    let code = Handlebars::new()
        .render_template(
            CODE_TEMPLATE,
            &serde_json::json!({"width": 800, "height": 600}),
        )
        .expect("Failed to render template");
    _render(code)
}

pub fn render_string() -> Result<String, RenderError> {
    let bytes = render()?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn _render(code: String) -> Result<Vec<u8>, RenderError> {
    let mut runtime = JsRuntime::new(RuntimeOptions::default());
    runtime
        .execute_script(
            "[runtime.js]",
            include_str!("./assets/runtime.js").to_string().into(),
        )
        .unwrap();
    runtime
        .execute_script(
            "[echarts.js]",
            include_str!("./assets/echarts-5.4.2.min.js")
                .to_string()
                .into(),
        )
        .unwrap();
    let result = runtime.execute_script("[anon]", code.into());

    match result {
        Ok(global) => {
            let scope = &mut runtime.handle_scope();
            let local = v8::Local::new(scope, global);
            let value = serde_v8::from_v8::<serde_json::Value>(scope, local);

            match value {
                Ok(value) => Ok(value.as_str().unwrap().as_bytes().to_vec()),
                Err(error) => Err(RenderError(error.to_string())),
            }
        }
        Err(error) => Err(RenderError(error.to_string())),
    }
}
