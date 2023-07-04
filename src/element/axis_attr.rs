use serde::{ser::SerializeSeq, Serialize};

use super::{color::Color, line_style::LineStyle};

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
    round_cap: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<AxisLineStyle>,
}

impl AxisLine {
    pub fn new() -> Self {
        Self {
            show: None,
            round_cap: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisTick {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_number: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,
}

impl AxisTick {
    pub fn new() -> Self {
        Self {
            show: None,
            split_number: None,
            length: None,
            distance: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn split_number<F: Into<f64>>(mut self, split_number: F) -> Self {
        self.split_number = Some(split_number.into());
        self
    }

    pub fn length<F: Into<f64>>(mut self, length: F) -> Self {
        self.length = Some(length.into());
        self
    }

    pub fn distance<F: Into<f64>>(mut self, distance: F) -> Self {
        self.distance = Some(distance.into());
        self
    }

    pub fn line_style<S: Into<LineStyle>>(mut self, line_style: S) -> Self {
        self.line_style = Some(line_style.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
}

impl AxisLabel {
    pub fn new() -> Self {
        Self {
            distance: None,
            font_size: None,
            color: None,
        }
    }

    pub fn distance<F: Into<f64>>(mut self, distance: F) -> Self {
        self.distance = Some(distance.into());
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
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisType {
    Value,
    Category,
    Time,
    Log,
}
