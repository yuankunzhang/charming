use crate::datatype::CompositeValue;
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimensionEncode {
    x: Option<CompositeValue>,
    y: Option<CompositeValue>,
    z: Option<CompositeValue>,
    item_name: Option<String>,
    #[charming_set_vec]
    tooltip: Vec<CompositeValue>,
}
