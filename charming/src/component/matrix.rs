use crate::{
    datatype::{CompositeValue, StringOrNumber},
    element::{Cursor, ItemStyle, Label, LineStyle, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatrixDimensionLevel {
    level_size: Option<StringOrNumber>,
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatrixDimensionCell {
    value: Option<String>,
    size: Option<StringOrNumber>,
    #[charming_set_vec]
    children: Vec<MatrixDimensionCell>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    cursor: Option<Cursor>,
    silent: Option<bool>,
    z2: Option<f64>,
}

impl From<&str> for MatrixDimensionCell {
    fn from(value: &str) -> Self {
        Self::new().value(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MatrixDimensionData {
    Cell(MatrixDimensionCell),
    Value(Option<String>),
}

impl From<MatrixDimensionCell> for MatrixDimensionData {
    fn from(value: MatrixDimensionCell) -> Self {
        Self::Cell(value)
    }
}

impl From<&str> for MatrixDimensionData {
    fn from(value: &str) -> Self {
        Self::Value(Some(value.to_owned()))
    }
}

impl From<String> for MatrixDimensionData {
    fn from(value: String) -> Self {
        Self::Value(Some(value))
    }
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatrixDimension {
    show: Option<bool>,
    #[charming_set_vec]
    data: Vec<MatrixDimensionData>,
    length: Option<usize>,
    #[charming_set_vec]
    levels: Vec<Option<MatrixDimensionLevel>>,
    level_size: Option<StringOrNumber>,
    divider_line_style: Option<LineStyle>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    cursor: Option<Cursor>,
    silent: Option<bool>,
    z2: Option<f64>,
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatrixCell {
    value: Option<String>,
    coord: Option<CompositeValue>,
    coord_clamp: Option<bool>,
    merge_cells: Option<bool>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    cursor: Option<Cursor>,
    silent: Option<bool>,
    z2: Option<f64>,
}

/// Shared styles and optional cell overrides for a matrix body or corner.
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatrixCells {
    #[charming_set_vec]
    data: Vec<MatrixCell>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    cursor: Option<Cursor>,
    silent: Option<bool>,
    z2: Option<f64>,
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Matrix {
    id: Option<String>,
    z: Option<f64>,
    zlevel: Option<f64>,
    left: Option<CompositeValue>,
    right: Option<CompositeValue>,
    top: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    x: Option<MatrixDimension>,
    y: Option<MatrixDimension>,
    body: Option<MatrixCells>,
    corner: Option<MatrixCells>,
    background_style: Option<ItemStyle>,
    border_z2: Option<f64>,
    tooltip: Option<Tooltip>,
    trigger_event: Option<bool>,
}
