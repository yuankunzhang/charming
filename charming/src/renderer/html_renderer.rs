use handlebars::Handlebars;

use crate::{Chart, EchartsError, component::SaveAsImageType, theme::Theme};

pub struct HtmlRenderer {
    title: String,
    theme: Theme,
    width: u64,
    height: u64,
    locale: Option<String>,
    extensions: Vec<String>,
}

impl HtmlRenderer {
    pub fn new<S: Into<String>>(title: S, width: u64, height: u64) -> Self {
        Self {
            title: title.into(),
            theme: Theme::Default,
            width,
            height,
            locale: None,
            extensions: Vec::new(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Set the locale passed to ECharts initialization.
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Register trusted JavaScript after the engine loads and before initialization.
    pub fn extension_source(mut self, source: impl Into<String>) -> Self {
        self.extensions.push(source.into());
        self
    }

    pub fn render(&self, chart: &Chart) -> Result<String, EchartsError> {
        let template = include_str!("../asset/charts.html.hbs");
        let (theme, theme_source) = self.theme.to_str();
        let canvas_type = match chart.save_as_image_type() {
            Some(&SaveAsImageType::Svg) => "svg".to_string(),
            _ => "canvas".to_string(),
        };
        let mut sources = self.extensions.clone();
        if !theme_source.is_empty() {
            sources.push(theme_source.to_string());
        }
        let data = Handlebars::new()
            .render_template(
                template,
                &serde_json::json!({
                    "title": self.title,
                    "theme": html_json(&theme),
                    "sources": html_json(&sources),
                    "init_options": html_json(&serde_json::json!({
                        "renderer": canvas_type,
                        "locale": self.locale,
                    })),
                    "width": self.width,
                    "height": self.height,
                    "chart_id": "chart",
                    "chart_option": chart.to_javascript()?,
                }),
            )
            .map_err(|error| EchartsError::HtmlRenderingError(error.to_string()))?;
        Ok(data)
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
}

// A script element terminates at </script>, even inside a JavaScript string.
fn html_json(value: &impl serde::Serialize) -> String {
    serde_json::to_string(value)
        .expect("renderer options serialize to JSON")
        .replace('<', "\\u003c")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}
