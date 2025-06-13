#![allow(dead_code)]

use super::{Color, Formatter, Padding, TextStyle, Trigger};
use crate::datatype::CompositeValue;
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateTooltip {
    show: Option<bool>,
    trigger: Option<Trigger>,
    position: Option<CompositeValue>,
    formatter: Option<Formatter>,
    value_formatter: Option<Formatter>,
    background_color: Option<Color>,
    border_color: Option<Color>,
    padding: Option<Padding>,
    text_style: Option<TextStyle>,
}
