use crate::element::{
    AxisLabel, AxisLine, AxisPointer, AxisTick, AxisType, BoundaryGap, MinorSplitLine, MinorTick,
    SplitArea, SplitLine,
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// The angle axis in Polar Coordinate.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AngleAxis {
    boundary_gap: Option<BoundaryGap>,
    /// Component ID.
    id: Option<String>,
    /// The index of angle axis in Polar Coordinate.
    polar_index: Option<f64>,
    /// Starting angle of axis, default to 90.
    start_angle: Option<f64>,
    end_angle: Option<f64>,
    /// Whether the direction of axis is clockwise, default to true.
    clockwise: Option<bool>,
    /// Type of axis.
    #[serde(rename = "type")]
    type_: Option<AxisType>,
    zlevel: Option<f64>,
    z: Option<f64>,
    /// The minimun value of axis.
    min: Option<f64>,
    /// The maximum value of axis.
    max: Option<f64>,
    scale: Option<bool>,
    split_number: Option<f64>,
    min_interval: Option<f64>,
    max_interval: Option<f64>,
    interval: Option<f64>,
    log_base: Option<f64>,
    start_value: Option<f64>,
    silent: Option<bool>,
    trigger_event: Option<bool>,
    axis_line: Option<AxisLine>,
    axis_tick: Option<AxisTick>,
    axis_pointer: Option<AxisPointer>,
    minor_tick: Option<MinorTick>,
    axis_label: Option<AxisLabel>,
    split_line: Option<SplitLine>,
    minor_split_line: Option<MinorSplitLine>,
    split_area: Option<SplitArea>,
    #[charming_set_vec]
    data: Vec<String>,
}
