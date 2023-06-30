use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contain_label: Option<bool>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            left: None,
            top: None,
            right: None,
            bottom: None,
            contain_label: None,
        }
    }

    pub fn left(mut self, left: &str) -> Self {
        self.left = Some(left.to_string());
        self
    }

    pub fn right(mut self, right: &str) -> Self {
        self.right = Some(right.to_string());
        self
    }

    pub fn top(mut self, top: &str) -> Self {
        self.top = Some(top.to_string());
        self
    }

    pub fn bottom(mut self, bottom: &str) -> Self {
        self.bottom = Some(bottom.to_string());
        self
    }

    pub fn contain_label(mut self, contain_label: bool) -> Self {
        self.contain_label = Some(contain_label);
        self
    }
}
