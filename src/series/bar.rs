use std::vec;

use serde::{Deserialize, Serialize};

use crate::utility::{
    background_style::BackgroundStyle, color::ColorBy, coordinate::CoordinateSystem,
    emphasis::Emphasis, label::Label,
};

#[derive(Serialize, Deserialize)]
pub struct DataPoint {
    pub value: Vec<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub type Data = Vec<DataPoint>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bar {
    #[serde(rename = "type")]
    type_: String,

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

    data: Data,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            type_: "bar".to_string(),
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

    pub fn data<F: Into<f64>>(mut self, data: Vec<F>) -> Self {
        for (i, d) in data.into_iter().enumerate() {
            self.data.push(DataPoint {
                value: vec![(i as f64).into(), d.into()],
                name: None,
            });
        }
        self
    }
}
