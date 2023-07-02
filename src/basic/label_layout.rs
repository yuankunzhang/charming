use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_overlap: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    overlap: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rotate: Option<f64>,
}

impl LabelLayout {
    pub fn new() -> Self {
        Self {
            hide_overlap: None,
            overlap: None,
            rotate: None,
        }
    }

    pub fn hide_overlap(mut self, hide_overlap: bool) -> Self {
        self.hide_overlap = Some(hide_overlap);
        self
    }

    pub fn overlap<S: Into<String>>(mut self, overlap: S) -> Self {
        self.overlap = Some(overlap.into());
        self
    }

    pub fn rotate<F: Into<f64>>(mut self, rotate: F) -> Self {
        self.rotate = Some(rotate.into());
        self
    }
}
