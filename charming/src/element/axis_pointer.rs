use crate::{
    datatype::CompositeValue,
    element::{Label, LineStyle},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AxisPointerType {
    Line,
    Shadow,
    Cross,
    None,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AxisPointerAxis {
    X,
    Y,
    Radius,
    Angle,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointerLink {
    x_axis_index: Option<CompositeValue>,
    x_axis_name: Option<String>,
    y_axis_index: Option<CompositeValue>,
    y_axis_name: Option<String>,
}

/// Axis Pointer is a tool for displaying reference line and axis value under
/// mouse pointer.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointer {
    /// Component ID.
    id: Option<String>,
    /// Whether to show the axis pointer.
    show: Option<bool>,
    /// Indicator type.
    #[serde(rename = "type")]
    type_: Option<AxisPointerType>,
    snap: Option<bool>,
    animation: Option<bool>,
    z: Option<f64>,
    axis: Option<AxisPointerAxis>,
    /// Label of axis pointer.
    label: Option<Label>,
    /// Line style of axis pointer.
    line_style: Option<LineStyle>,
    /// Axis pointer can be linked to each other.
    #[charming_set_vec]
    link: Vec<AxisPointerLink>,
}
