use serde::{Deserialize, Serialize};

use crate::component::{emphasis::Emphasis, label::Label, symbol::Symbol};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeLayout {
    Orthogonal,
    Radial,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeLeaves {
    label: Option<Label>,
}

impl TreeLeaves {
    pub fn new() -> Self {
        Self { label: None }
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TreeNode>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z_level: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<(f64, f64)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<TreeLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand_and_collapse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    animation_duration_update: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    leaves: Option<TreeLeaves>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<TreeNode>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            type_: "tree".into(),
            name: None,
            z_level: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            center: None,
            layout: None,
            symbol: None,
            symbol_size: None,
            label: None,
            emphasis: None,
            expand_and_collapse: None,
            animation_duration: None,
            animation_duration_update: None,
            leaves: None,
            data: vec![],
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn z_level(mut self, z_level: u64) -> Self {
        self.z_level = Some(z_level);
        self
    }

    pub fn z(mut self, z: u64) -> Self {
        self.z = Some(z);
        self
    }

    pub fn left<S: Into<String>>(mut self, left: S) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<S: Into<String>>(mut self, top: S) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<S: Into<String>>(mut self, right: S) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<S: Into<String>>(mut self, bottom: S) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn center(mut self, center: (f64, f64)) -> Self {
        self.center = Some(center);
        self
    }

    pub fn layout(mut self, layout: TreeLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    pub fn symbol(mut self, symbol: Symbol) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbol_size(mut self, symbol_size: f64) -> Self {
        self.symbol_size = Some(symbol_size);
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn emphasis(mut self, emphasis: Emphasis) -> Self {
        self.emphasis = Some(emphasis);
        self
    }

    pub fn expand_and_collapse(mut self, expand_and_collapse: bool) -> Self {
        self.expand_and_collapse = Some(expand_and_collapse);
        self
    }

    pub fn animation_duration(mut self, animation_duration: f64) -> Self {
        self.animation_duration = Some(animation_duration);
        self
    }

    pub fn animation_duration_update(mut self, animation_duration_update: f64) -> Self {
        self.animation_duration_update = Some(animation_duration_update);
        self
    }

    pub fn leaves(mut self, leaves: TreeLeaves) -> Self {
        self.leaves = Some(leaves);
        self
    }

    pub fn data(mut self, data: Vec<TreeNode>) -> Self {
        self.data = data;
        self
    }
}
