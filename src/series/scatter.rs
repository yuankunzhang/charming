use serde::Serialize;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{color::ColorBy, coordinate::CoordinateSystem, symbol::Symbol},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scatter {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl Scatter {
    pub fn new() -> Self {
        Self {
            type_: String::from("scatter"),
            name: None,
            color_by: None,
            dataset_index: None,
            coordinate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            symbol: None,
            symbol_size: None,
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

    pub fn dataset_index<F: Into<f64>>(mut self, dataset_index: F) -> Self {
        self.dataset_index = Some(dataset_index.into());
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

    pub fn symbol(mut self, symbol: Symbol) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbol_size<F: Into<f64>>(mut self, symbol_size: F) -> Self {
        self.symbol_size = Some(symbol_size.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
