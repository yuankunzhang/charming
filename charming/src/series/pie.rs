use serde::Serialize;

use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{ColorBy, CoordinateSystem, Emphasis, ItemStyle, Label, LabelLine},
};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PieRoseType {
    Radius,
    Area,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pie {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordiate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    geo_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected_mode: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected_offset: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clockwise: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avoid_label_overlap: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_angle: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rose_type: Option<PieRoseType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label_line: Option<LabelLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl Pie {
    pub fn new() -> Self {
        Pie {
            type_: "pie".to_string(),
            id: None,
            name: None,
            color_by: None,
            legend_hover_link: None,
            coordiate_system: None,
            geo_index: None,
            calendar_index: None,
            selected_mode: None,
            selected_offset: None,
            clockwise: None,
            avoid_label_overlap: None,
            start_angle: None,
            rose_type: None,
            label: None,
            label_line: None,
            item_style: None,
            emphasis: None,
            center: None,
            radius: None,
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

    pub fn color_by<C: Into<ColorBy>>(mut self, color_by: C) -> Self {
        self.color_by = Some(color_by.into());
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn coordiate_system<C: Into<CoordinateSystem>>(mut self, coordiate_system: C) -> Self {
        self.coordiate_system = Some(coordiate_system.into());
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

    pub fn selected_mode(mut self, selected_mode: bool) -> Self {
        self.selected_mode = Some(selected_mode);
        self
    }

    pub fn selected_offset<F: Into<f64>>(mut self, selected_offset: F) -> Self {
        self.selected_offset = Some(selected_offset.into());
        self
    }

    pub fn clockwise(mut self, clockwise: bool) -> Self {
        self.clockwise = Some(clockwise);
        self
    }

    pub fn avoid_label_overlap(mut self, avoid_label_overlap: bool) -> Self {
        self.avoid_label_overlap = Some(avoid_label_overlap);
        self
    }

    pub fn start_angle<F: Into<f64>>(mut self, start_angle: F) -> Self {
        self.start_angle = Some(start_angle.into());
        self
    }

    pub fn rose_type<P: Into<PieRoseType>>(mut self, rose_type: P) -> Self {
        self.rose_type = Some(rose_type.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn label_line<L: Into<LabelLine>>(mut self, label_line: L) -> Self {
        self.label_line = Some(label_line.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn center<C: Into<CompositeValue>>(mut self, center: C) -> Self {
        self.center = Some(center.into());
        self
    }

    pub fn radius<C: Into<CompositeValue>>(mut self, radius: C) -> Self {
        self.radius = Some(radius.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
