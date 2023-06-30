use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subtext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<String>,
}

impl Title {
    pub fn new() -> Self {
        Self {
            text: None,
            subtext: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
        }
    }

    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn subtext<S: Into<String>>(mut self, subtext: S) -> Self {
        self.subtext = Some(subtext.into());
        self
    }

    pub fn left<S: Into<String>>(mut self, left: S) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<S: Into<String>>(mut self, top: S) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<S: Into<String>>(mut self, right: S) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<S: Into<String>>(mut self, bottom: S) -> Self {
        self.bottom = Some(bottom.into());
        self
    }
}
