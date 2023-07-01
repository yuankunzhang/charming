use serde::Serialize;

use super::color::Color;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginPosition {
    Auto,
    Start,
    End,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<OriginPosition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
}

impl AreaStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            origin: None,
            opacity: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn origin(mut self, origin: OriginPosition) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }
}
