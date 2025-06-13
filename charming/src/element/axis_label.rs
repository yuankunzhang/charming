use super::{
    color::Color,
    font_settings::{FontFamily, FontStyle, FontWeight},
    Formatter,
};
use crate::datatype::CompositeValue;
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisLabel {
    show: Option<bool>,
    distance: Option<f64>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_family: Option<FontFamily>,
    font_size: Option<f64>,
    color: Option<Color>,
    formatter: Option<Formatter>,
    rotate: Option<f64>,
    interval: Option<f64>,
    custom_values: Vec<CompositeValue>,
}
