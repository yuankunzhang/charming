use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Legend {
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<String>>,
}

impl Legend {
    pub fn new() -> Self {
        Self {
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            data: None,
        }
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

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        let data = data.into_iter().map(|s| s.into()).collect();
        self.data = Some(data);
        self
    }
}
