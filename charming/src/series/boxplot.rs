use crate::{
    datatype::{DataFrame, DataPoint},
    element::{ColorBy, CoordinateSystem, ItemStyle, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Boxplot {
    #[serde(rename = "type")]
    #[charming_type = "boxplot"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    coordinate_system: Option<CoordinateSystem>,
    color_by: Option<ColorBy>,
    legend_hover_link: Option<bool>,
    hover_animation: Option<bool>,
    dataset_index: Option<f64>,
    tooltip: Option<Tooltip>,
    item_style: Option<ItemStyle>,
    z: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
