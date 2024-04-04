use serde::{Deserialize, Serialize};

use crate::element::{CoordinateSystem, Label, LabelLayout, LineStyle, ScaleLimit};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphLayoutCircular {
    #[serde(skip_serializing_if = "Option::is_none")]
    rotate_label: Option<bool>,
}

impl GraphLayoutCircular {
    pub fn new() -> Self {
        Self { rotate_label: None }
    }

    pub fn rotate_label(mut self, rotate_label: bool) -> Self {
        self.rotate_label = Some(rotate_label);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphLayoutForce {
    #[serde(skip_serializing_if = "Option::is_none")]
    init_layout: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gravity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    edge_length: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout_animation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    friction: Option<f64>,
}

impl GraphLayoutForce {
    pub fn new() -> Self {
        Self {
            init_layout: None,
            gravity: None,
            edge_length: None,
            layout_animation: None,
            friction: None,
        }
    }

    pub fn init_layout<S: Into<String>>(mut self, init_layout: S) -> Self {
        self.init_layout = Some(init_layout.into());
        self
    }

    pub fn gravity(mut self, gravity: f64) -> Self {
        self.gravity = Some(gravity);
        self
    }

    pub fn edge_length(mut self, edge_length: f64) -> Self {
        self.edge_length = Some(edge_length);
        self
    }

    pub fn layout_animation(mut self, layout_animation: bool) -> Self {
        self.layout_animation = Some(layout_animation);
        self
    }

    pub fn friction(mut self, friction: f64) -> Self {
        self.friction = Some(friction);
        self
    }
}

#[derive(Serialize)]
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNodeLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<u64>,
}

impl GraphNodeLabel {
    pub fn new() -> Self {
        Self {
            show: None,
            position: None,
            formatter: None,
            color: None,
            font_size: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn position<S: Into<String>>(mut self, position: S) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn formatter<S: Into<String>>(mut self, formatter: S) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn color<S: Into<String>>(mut self, color: S) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn font_size(mut self, font_size: u64) -> Self {
        self.font_size = Some(font_size);
        self
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphLink {
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphCategory {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
    pub categories: Vec<GraphCategory>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    polar_axis_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    geo_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<GraphLayout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    circular: Option<GraphLayoutCircular>,

    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<GraphLayoutForce>,

    #[serde(skip_serializing_if = "Option::is_none")]
    roam: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label_layout: Option<LabelLayout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale_limit: Option<ScaleLimit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    categories: Vec<GraphCategory>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    links: Vec<GraphLink>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<GraphNode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    edge_symbol: Option<(String, String)>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            type_: "graph".into(),
            id: None,
            name: None,
            legend_hover_link: None,
            coordinate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            polar_axis_index: None,
            geo_index: None,
            calendar_index: None,
            layout: None,
            circular: None,
            force: None,
            roam: None,
            label: None,
            label_layout: None,
            scale_limit: None,
            line_style: None,
            categories: vec![],
            links: vec![],
            data: vec![],
            edge_symbol: None,
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

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(coordinate_system);
        self
    }

    pub fn x_axis_index(mut self, x_axis_index: u64) -> Self {
        self.x_axis_index = Some(x_axis_index);
        self
    }

    pub fn y_axis_index(mut self, y_axis_index: u64) -> Self {
        self.y_axis_index = Some(y_axis_index);
        self
    }

    pub fn polar_axis_index(mut self, polar_axis_index: u64) -> Self {
        self.polar_axis_index = Some(polar_axis_index);
        self
    }

    pub fn geo_index(mut self, geo_index: u64) -> Self {
        self.geo_index = Some(geo_index);
        self
    }

    pub fn calendar_index(mut self, calendar_index: u64) -> Self {
        self.calendar_index = Some(calendar_index);
        self
    }

    pub fn layout(mut self, layout: GraphLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    pub fn circular(mut self, circular: GraphLayoutCircular) -> Self {
        self.circular = Some(circular);
        self
    }

    pub fn force(mut self, force: GraphLayoutForce) -> Self {
        self.force = Some(force);
        self
    }

    pub fn roam(mut self, roam: bool) -> Self {
        self.roam = Some(roam);
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn label_layout(mut self, label_layout: LabelLayout) -> Self {
        self.label_layout = Some(label_layout);
        self
    }

    pub fn scale_limit(mut self, scale_limit: ScaleLimit) -> Self {
        self.scale_limit = Some(scale_limit);
        self
    }

    pub fn line_style(mut self, line_style: LineStyle) -> Self {
        self.line_style = Some(line_style);
        self
    }

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
