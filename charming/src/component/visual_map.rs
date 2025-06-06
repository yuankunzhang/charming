use crate::{
    datatype::CompositeValue,
    element::{Color, Orient, TextStyle},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum VisualMapType {
    Continuous,
    Piecewise,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VisualMapPiece {
    min: Option<f64>,
    max: Option<f64>,
    lt: Option<f64>,
    lte: Option<f64>,
    gt: Option<f64>,
    gte: Option<f64>,
    label: Option<String>,
    color: Option<Color>,
}

impl From<(f64, f64)> for VisualMapPiece {
    fn from((min, max): (f64, f64)) -> Self {
        Self::new().min(min).max(max)
    }
}

impl From<(i64, i64)> for VisualMapPiece {
    fn from((min, max): (i64, i64)) -> Self {
        Self::new().min(min as f64).max(max as f64)
    }
}

impl From<(f64, f64, &str)> for VisualMapPiece {
    fn from((min, max, label): (f64, f64, &str)) -> Self {
        Self::new().min(min).max(max).label(label)
    }
}

impl From<(i64, i64, &str)> for VisualMapPiece {
    fn from((min, max, label): (i64, i64, &str)) -> Self {
        Self::new().min(min as f64).max(max as f64).label(label)
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VisualMapChannel {
    #[charming_set_vec]
    color: Vec<Color>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VisualMap {
    #[serde(rename = "type")]
    type_: Option<VisualMapType>,
    #[charming_set_vec]
    color: Vec<Color>,
    show: Option<bool>,
    dimension: Option<CompositeValue>,
    series_index: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,
    #[charming_set_vec]
    categories: Vec<String>,
    calculable: Option<bool>,
    orient: Option<Orient>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    text_style: Option<TextStyle>,
    #[charming_skip_setter]
    range: Option<(f64, f64)>,
    realtime: Option<bool>,
    inverse: Option<bool>,
    precision: Option<f64>,
    item_width: Option<f64>,
    item_height: Option<f64>,
    in_range: Option<VisualMapChannel>,
    out_range: Option<VisualMapChannel>,
    pieces: Option<Vec<VisualMapPiece>>,
}

impl VisualMap {
    pub fn range<F: Into<f64>>(mut self, range: (F, F)) -> Self {
        self.range = Some((range.0.into(), range.1.into()));
        self
    }
}
