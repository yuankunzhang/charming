use serde::Serialize;

use crate::element::AxisType;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadiusAxis {
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<AxisType>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<String>,
}

impl RadiusAxis {
    pub fn new() -> Self {
        Self {
            type_: None,
            data: vec![],
        }
    }

    pub fn type_<T: Into<AxisType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }
}
