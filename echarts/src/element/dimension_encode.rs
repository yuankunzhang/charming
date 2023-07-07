use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionEncode {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_name: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tooltip: Vec<String>,
}

impl DimensionEncode {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            item_name: None,
            tooltip: vec![],
        }
    }

    pub fn x<S: Into<String>>(mut self, x: S) -> Self {
        self.x = Some(x.into());
        self
    }

    pub fn y<S: Into<String>>(mut self, y: S) -> Self {
        self.y = Some(y.into());
        self
    }

    pub fn item_name<S: Into<String>>(mut self, item_name: S) -> Self {
        self.item_name = Some(item_name.into());
        self
    }

    pub fn tooltip<S: Into<String>>(mut self, tooltip: Vec<S>) -> Self {
        self.tooltip = tooltip.into_iter().map(|s| s.into()).collect();
        self
    }
}
