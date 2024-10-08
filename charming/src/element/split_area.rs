use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SplitArea {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,
}

impl Default for SplitArea {
    fn default() -> Self {
        Self::new()
    }
}

impl SplitArea {
    pub fn new() -> Self {
        Self { show: None }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }
}
