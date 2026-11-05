use crate::datatype::CompositeValue;
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
pub struct LinesData {
    name: Option<String>,
    #[charming_set_vec]
    coords: Vec<Vec<f64>>,
    value: Option<CompositeValue>,
    line_style: Option<LineStyle>,
    label: Option<Label>,
    emphasis: Option<Emphasis>,
}

impl From<Vec<Vec<f64>>> for LinesData {
    fn from(coords: Vec<Vec<f64>>) -> Self {
        Self::new().coords(coords)
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lines {
    clip: Option<bool>,
    polar_index: Option<f64>,
    polar_id: Option<String>,
    geo_id: Option<String>,
    x_axis_id: Option<String>,
    y_axis_id: Option<String>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
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
    #[charming_set_vec]
    data: Vec<LinesData>,
}
