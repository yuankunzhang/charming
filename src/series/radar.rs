use serde::Serialize;

use crate::utility::{color::ColorBy, symbol::Symbol};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarData {
    pub name: String,
    pub value: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Radar {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

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

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<RadarData>,
}

impl Radar {
    pub fn new() -> Self {
        Self {
            type_: "radar".into(),
            name: None,
            color_by: None,
            symbol: None,
            symbol_size: None,
            symbol_rotate: None,
            symbol_keep_aspect: None,
            data: vec![],
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
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

    pub fn data(mut self, data: Vec<RadarData>) -> Self {
        self.data = data;
        self
    }
}
