use crate::element::{AxisType, NameLocation};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParallelAxis {
    dim: Option<f64>,
    parallel_index: Option<f64>,
    realtime: Option<bool>,
    #[serde(rename = "type")]
    type_: Option<AxisType>,
    name: Option<String>,
    name_location: Option<NameLocation>,
    name_gap: Option<f64>,
    inverse: Option<bool>,
    max: Option<f64>,
    min: Option<f64>,
    start_value: Option<f64>,
    #[charming_set_vec]
    data: Vec<String>,
}
