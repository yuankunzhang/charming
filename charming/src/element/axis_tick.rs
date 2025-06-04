use serde::{Deserialize, Serialize};

use crate::datatype::CompositeValue;

use super::line_style::LineStyle;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Default)]
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

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    custom_values: Vec<CompositeValue>,
}

impl AxisTick {
    pub fn new() -> Self {
        Self::default()
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

    pub fn custom_values<C: Into<CompositeValue>>(mut self, custom_values: Vec<C>) -> Self {
        self.custom_values = custom_values.into_iter().map(|c| c.into()).collect();
        self
    }
}
