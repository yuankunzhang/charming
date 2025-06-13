use crate::element::{CoordinateSystem, Label, LabelLayout, LineStyle, ScaleLimit, Tooltip};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphLayoutCircular {
    rotate_label: Option<bool>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphLayoutForce {
    init_layout: Option<String>,
    gravity: Option<f64>,
    edge_length: Option<f64>,
    layout_animation: Option<bool>,
    friction: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum GraphLayout {
    None,
    Circular,
    Force,
}

impl From<&str> for GraphLayout {
    fn from(s: &str) -> Self {
        match s {
            "none" => Self::None,
            "circular" => Self::Circular,
            "force" => Self::Force,
            _ => panic!("Invalid Layout"),
        }
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphNodeLabel {
    show: Option<bool>,
    position: Option<String>,
    formatter: Option<String>,
    color: Option<String>,
    font_size: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub value: f64,
    pub category: u64,
    pub symbol_size: f64,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<GraphNodeLabel>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphCategory {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
    pub categories: Vec<GraphCategory>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    #[serde(rename = "type")]
    #[charming_type = "graph"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    legend_hover_link: Option<bool>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    polar_axis_index: Option<f64>,
    geo_index: Option<f64>,
    calendar_index: Option<f64>,
    layout: Option<GraphLayout>,
    circular: Option<GraphLayoutCircular>,
    force: Option<GraphLayoutForce>,
    roam: Option<bool>,
    label: Option<Label>,
    label_layout: Option<LabelLayout>,
    scale_limit: Option<ScaleLimit>,
    line_style: Option<LineStyle>,
    #[charming_skip_setter]
    categories: Vec<GraphCategory>,
    #[charming_skip_setter]
    links: Vec<GraphLink>,
    #[charming_skip_setter]
    data: Vec<GraphNode>,
    #[charming_skip_setter]
    edge_symbol: Option<(String, String)>,
    tooltip: Option<Tooltip>,
}

impl Graph {
    pub fn data(mut self, data: GraphData) -> Self {
        self.data = data.nodes;
        self.links = data.links;
        self.categories = data.categories;
        self
    }

    pub fn edge_symbol(mut self, edge_symbol: Option<(String, String)>) -> Self {
        self.edge_symbol = edge_symbol;
        self
    }
}
