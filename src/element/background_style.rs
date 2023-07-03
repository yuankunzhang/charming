use serde::Serialize;

use super::{border_type::BorderType, color::Color};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_type: Option<BorderType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
}

impl BackgroundStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            border_color: None,
            border_width: None,
            border_type: None,
            border_radius: None,
            opacity: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn border_width(mut self, border_width: u64) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = Some(border_type);
        self
    }

    pub fn border_radius(mut self, border_radius: u64) -> Self {
        self.border_radius = Some(border_radius);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }
}
