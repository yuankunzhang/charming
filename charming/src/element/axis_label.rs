use serde::Serialize;

use super::{
    color::Color,
    font_settings::{FontFamily, FontStyle, FontWeight},
    Formatter,
};

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_style: Option<FontStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_weight: Option<FontWeight>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<FontFamily>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<Formatter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rotate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<f64>,
}

impl Default for AxisLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl AxisLabel {
    pub fn new() -> Self {
        Self {
            show: None,
            distance: None,
            font_style: None,
            font_weight: None,
            font_family: None,
            font_size: None,
            color: None,
            formatter: None,
            rotate: None,
            interval: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn distance<F: Into<f64>>(mut self, distance: F) -> Self {
        self.distance = Some(distance.into());
        self
    }

    pub fn font_style<F: Into<FontStyle>>(mut self, font_style: F) -> Self {
        self.font_style = Some(font_style.into());
        self
    }

    pub fn font_weight<F: Into<FontWeight>>(mut self, font_weight: F) -> Self {
        self.font_weight = Some(font_weight.into());
        self
    }

    pub fn font_family<F: Into<FontFamily>>(mut self, font_family: F) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    pub fn font_size<F: Into<f64>>(mut self, font_size: F) -> Self {
        self.font_size = Some(font_size.into());
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn formatter<F: Into<Formatter>>(mut self, formatter: F) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn rotate<F: Into<f64>>(mut self, rotate: F) -> Self {
        self.rotate = Some(rotate.into());
        self
    }

    pub fn interval<F: Into<f64>>(mut self, interval: F) -> Self {
        self.interval = Some(interval.into());
        self
    }
}
