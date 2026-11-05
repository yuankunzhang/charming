use crate::{datatype::CompositeValue, element::Orient};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Value,
    Category,
    Time,
    Log,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SingleAxis {
    breaks: Option<Vec<crate::element::AxisBreak>>,
    break_area: Option<crate::element::BreakArea>,
    break_label_layout: Option<crate::element::BreakLabelLayout>,
    jitter: Option<f64>,
    jitter_overlap: Option<bool>,
    jitter_margin: Option<f64>,
    name_move_overlap: Option<crate::element::BooleanOrAuto>,
    contain_shape: Option<bool>,
    data_min: Option<crate::datatype::StringOrNumber>,
    data_max: Option<crate::datatype::StringOrNumber>,
    tooltip: Option<crate::element::AxisTooltip>,
    coordinate_system: Option<crate::element::CoordinateSystem>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
    #[serde(rename = "type")]
    type_: Option<Type>,
    name: Option<String>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    orient: Option<Orient>,
    inverse: Option<bool>,
    min: Option<String>,
    max: Option<String>,
    start_value: Option<f64>,
}
