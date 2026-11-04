use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

use crate::element::{ItemStyle, Label, Symbol, SymbolSize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum MarkPointDataType {
    Min,
    Max,
    Average,
}

impl From<&str> for MarkPointDataType {
    fn from(s: &str) -> Self {
        match s {
            "min" => Self::Min,
            "max" => Self::Max,
            "avg" | "average" => Self::Average,
            _ => panic!("Invalid MarkPointDataType"),
        }
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPointData {
    #[serde(rename = "type")]
    type_: Option<MarkPointDataType>,
    name: Option<String>,
    x_axis: Option<f64>,
    y_axis: Option<f64>,
    value: Option<f64>,
}

impl From<(&str, &str)> for MarkPointData {
    fn from((type_, name): (&str, &str)) -> Self {
        Self::new().type_(type_).name(name)
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPoint {
    #[charming_set_vec]
    data: Vec<MarkPointData>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
}
