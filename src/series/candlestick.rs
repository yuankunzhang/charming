use serde::Serialize;

use crate::basic::{color::ColorBy, coordinate::CoordinateSystem, DataFrame, DataPoint};

pub type Data = DataFrame;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordiate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    data: Data,
}

impl Candlestick {
    pub fn new() -> Self {
        Self {
            type_: "candlestick".to_string(),
            name: None,
            coordiate_system: None,
            color_by: None,
            legend_hover_link: None,
            data: vec![],
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn coordiate_system(mut self, coordiate_system: CoordinateSystem) -> Self {
        self.coordiate_system = Some(coordiate_system);
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

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
