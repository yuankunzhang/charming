#![allow(dead_code)]

use crate::element::{
    ColorBy, CoordinateSystem, Emphasis, Label, LabelLayout, LineStyle, Symbol, Tooltip,
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lines {
    #[serde(rename = "type")]
    #[charming_type = "lines"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    geo_index: Option<f64>,
    polyline: Option<bool>,
    large: Option<bool>,
    large_threshold: Option<f64>,
    symbol: Option<Symbol>,
    symbol_size: Option<f64>,
    line_style: Option<LineStyle>,
    label: Option<Label>,
    label_layout: Option<LabelLayout>,
    emphasis: Option<Emphasis>,
    tooltip: Option<Tooltip>,
}
