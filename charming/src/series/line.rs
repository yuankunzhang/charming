use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        smoothness::Smoothness, AreaStyle, CoordinateSystem, DimensionEncode, Emphasis, ItemStyle,
        Label, LineStyle, MarkArea, MarkLine, MarkPoint, Sampling, Step, Symbol, SymbolSize,
        Tooltip,
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
pub struct Line {
    #[serde(rename = "type")]
    #[charming_type = "line"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    coordinate_system: Option<CoordinateSystem>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    show_symbol: Option<bool>,
    stack: Option<String>,
    sampling: Option<Sampling>,
    label: Option<Label>,
    line_style: Option<LineStyle>,
    area_style: Option<AreaStyle>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    smooth: Option<Smoothness>,
    step: Option<Step>,
    connect_nulls: Option<bool>,
    mark_point: Option<MarkPoint>,
    mark_line: Option<MarkLine>,
    mark_area: Option<MarkArea>,
    dataset_id: Option<String>,
    encode: Option<DimensionEncode>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    tooltip: Option<Tooltip>,
    silent: Option<bool>,
    z: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
