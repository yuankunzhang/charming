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

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn subtext(mut self, subtext: &str) -> Self {
        self.subtext = Some(subtext.to_string());
        self
    }

    pub fn left(mut self, left: &str) -> Self {
        self.left = Some(left.to_string());
        self
    }

    pub fn top(mut self, top: &str) -> Self {
        self.top = Some(top.to_string());
        self
    }

    pub fn right(mut self, right: &str) -> Self {
        self.right = Some(right.to_string());
        self
    }

    pub fn bottom(mut self, bottom: &str) -> Self {
        self.bottom = Some(bottom.to_string());
        self
    }
}
