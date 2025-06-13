use crate::{
    datatype::{DataFrame, DataPoint},
    element::{ColorBy, CoordinateSystem, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    #[serde(rename = "type")]
    #[charming_type = "candlestick"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    coordiate_system: Option<CoordinateSystem>,
    color_by: Option<ColorBy>,
    legend_hover_link: Option<bool>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
