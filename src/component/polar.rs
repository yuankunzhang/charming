use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Polar {
    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<(String, String)>,
}

impl Polar {
    pub fn new() -> Self {
        Self { radius: None }
    }

    pub fn radius<S: Into<String>>(mut self, radius: (S, S)) -> Self {
        self.radius = Some((radius.0.into(), radius.1.into()));
        self
    }
}
