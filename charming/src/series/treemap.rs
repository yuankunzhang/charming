use crate::{
    datatype::CompositeValue,
    element::{Emphasis, ItemStyle, Label, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Treemap {
    roam: Option<crate::element::Roam>,
    roam_trigger: Option<crate::element::RoamTrigger>,
    center: Option<crate::datatype::CompositeValue>,
    zoom: Option<f64>,
    scale_limit: Option<crate::element::ScaleLimit>,
    cursor: Option<crate::element::Cursor>,
    coordinate_system: Option<crate::element::CoordinateSystem>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
    #[serde(rename = "type")]
    #[charming_type = "treemap"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    zlevel: Option<f64>,
    z: Option<f64>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<TreemapNode>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TreemapNode {
    name: Option<String>,
    value: Option<CompositeValue>,
    cursor: Option<crate::element::Cursor>,
    #[charming_set_vec]
    children: Vec<TreemapNode>,
}
