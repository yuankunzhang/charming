use serde::Serialize;

use crate::{
    basic::{area_style::AreaStyle, color::ColorBy, symbol::Symbol},
    component::tooltip::Tooltip,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPoint {
    name: String,
    value: Vec<f64>,
}

impl DataPoint {
    pub fn new<S: Into<String>, F: Into<f64>>(name: S, value: Vec<F>) -> Self {
        Self {
            name: name.into(),
            value: value.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl From<(&str, Vec<f64>)> for DataPoint {
    fn from((name, value): (&str, Vec<f64>)) -> Self {
        Self::new(name, value)
    }
}

impl From<(&str, Vec<i64>)> for DataPoint {
    fn from((name, value): (&str, Vec<i64>)) -> Self {
        Self::new(name, value.into_iter().map(|v| v as f64).collect())
    }
}

pub type Data = Vec<DataPoint>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Radar {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radar_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_rotate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_keep_aspect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip: Option<Tooltip>,

    #[serde(skip_serializing_if = "Option::is_none")]
    area_style: Option<AreaStyle>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Data,
}

impl Radar {
    pub fn new() -> Self {
        Self {
            type_: "radar".into(),
            name: None,
            radar_index: None,
            color_by: None,
            symbol: None,
            symbol_size: None,
            symbol_rotate: None,
            symbol_keep_aspect: None,
            tooltip: None,
            area_style: None,
            data: vec![],
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn radar_index<F: Into<f64>>(mut self, radar_index: F) -> Self {
        self.radar_index = Some(radar_index.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
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

    pub fn symbol_rotate(mut self, symbol_rotate: f64) -> Self {
        self.symbol_rotate = Some(symbol_rotate);
        self
    }

    pub fn symbol_keep_aspect(mut self, symbol_keep_aspect: bool) -> Self {
        self.symbol_keep_aspect = Some(symbol_keep_aspect);
        self
    }

    pub fn tooltip(mut self, tooltip: Tooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn area_style(mut self, area_style: AreaStyle) -> Self {
        self.area_style = Some(area_style);
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
