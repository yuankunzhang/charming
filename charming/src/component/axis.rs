use crate::{
    datatype::CompositeValue,
    element::{
        AxisLabel, AxisLine, AxisPointer, AxisTick, AxisType, BoundaryGap, NameLocation, SplitArea,
        SplitLine, TextStyle,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// Axis in cartesian coordinate.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Axis {
    /// Type of axis.
    #[serde(rename = "type")]
    type_: Option<AxisType>,
    /// Id of axis.
    id: Option<String>,
    /// Whether to show the axis.
    show: Option<bool>,
    /// Index of grid which is used to place this axis.
    grid_index: Option<f64>,
    /// Offset of this axis relative to default position.
    offset: Option<f64>,
    /// Name of axis.
    name: Option<String>,
    /// Location of axis name.
    name_location: Option<NameLocation>,
    /// Text style of axis name.
    name_text_style: Option<TextStyle>,
    /// Gap between axis name and axis line.
    name_gap: Option<f64>,
    /// Rotation of axis name
    name_rotation: Option<f64>,
    /// Set this to `true` to invert the axis.
    inverse: Option<bool>,
    align_ticks: Option<bool>,
    /// The boundary gap on both sides of a coordinate axis.
    boundary_gap: Option<BoundaryGap>,
    position: Option<CompositeValue>,
    /// The mimimum value of axis.
    min: Option<CompositeValue>,
    /// The maximum value of axis.
    max: Option<CompositeValue>,
    scale: Option<bool>,
    /// Number of segments that the axis is split into.
    split_number: Option<f64>,
    /// Minimum gap between split lines.
    min_interval: Option<f64>,
    /// Maximum gap between split lines.
    max_interval: Option<f64>,
    /// Compulsively set segmentation interval for axis.
    interval: Option<f64>,
    /// Base of logarithm, which is valid only for numeric axes with `log` type.
    log_base: Option<f64>,
    start_value: Option<f64>,
    /// Value of all graphical elements in x axis.
    z_level: Option<f64>,
    /// Value of all graphical elements in x axis, which controls order of drawing graphical components.
    z: Option<f64>,
    /// Settings related to axis label.
    axis_label: Option<AxisLabel>,
    /// Settings related to axis tick.
    axis_tick: Option<AxisTick>,
    /// Settings related to axis line.
    axis_line: Option<AxisLine>,
    /// Settings related to axis pointer.
    axis_pointer: Option<AxisPointer>,
    /// Settings related to split area.
    split_area: Option<SplitArea>,
    /// Settings related to split line.
    split_line: Option<SplitLine>,
    #[charming_set_vec]
    data: Vec<String>,
}
