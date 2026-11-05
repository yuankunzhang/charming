use crate::{
    datatype::CompositeValue,
    element::{Blur, Emphasis, ItemStyle, Label, LineStyle, Select, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChordNode {
    id: Option<String>,
    name: Option<String>,
    value: Option<CompositeValue>,
    item_style: Option<ItemStyle>,
    label: Option<Label>,
    emphasis: Option<Emphasis>,
    blur: Option<Blur>,
    select: Option<Select>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChordData {
    Node(ChordNode),
    Value(CompositeValue),
}

impl From<ChordNode> for ChordData {
    fn from(value: ChordNode) -> Self {
        Self::Node(value)
    }
}

impl<V: Into<CompositeValue>> From<V> for ChordData {
    fn from(value: V) -> Self {
        Self::Value(value.into())
    }
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChordEdge {
    source: Option<CompositeValue>,
    target: Option<CompositeValue>,
    value: Option<f64>,
    line_style: Option<LineStyle>,
    label: Option<Label>,
    emphasis: Option<Emphasis>,
    blur: Option<Blur>,
    select: Option<Select>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chord {
    roam: Option<crate::element::Roam>,
    roam_trigger: Option<crate::element::RoamTrigger>,
    zoom: Option<f64>,
    scale_limit: Option<crate::element::ScaleLimit>,
    single_axis_index: Option<f64>,
    single_axis_id: Option<String>,
    polar_index: Option<f64>,
    polar_id: Option<String>,
    geo_index: Option<f64>,
    geo_id: Option<String>,
    x_axis_index: Option<f64>,
    x_axis_id: Option<String>,
    y_axis_index: Option<f64>,
    y_axis_id: Option<String>,
    coordinate_system: Option<crate::element::CoordinateSystem>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
    #[serde(rename = "type")]
    #[charming_type = "chord"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    legend_hover_link: Option<bool>,
    left: Option<CompositeValue>,
    right: Option<CompositeValue>,
    top: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    center: Option<CompositeValue>,
    radius: Option<CompositeValue>,
    clockwise: Option<bool>,
    start_angle: Option<f64>,
    end_angle: Option<CompositeValue>,
    pad_angle: Option<f64>,
    min_angle: Option<f64>,
    item_style: Option<ItemStyle>,
    label: Option<Label>,
    line_style: Option<LineStyle>,
    edge_label: Option<Label>,
    emphasis: Option<Emphasis>,
    blur: Option<Blur>,
    select: Option<Select>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<ChordData>,
    #[charming_set_vec]
    nodes: Vec<ChordData>,
    #[charming_set_vec]
    edges: Vec<ChordEdge>,
    #[charming_set_vec]
    links: Vec<ChordEdge>,
}
