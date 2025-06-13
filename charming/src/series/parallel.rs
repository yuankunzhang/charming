use crate::{
    datatype::{DataFrame, DataPoint},
    element::{smoothness::Smoothness, ColorBy, CoordinateSystem, Emphasis, LineStyle},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ProgressiveChunkMode {
    Sequential,
    Mod,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Parallel {
    #[serde(rename = "type")]
    #[charming_type = "parallel"]
    type_: String,
    id: Option<String>,
    coordinate_system: Option<CoordinateSystem>,
    parallel_index: Option<f64>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    line_style: Option<LineStyle>,
    emphasis: Option<Emphasis>,
    inactive_opacity: Option<f64>,
    active_opacity: Option<f64>,
    realtime: Option<bool>,
    smooth: Option<Smoothness>,
    progressive: Option<f64>,
    progressive_threshold: Option<f64>,
    progressive_chunk_mode: Option<ProgressiveChunkMode>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
