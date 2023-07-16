use serde::Serialize;

use crate::datatype::CompositeValue;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionEncode {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_name: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tooltip: Vec<CompositeValue>,
}

impl DimensionEncode {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            item_name: None,
            tooltip: vec![],
        }
    }

    pub fn x<C: Into<CompositeValue>>(mut self, x: C) -> Self {
        self.x = Some(x.into());
        self
    }

    pub fn y<C: Into<CompositeValue>>(mut self, y: C) -> Self {
        self.y = Some(y.into());
        self
    }

    pub fn z<C: Into<CompositeValue>>(mut self, z: C) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn item_name<S: Into<String>>(mut self, item_name: S) -> Self {
        self.item_name = Some(item_name.into());
        self
    }

    pub fn tooltip<S: Into<CompositeValue>>(mut self, tooltip: Vec<S>) -> Self {
        self.tooltip = tooltip.into_iter().map(|s| s.into()).collect();
        self
    }
}
