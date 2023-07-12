use serde::Serialize;

use crate::element::{ColorBy, CoordinateSystem, Emphasis, Label, LabelLayout, LineStyle, Symbol};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Lines {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    geo_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    polyline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    large: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    large_threshold: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label_layout: Option<LabelLayout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,
}
