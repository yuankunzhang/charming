use super::color::Color;
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LineStyleType {
    Solid,
    Dashed,
    Dotted,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    color: Option<Color>,
    width: Option<f64>,
    #[serde(rename = "type")]
    type_: Option<LineStyleType>,
    opacity: Option<f64>,
    curveness: Option<f64>,
}
