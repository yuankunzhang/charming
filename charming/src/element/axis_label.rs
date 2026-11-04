use super::{
    Formatter,
    color::Color,
    font_settings::{FontFamily, FontStyle, FontWeight},
};
use crate::{
    datatype::CompositeValue,
    element::{LabelAlign, TextStyle},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisLabel {
    show: Option<bool>,
    inside: Option<bool>,
    distance: Option<f64>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_family: Option<FontFamily>,
    font_size: Option<f64>,
    color: Option<Color>,
    formatter: Option<Formatter>,
    rich: Option<HashMap<String, TextStyle>>,
    rotate: Option<f64>,
    interval: Option<f64>,
    align: Option<LabelAlign>,
    align_min_label: Option<LabelAlign>,
    align_max_label: Option<LabelAlign>,
    custom_values: Vec<CompositeValue>,
}
