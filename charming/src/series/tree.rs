use crate::{
    datatype::CompositeValue,
    element::{Blur, Emphasis, ItemStyle, Label, Select, Symbol, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TreeLayout {
    Orthogonal,
    Radial,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum TreeOrient {
    #[serde(rename = "LR")]
    LeftRight,
    #[serde(rename = "RL")]
    RightLeft,
    #[serde(rename = "TB")]
    TopBottom,
    #[serde(rename = "BT")]
    BottomTop,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TreeEdgeShape {
    Curve,
    Polyline,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TreeLeaves {
    label: Option<Label>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    pub name: Option<String>,
    pub value: Option<f64>,
    pub collapsed: Option<bool>,
    pub children: Option<Vec<TreeNode>>,
}

/// The tree diagram is mainly used to display the tree data structure.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    #[serde(rename = "type")]
    #[charming_type = "tree"]
    type_: String,
    /// Component ID.
    id: Option<String>,
    /// Component name.
    name: Option<String>,
    /// zlevel value of all graphical elements in the tree.
    z_level: Option<u64>,
    /// z value of all graphical elements in the tree.
    z: Option<u64>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    center: Option<CompositeValue>,
    zoom: Option<f64>,
    layout: Option<TreeLayout>,
    orient: Option<TreeOrient>,
    symbol: Option<Symbol>,
    symbol_size: Option<f64>,
    symbol_rotate: Option<f64>,
    symbol_keep_aspect: Option<bool>,
    symbol_offset: Option<CompositeValue>,
    edge_shape: Option<TreeEdgeShape>,
    edge_fork_position: Option<String>,
    roam: Option<bool>,
    initial_tree_depth: Option<f64>,
    item_style: Option<ItemStyle>,
    label: Option<Label>,
    emphasis: Option<Emphasis>,
    blur: Option<Blur>,
    select: Option<Select>,
    selected_mode: Option<bool>,
    expand_and_collapse: Option<bool>,
    animation_duration: Option<f64>,
    animation_duration_update: Option<f64>,
    leaves: Option<TreeLeaves>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<TreeNode>,
}
