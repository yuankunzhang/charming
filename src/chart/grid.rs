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

    pub fn left<S: Into<String>>(mut self, left: S) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn right<S: Into<String>>(mut self, right: S) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn top<S: Into<String>>(mut self, top: S) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn bottom<S: Into<String>>(mut self, bottom: S) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn contain_label(mut self, contain_label: bool) -> Self {
        self.contain_label = Some(contain_label);
        self
    }
}
