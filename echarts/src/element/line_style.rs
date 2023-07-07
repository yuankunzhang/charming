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

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn width<F: Into<f64>>(mut self, width: F) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn type_<L: Into<LineStyleType>>(mut self, type_: L) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }

    pub fn curveness<F: Into<f64>>(mut self, curveness: F) -> Self {
        self.curveness = Some(curveness.into());
        self
    }
}
