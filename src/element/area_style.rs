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

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn origin<O: Into<OriginPosition>>(mut self, origin: O) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }
}
