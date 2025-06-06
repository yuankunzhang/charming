use super::line_style::LineStyle;
use crate::datatype::CompositeValue;
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisTick {
    show: Option<bool>,
    split_number: Option<f64>,
    length: Option<f64>,
    distance: Option<f64>,
    line_style: Option<LineStyle>,
    #[charming_set_vec]
    custom_values: Vec<CompositeValue>,
}
