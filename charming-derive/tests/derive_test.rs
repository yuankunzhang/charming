use charming_derive::CharmingSetter;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, CharmingSetter)]
#[serde(rename_all = "camelCase")]
struct LineComponent {
    #[serde(rename = "type")]
    #[charming_type = "line"]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    title: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[charming_skip_setter]
    data: Vec<i32>,
}

impl LineComponent {
    pub fn data<I: Into<i32>>(mut self, data: Vec<I>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}

#[test]
fn expand_test() {
    LineComponent::new()
        .name("LineComponent")
        .title("Title 1")
        .title("Title 2")
        .data(vec![1, 2, 3]);
}
