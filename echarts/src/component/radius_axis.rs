use serde::Serialize;

use crate::element::{AxisLabel, AxisLine, AxisType};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadiusAxis {
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label: Option<AxisLabel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_line: Option<AxisLine>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<AxisType>,
}

impl RadiusAxis {
    pub fn new() -> Self {
        Self {
            axis_label: None,
            axis_line: None,
            data: vec![],
            type_: None,
        }
    }

    pub fn axis_label(mut self, axis_label: AxisLabel) -> Self {
        self.axis_label = Some(axis_label);
        self
    }

    pub fn axis_line(mut self, axis_line: AxisLine) -> Self {
        self.axis_line = Some(axis_line);
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn type_<T: Into<AxisType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }
}
