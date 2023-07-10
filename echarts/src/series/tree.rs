use serde::{Deserialize, Serialize};

use crate::{
    datatype::CompositeValue,
    element::{Blur, Emphasis, ItemStyle, Label, Select, Symbol},
};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeLayout {
    Orthogonal,
    Radial,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreeEdgeShape {
    Curve,
    Polyline,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeLeaves {
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// The tree diagram is mainly used to display the tree data structure.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    #[serde(rename = "type")]
    type_: String,

    /// Component ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// Component name.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// zlevel value of all graphical elements in the tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    z_level: Option<u64>,

    /// z value of all graphical elements in the tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zoom: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<TreeLayout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    orient: Option<TreeOrient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_rotate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_keep_aspect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_offset: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    edge_shape: Option<TreeEdgeShape>,

    #[serde(skip_serializing_if = "Option::is_none")]
    edge_fork_position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    roam: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_tree_depth: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    blur: Option<Blur>,

    #[serde(skip_serializing_if = "Option::is_none")]
    select: Option<Select>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected_mode: Option<bool>,

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
            center: None,
            zoom: None,
            layout: None,
            orient: None,
            symbol: None,
            symbol_size: None,
            symbol_rotate: None,
            symbol_keep_aspect: None,
            symbol_offset: None,
            edge_shape: None,
            edge_fork_position: None,
            roam: None,
            initial_tree_depth: None,
            item_style: None,
            label: None,
            emphasis: None,
            blur: None,
            select: None,
            selected_mode: None,
            expand_and_collapse: None,
            animation_duration: None,
            animation_duration_update: None,
            leaves: None,
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

    pub fn z_level(mut self, z_level: u64) -> Self {
        self.z_level = Some(z_level);
        self
    }

    pub fn z(mut self, z: u64) -> Self {
        self.z = Some(z);
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

    pub fn width<C: Into<CompositeValue>>(mut self, width: C) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<C: Into<CompositeValue>>(mut self, height: C) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn center<C: Into<CompositeValue>>(mut self, center: C) -> Self {
        self.center = Some(center.into());
        self
    }

    pub fn zoom<F: Into<f64>>(mut self, zoom: F) -> Self {
        self.zoom = Some(zoom.into());
        self
    }

    pub fn layout<T: Into<TreeLayout>>(mut self, layout: T) -> Self {
        self.layout = Some(layout.into());
        self
    }

    pub fn orient<T: Into<TreeOrient>>(mut self, orient: T) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn symbol<S: Into<Symbol>>(mut self, symbol: S) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn symbol_size<F: Into<f64>>(mut self, symbol_size: F) -> Self {
        self.symbol_size = Some(symbol_size.into());
        self
    }

    pub fn symbol_rotate<F: Into<f64>>(mut self, symbol_rotate: F) -> Self {
        self.symbol_rotate = Some(symbol_rotate.into());
        self
    }

    pub fn symbol_keep_aspect(mut self, symbol_keep_aspect: bool) -> Self {
        self.symbol_keep_aspect = Some(symbol_keep_aspect);
        self
    }

    pub fn symbol_offset<C: Into<CompositeValue>>(mut self, symbol_offset: C) -> Self {
        self.symbol_offset = Some(symbol_offset.into());
        self
    }

    pub fn edge_shape<T: Into<TreeEdgeShape>>(mut self, edge_shape: T) -> Self {
        self.edge_shape = Some(edge_shape.into());
        self
    }

    pub fn edge_fork_position<S: Into<String>>(mut self, edge_fork_position: S) -> Self {
        self.edge_fork_position = Some(edge_fork_position.into());
        self
    }

    pub fn roam(mut self, roam: bool) -> Self {
        self.roam = Some(roam);
        self
    }

    pub fn initial_tree_depth<F: Into<f64>>(mut self, initial_tree_depth: F) -> Self {
        self.initial_tree_depth = Some(initial_tree_depth.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn blur<B: Into<Blur>>(mut self, blur: B) -> Self {
        self.blur = Some(blur.into());
        self
    }

    pub fn select<S: Into<Select>>(mut self, select: S) -> Self {
        self.select = Some(select.into());
        self
    }

    pub fn selected_mode(mut self, selected_mode: bool) -> Self {
        self.selected_mode = Some(selected_mode);
        self
    }

    pub fn expand_and_collapse(mut self, expand_and_collapse: bool) -> Self {
        self.expand_and_collapse = Some(expand_and_collapse);
        self
    }

    pub fn animation_duration<F: Into<f64>>(mut self, animation_duration: F) -> Self {
        self.animation_duration = Some(animation_duration.into());
        self
    }

    pub fn animation_duration_update<F: Into<f64>>(mut self, animation_duration_update: F) -> Self {
        self.animation_duration_update = Some(animation_duration_update.into());
        self
    }

    pub fn leaves<T: Into<TreeLeaves>>(mut self, leaves: T) -> Self {
        self.leaves = Some(leaves.into());
        self
    }

    pub fn data<T: Into<TreeNode>>(mut self, data: Vec<T>) -> Self {
        self.data = data.into_iter().map(|t| t.into()).collect();
        self
    }
}
