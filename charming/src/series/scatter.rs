use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        ColorBy, CoordinateSystem, DimensionEncode, Emphasis, ItemStyle, Label, MarkArea, MarkLine,
        Symbol, SymbolSize,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scatter {
    clip: Option<bool>,
    stack_order: Option<crate::element::StackOrder>,
    single_axis_index: Option<f64>,
    single_axis_id: Option<String>,
    polar_index: Option<f64>,
    polar_id: Option<String>,
    geo_index: Option<f64>,
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
    #[charming_type = "scatter"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    label: Option<Label>,
    dataset_index: Option<f64>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    encode: Option<DimensionEncode>,
    mark_line: Option<MarkLine>,
    mark_area: Option<MarkArea>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
