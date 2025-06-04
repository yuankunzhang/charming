use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Grid3D {}

impl Default for Grid3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid3D {
    pub fn new() -> Self {
        Self {}
    }
}
