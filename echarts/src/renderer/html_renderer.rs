use handlebars::Handlebars;

use crate::Chart;

pub struct HtmlRenderer {
    title: String,
    width: u64,
    height: u64,
}

impl HtmlRenderer {
    pub fn new<S: Into<String>>(title: S, width: u64, height: u64) -> Self {
        Self {
            title: title.into(),
            width,
            height,
        }
    }

    pub fn render(self, chart: Chart) -> String {
        let template = include_str!("../asset/charts.html.hbs");
        let data = Handlebars::new()
            .render_template(
                template,
                &serde_json::json!({
                    "title": self.title,
                    "width": self.width,
                    "height": self.height,
                    "chart_id": "chart",
                    "chart_option": chart.to_string(),
                }),
            )
            .expect("Failed to render template");
        data
    }
}
