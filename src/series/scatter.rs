use serde::Serialize;

use crate::utility::color::ColorBy;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scatter {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_index: Option<u64>,
}

impl Scatter {
    pub fn new() -> Self {
        Self {
            type_: String::from("scatter"),
            name: None,
            color_by: None,
            dataset_index: None,
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn dataset_index(mut self, dataset_index: u64) -> Self {
        self.dataset_index = Some(dataset_index);
        self
    }
}
