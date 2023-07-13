use handlebars::Handlebars;

use crate::{theme::Theme, Chart, EchartsError};

pub struct HtmlRenderer {
    title: String,
    theme: Theme,
    width: u64,
    height: u64,
}

impl HtmlRenderer {
    pub fn new<S: Into<String>>(title: S, width: u64, height: u64) -> Self {
        Self {
            title: title.into(),
            theme: Theme::Default,
            width,
            height,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn render(&self, chart: &Chart) -> Result<String, EchartsError> {
        let template = include_str!("../asset/charts.html.hbs");
        let (theme, theme_source) = self.theme.to_str();
        let data = Handlebars::new()
            .render_template(
                template,
                &serde_json::json!({
                    "title": self.title,
                    "theme": theme,
                    "theme_source": theme_source,
                    "width": self.width,
                    "height": self.height,
                    "chart_id": "chart",
                    "chart_option": chart.to_string(),
                }),
            )
            .map_err(|error| EchartsError::HtmlRenderingError(error.to_string()))?;
        Ok(data)
    }
}
