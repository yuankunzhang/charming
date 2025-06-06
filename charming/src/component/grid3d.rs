use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Grid3D {}
