use std::vec;

use serde::Serialize;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        background_style::BackgroundStyle, color::ColorBy, coordinate::CoordinateSystem,
        emphasis::Emphasis, label::Label,
    },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bar {
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
    x_axis_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    polar_index: Option<u64>,

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
    emphais: Option<Emphasis>,

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
            coordiate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            polar_index: None,
            round_cap: None,
            realtime_sort: None,
            show_background: None,
            background_style: None,
            label: None,
            emphais: None,
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

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn coordiate_system(mut self, coordiate_system: CoordinateSystem) -> Self {
        self.coordiate_system = Some(coordiate_system);
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

    pub fn polar_index(mut self, polar_index: u64) -> Self {
        self.polar_index = Some(polar_index);
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

    pub fn background_style(mut self, background_style: BackgroundStyle) -> Self {
        self.background_style = Some(background_style);
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn emphasis(mut self, emphasis: Emphasis) -> Self {
        self.emphais = Some(emphasis);
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
