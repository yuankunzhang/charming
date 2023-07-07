use std::vec;

use serde::Serialize;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{BackgroundStyle, ColorBy, CoordinateSystem, Emphasis, ItemStyle, Label, MarkLine},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bar {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    polar_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    round_cap: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_sort: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show_background: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_style: Option<BackgroundStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphais: Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mark_line: Option<MarkLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bar_width: Option<f64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            type_: "bar".to_string(),
            id: None,
            name: None,
            color_by: None,
            legend_hover_link: None,
            coordinate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            polar_index: None,
            round_cap: None,
            realtime_sort: None,
            show_background: None,
            background_style: None,
            label: None,
            item_style: None,
            emphais: None,
            mark_line: None,
            stack: None,
            bar_width: None,
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

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordiate_system: C) -> Self {
        self.coordinate_system = Some(coordiate_system.into());
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

    pub fn polar_index<F: Into<f64>>(mut self, polar_index: F) -> Self {
        self.polar_index = Some(polar_index.into());
        self
    }

    pub fn round_cap(mut self, round_cap: bool) -> Self {
        self.round_cap = Some(round_cap);
        self
    }

    pub fn realtime_sort(mut self, realtime_sort: bool) -> Self {
        self.realtime_sort = Some(realtime_sort);
        self
    }

    pub fn show_background(mut self, show_background: bool) -> Self {
        self.show_background = Some(show_background);
        self
    }

    pub fn background_style<S: Into<BackgroundStyle>>(mut self, background_style: S) -> Self {
        self.background_style = Some(background_style.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn item_style<S: Into<ItemStyle>>(mut self, item_style: S) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphais = Some(emphasis.into());
        self
    }

    pub fn mark_line<M: Into<MarkLine>>(mut self, mark_line: M) -> Self {
        self.mark_line = Some(mark_line.into());
        self
    }

    pub fn stack<S: Into<String>>(mut self, stack: S) -> Self {
        self.stack = Some(stack.into());
        self
    }

    pub fn bar_width<F: Into<f64>>(mut self, bar_width: F) -> Self {
        self.bar_width = Some(bar_width.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
