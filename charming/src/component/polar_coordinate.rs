use crate::datatype::CompositeValue;
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// Polar coordinate can be used in scatter and line chart.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PolarCoordinate {
    id: Option<String>,
    /// The `zlevel` value of all graphical elements in.
    zlevel: Option<f64>,
    /// The `z` value of all graphical elements in.
    z: Option<f64>,
    center: Option<CompositeValue>,
    radius: Option<CompositeValue>,
}
