use crate::{
    datatype::CompositeValue,
    element::{Emphasis, ItemStyle, Label, LineStyle, Orient, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SankeyNodeAlign {
    Left,
    Right,
    Justify,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SankeyNode {
    pub name: String,
    pub value: Option<f64>,
    pub depth: Option<f64>,
    pub item_style: Option<ItemStyle>,
}

impl SankeyNode {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            value: None,
            depth: None,
            item_style: None,
        }
    }

    pub fn value<V: Into<f64>>(mut self, value: V) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn depth<D: Into<f64>>(mut self, depth: D) -> Self {
        self.depth = Some(depth.into());
        self
    }

    pub fn item_style<S: Into<ItemStyle>>(mut self, item_style: S) -> Self {
        self.item_style = Some(item_style.into());
        self
    }
}

impl<S> From<S> for SankeyNode
where
    S: Into<String>,
{
    fn from(name: S) -> Self {
        SankeyNode {
            name: name.into(),
            value: None,
            depth: None,
            item_style: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SankeyLink {
    pub source: String,
    pub target: String,
    pub value: f64,
}

impl<S, F> From<(S, S, F)> for SankeyLink
where
    S: Into<String>,
    F: Into<f64>,
{
    fn from((source, target, value): (S, S, F)) -> Self {
        SankeyLink {
            source: source.into(),
            target: target.into(),
            value: value.into(),
        }
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sankey {
    #[serde(rename = "type")]
    #[charming_type = "sankey"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    z_level: Option<f64>,
    z: Option<f64>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<f64>,
    height: Option<f64>,
    emphasis: Option<Emphasis>,
    layout_iterations: Option<u64>,
    orient: Option<Orient>,
    label: Option<Label>,
    node_align: Option<SankeyNodeAlign>,
    line_style: Option<LineStyle>,
    #[charming_set_vec]
    links: Vec<SankeyLink>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<SankeyNode>,
}

impl Sankey {
    pub fn nodes<S: Into<SankeyNode>>(mut self, nodes: Vec<S>) -> Self {
        self.data = nodes.into_iter().map(|s| s.into()).collect();
        self
    }
}
