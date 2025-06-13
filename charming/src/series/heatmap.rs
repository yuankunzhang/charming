use crate::{
    datatype::DataFrame,
    element::{CoordinateSystem, Emphasis, ItemStyle, Label, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Heatmap {
    #[serde(rename = "type")]
    #[charming_type = "heatmap"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    geo_index: Option<f64>,
    calendar_index: Option<f64>,
    point_size: Option<f64>,
    blur_size: Option<f64>,
    min_opacity: Option<f64>,
    max_opacity: Option<f64>,
    progressive: Option<f64>,
    progressive_threshold: Option<f64>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<DataFrame>,
}
