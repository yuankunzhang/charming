use crate::{
    datatype::CompositeValue,
    element::{
        AxisLabel, AxisLine, AxisTick, AxisType, BoundaryGap, NameLocation, ParallelLayout,
        SplitLine, TextStyle,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParallelAxisDefault {
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
    #[serde(rename = "type")]
    type_: Option<AxisType>,
    name: Option<String>,
    name_location: Option<NameLocation>,
    name_text_style: Option<TextStyle>,
    name_gap: Option<f64>,
    name_rotate: Option<f64>,
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
    silent: Option<bool>,
    trigger_event: Option<bool>,
    axis_line: Option<AxisLine>,
    axis_tick: Option<AxisTick>,
    axis_label: Option<AxisLabel>,
    split_line: Option<SplitLine>,
    #[charming_set_vec]
    data: Vec<String>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParallelCoordinate {
    coordinate_system: Option<crate::element::CoordinateSystem>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
    id: Option<String>,
    zlevel: Option<f64>,
    z: Option<f64>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    layout: Option<ParallelLayout>,
    parallel_axis_default: Option<ParallelAxisDefault>,
}
