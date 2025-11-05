use crate::{
    datatype::{DataFrame, DataPoint, Dimension},
    element::{
        bar_width::BarWidth, selected_mode::SelectedMode,
        universal_transition::UniversalTransition, AnimationTime, Blur, ColorBy, Coordinate,
        CoordinateSystem, CoordinateSystemUsage, DimensionEncode, Emphasis, ItemStyle, MarkArea,
        MarkLine, MarkPoint, Select, Tooltip,
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
pub struct Candlestick {
    #[serde(rename = "type")]
    #[charming_type = "candlestick"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    item_style: Option<ItemStyle>,
    coordiate_system: Option<CoordinateSystem>,
    color_by: Option<ColorBy>,
    legend_hover_link: Option<bool>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
    coordinate_system_usage: Option<CoordinateSystemUsage>,
    coord: Option<Coordinate>,
    x_axis_index: Option<f64>,
    x_axis_id: Option<f64>,
    y_axis_index: Option<f64>,
    y_axis_id: Option<f64>,
    bar_width: Option<f64>,
    bar_min_width: Option<BarWidth>,
    bar_max_width: Option<BarWidth>,
    emphasis: Option<Emphasis>,
    blur: Option<Blur>,
    select: Option<Select>,
    selected_mode: Option<SelectedMode>,
    large: Option<bool>,
    large_threshold: Option<f64>,
    progressive: Option<f64>,
    progressive_threshold: Option<f64>,
    progressive_chunk_mode: Option<String>,
    dimensions: Option<Vec<Dimension>>,
    encode: Option<DimensionEncode>,
    data_group_id: Option<String>,
    mark_point: Option<MarkPoint>,
    mark_line: Option<MarkLine>,
    mark_area: Option<MarkArea>,
    clip: Option<bool>,
    zlevel: Option<f64>,
    z: Option<f64>,
    silent: Option<bool>,
    animation_duration: Option<AnimationTime>,
    animation_easing: Option<String>,
    animation_delay: Option<AnimationTime>,
    universal_transition: Option<UniversalTransition>,
}
