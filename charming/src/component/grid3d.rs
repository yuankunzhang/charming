use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid3D {}

impl Grid3D {
    pub fn new() -> Self {
        Self {}
    }
}
