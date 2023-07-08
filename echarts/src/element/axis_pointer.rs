use serde::Serialize;

use super::Label;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisPointerType {
    Line,
    Shadow,
    Cross,
    None,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisPointerAxis {
    X,
    Y,
    Radius,
    Angle,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointer {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<AxisPointerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis: Option<AxisPointerAxis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    snap: Option<bool>,
}

impl AxisPointer {
    pub fn new() -> Self {
        Self {
            type_: None,
            axis: None,
            label: None,
            snap: None,
        }
    }

    pub fn type_<A: Into<AxisPointerType>>(mut self, type_: A) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn axis<A: Into<AxisPointerAxis>>(mut self, axis: A) -> Self {
        self.axis = Some(axis.into());
        self
    }

    pub fn label<A: Into<Label>>(mut self, label: A) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn snap(mut self, snap: bool) -> Self {
        self.snap = Some(snap);
        self
    }
}
