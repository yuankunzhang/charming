use serde::Serialize;

#[derive(Serialize)]
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
