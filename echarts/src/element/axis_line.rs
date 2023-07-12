use serde::{ser::SerializeSeq, Serialize};

use super::color::Color;

pub struct ColorSegment(f64, Color);

impl Serialize for ColorSegment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.0)?;
        seq.serialize_element(&self.1)?;
        seq.end()
    }
}

impl From<(f64, &str)> for ColorSegment {
    fn from(tuple: (f64, &str)) -> Self {
        Self(tuple.0, tuple.1.into())
    }
}

impl From<(f64, Color)> for ColorSegment {
    fn from(tuple: (f64, Color)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisLineStyle {
    color: Vec<ColorSegment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_blur: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_y: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
}

impl AxisLineStyle {
    pub fn new() -> Self {
        Self {
            color: vec![],
            width: None,
            shadow_blur: None,
            shadow_color: None,
            shadow_offset_x: None,
            shadow_offset_y: None,
            opacity: None,
        }
    }

    pub fn color<C: Into<ColorSegment>>(mut self, color: C) -> Self {
        self.color.push(color.into());
        self
    }

    pub fn width<F: Into<f64>>(mut self, width: F) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn shadow_blur<F: Into<f64>>(mut self, shadow_blur: F) -> Self {
        self.shadow_blur = Some(shadow_blur.into());
        self
    }

    pub fn shadow_color<C: Into<Color>>(mut self, shadow_color: C) -> Self {
        self.shadow_color = Some(shadow_color.into());
        self
    }

    pub fn shadow_offset_x<F: Into<f64>>(mut self, shadow_offset_x: F) -> Self {
        self.shadow_offset_x = Some(shadow_offset_x.into());
        self
    }

    pub fn shadow_offset_y<F: Into<f64>>(mut self, shadow_offset_y: F) -> Self {
        self.shadow_offset_y = Some(shadow_offset_y.into());
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }
}

impl From<(f64, &str)> for AxisLineStyle {
    fn from(tuple: (f64, &str)) -> Self {
        Self::new().color(tuple)
    }
}

impl From<(f64, &str, f64)> for AxisLineStyle {
    fn from(tuple: (f64, &str, f64)) -> Self {
        Self::new().color((tuple.0, tuple.1)).width(tuple.2)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    on_zero: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    round_cap: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<AxisLineStyle>,
}

impl AxisLine {
    pub fn new() -> Self {
        Self {
            show: None,
            on_zero: None,
            round_cap: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn on_zero(mut self, on_zero: bool) -> Self {
        self.on_zero = Some(on_zero);
        self
    }

    pub fn round_cap(mut self, round_cap: bool) -> Self {
        self.round_cap = Some(round_cap);
        self
    }

    pub fn line_style<S: Into<AxisLineStyle>>(mut self, line_style: S) -> Self {
        self.line_style = Some(line_style.into());
        self
    }
}
