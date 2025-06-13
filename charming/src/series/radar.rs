use crate::{
    datatype::{DataFrame, DataPoint},
    element::{AreaStyle, ColorBy, Emphasis, LineStyle, Symbol, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Radar {
    #[serde(rename = "type")]
    #[charming_type = "radar"]
    type_: String,
    area_style: Option<AreaStyle>,
    color_by: Option<ColorBy>,
    id: Option<String>,
    name: Option<String>,
    radar_index: Option<f64>,
    symbol: Option<Symbol>,
    symbol_keep_aspect: Option<bool>,
    symbol_rotate: Option<f64>,
    symbol_size: Option<f64>,
    tooltip: Option<Tooltip>,
    line_style: Option<LineStyle>,
    emphasis: Option<Emphasis>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
