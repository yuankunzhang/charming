use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{CoordinateSystem, DimensionEncode},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
pub struct Bar3d {
    #[serde(rename = "type")]
    #[charming_type = "bar3D"]
    type_: String,
    name: Option<String>,
    coordinate_system: Option<CoordinateSystem>,
    grid3d_index: Option<CompositeValue>,
    geo3d_index: Option<CompositeValue>,
    globe_index: Option<CompositeValue>,
    shading: Option<String>,
    encode: Option<DimensionEncode>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
