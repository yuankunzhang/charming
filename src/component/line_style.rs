use serde::Serialize;

use super::color::Color;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LineStyleType {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<LineStyleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    curveness: Option<f64>,
}

impl LineStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            width: None,
            type_: None,
            opacity: None,
            curveness: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn type_(mut self, type_: LineStyleType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn curveness(mut self, curveness: f64) -> Self {
        self.curveness = Some(curveness);
        self
    }
}
