use serde::Serialize;

use crate::{
    datatype::DataFrame,
    element::{CoordinateSystem, Emphasis, ItemStyle, Label},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Heatmap {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    geo_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    point_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    blur_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_opacity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_opacity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    progressive: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    progressive_threshold: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<DataFrame>,
}

impl Heatmap {
    pub fn new() -> Self {
        Self {
            type_: "heatmap".to_string(),
            id: None,
            name: None,
            coordinate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            geo_index: None,
            calendar_index: None,
            point_size: None,
            blur_size: None,
            min_opacity: None,
            max_opacity: None,
            progressive: None,
            progressive_threshold: None,
            label: None,
            item_style: None,
            emphasis: None,
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

    pub fn coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(coordinate_system);
        self
    }

    pub fn x_axis_index<F: Into<f64>>(mut self, x_axis_index: F) -> Self {
        self.x_axis_index = Some(x_axis_index.into());
        self
    }

    pub fn y_axis_index<F: Into<f64>>(mut self, y_axis_index: F) -> Self {
        self.y_axis_index = Some(y_axis_index.into());
        self
    }

    pub fn geo_index<F: Into<f64>>(mut self, geo_index: F) -> Self {
        self.geo_index = Some(geo_index.into());
        self
    }

    pub fn calendar_index<F: Into<f64>>(mut self, calendar_index: F) -> Self {
        self.calendar_index = Some(calendar_index.into());
        self
    }

    pub fn point_size<F: Into<f64>>(mut self, point_size: F) -> Self {
        self.point_size = Some(point_size.into());
        self
    }

    pub fn blur_size<F: Into<f64>>(mut self, blur_size: F) -> Self {
        self.blur_size = Some(blur_size.into());
        self
    }

    pub fn min_opacity<F: Into<f64>>(mut self, min_opacity: F) -> Self {
        self.min_opacity = Some(min_opacity.into());
        self
    }

    pub fn max_opacity<F: Into<f64>>(mut self, max_opacity: F) -> Self {
        self.max_opacity = Some(max_opacity.into());
        self
    }

    pub fn progressive<F: Into<f64>>(mut self, progressive: F) -> Self {
        self.progressive = Some(progressive.into());
        self
    }

    pub fn progressive_threshold<F: Into<f64>>(mut self, progressive_threshold: F) -> Self {
        self.progressive_threshold = Some(progressive_threshold.into());
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }

    pub fn emphasis(mut self, emphasis: Emphasis) -> Self {
        self.emphasis = Some(emphasis);
        self
    }

    pub fn data<S: Into<DataFrame>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
