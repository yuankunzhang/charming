use serde::{Deserialize, Serialize};

use crate::{
    datatype::CompositeValue,
    element::{Emphasis, Label, LineStyle, Orient, ItemStyle},
};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SankeyNodeAlign {
    Left,
    Right,
    Justify,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SankeyNode {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_style: Option<ItemStyle>,
}

impl SankeyNode {
    pub fn new<S>(name: S) -> Self where S: Into<String> {
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sankey {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z_level: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout_iterations: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    orient: Option<Orient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    node_align: Option<SankeyNodeAlign>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    links: Vec<SankeyLink>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<SankeyNode>,
}

impl Sankey {
    pub fn new() -> Self {
        Sankey {
            type_: "sankey".to_string(),
            id: None,
            name: None,
            z_level: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            emphasis: None,
            layout_iterations: None,
            orient: None,
            label: None,
            node_align: None,
            line_style: None,
            links: vec![],
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn z_level<F: Into<f64>>(mut self, z_level: F) -> Self {
        self.z_level = Some(z_level.into());
        self
    }

    pub fn z<F: Into<f64>>(mut self, z: F) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width<F: Into<f64>>(mut self, width: F) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<F: Into<f64>>(mut self, height: F) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn layout_iterations<F: Into<u64>>(mut self, layout_iterations: F) -> Self {
        self.layout_iterations = Some(layout_iterations.into());
        self
    }

    pub fn orient<O: Into<Orient>>(mut self, orient: O) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn node_align<S: Into<SankeyNodeAlign>>(mut self, node_align: S) -> Self {
        self.node_align = Some(node_align.into());
        self
    }

    pub fn line_style<L: Into<LineStyle>>(mut self, line_style: L) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn data<S: Into<SankeyNode>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn nodes<S: Into<SankeyNode>>(mut self, nodes: Vec<S>) -> Self {
        self.data = nodes.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn links<S: Into<SankeyLink>>(mut self, links: Vec<S>) -> Self {
        self.links = links.into_iter().map(|s| s.into()).collect();
        self
    }
}
