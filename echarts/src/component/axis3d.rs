use serde::Serialize;

use crate::element::AxisType;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Axis3D {
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<AxisType>,
}

impl Axis3D {
    pub fn new() -> Self {
        Self { type_: None }
    }

    pub fn type_<A: Into<AxisType>>(mut self, type_: A) -> Self {
        self.type_ = Some(type_.into());
        self
    }
}
