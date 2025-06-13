use crate::element::{AxisLabel, AxisLine, AxisType, BoundaryGap, NameLocation, TextStyle};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// Radius axis in polar coordinate.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadiusAxis {
    id: Option<String>,
    polar_index: Option<f64>,
    type_: Option<AxisType>,
    name: Option<String>,
    name_location: Option<NameLocation>,
    name_text_style: Option<TextStyle>,
    name_gap: Option<f64>,
    name_rotation: Option<f64>,
    inverse: Option<bool>,
    boundary_gap: Option<BoundaryGap>,
    min: Option<f64>,
    max: Option<f64>,
    scale: Option<bool>,
    split_number: Option<f64>,
    min_interval: Option<f64>,
    max_interval: Option<f64>,
    interval: Option<f64>,
    log_base: Option<f64>,
    start_value: Option<f64>,
    axis_label: Option<AxisLabel>,
    axis_line: Option<AxisLine>,
    #[charming_set_vec]
    data: Vec<String>,
}
