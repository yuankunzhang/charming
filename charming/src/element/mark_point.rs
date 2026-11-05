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
    x: Option<crate::datatype::StringOrNumber>,
    y: Option<crate::datatype::StringOrNumber>,
    coord: Option<crate::datatype::CompositeValue>,
    relative_to: Option<super::MarkerRelativeTo>,
    z2: Option<f64>,
    symbol_offset: Option<crate::datatype::CompositeValue>,
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
    z: Option<f64>,
    z2: Option<f64>,
    symbol_offset: Option<crate::datatype::CompositeValue>,
    emphasis: Option<Box<super::Emphasis>>,
    blur: Option<Box<super::Blur>>,
    select: Option<Box<super::Select>>,
    #[charming_set_vec]
    data: Vec<MarkPointData>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
}
