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
