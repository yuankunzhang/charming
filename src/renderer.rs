use deno_core::{v8, JsRuntime, RuntimeOptions};
use handlebars::Handlebars;
use std::fmt;

use crate::style::Theme;

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

pub struct Renderer {
    pub theme: Theme,
    pub width: u32,
    pub height: u32,
    pub ssr: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            theme: Theme::Default,
            width: 800,
            height: 600,
            ssr: true,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn ssr(mut self, ssr: bool) -> Self {
        self.ssr = ssr;
        self
    }

    fn init_chart(&self) -> String {
        Handlebars::new()
            .render_template(
                r#"
const chart = echarts.init(null, '{{ theme }}', {
    renderer: 'svg',
    ssr: {{ ssr}},
    width: {{ width }},
    height: {{ height }}
});
"#,
                &serde_json::json!({"theme": self.theme.to_string(), "width": self.width, "height": self.height, "ssr": self.ssr}),
            )
            .expect("Failed to render template")
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_init_chart() {
        use super::Renderer;
        let renderer = Renderer::new()
            .theme(super::Theme::Dark)
            .size(1000, 900)
            .ssr(false);
        let code = renderer.init_chart();
        println!("{}", code);
        assert_eq!(
            code,
            r#"
const chart = echarts.init(null, 'dark', {
    renderer: 'svg',
    ssr: false,
    width: 1000,
    height: 900
});
"#
        );
    }
}
