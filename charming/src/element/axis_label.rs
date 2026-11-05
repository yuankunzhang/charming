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
    show_min_label: Option<bool>,
    show_max_label: Option<bool>,
    hide_overlap: Option<bool>,
    margin: Option<f64>,
    silent: Option<bool>,
    rich_inherit_plain_label: Option<bool>,
    min_margin: Option<f64>,
    text_margin: Option<crate::datatype::NumberOrArray>,
    line_overflow: Option<crate::element::LineOverflow>,
    overflow: Option<crate::element::TextOverflow>,
    ellipsis: Option<String>,
    show: Option<bool>,
    inside: Option<bool>,
    distance: Option<f64>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_family: Option<FontFamily>,
    font_size: Option<crate::datatype::StringOrNumber>,
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
