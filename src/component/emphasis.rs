use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Focus {
    None,
    #[serde(rename = "self")]
    Self_,
    Series,
    Ancestor,
    Descendant,
    Relative,
    Adjacency,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Emphasis {
    #[serde(skip_serializing_if = "Option::is_none")]
    focus: Option<Focus>,
}

impl Emphasis {
    pub fn new() -> Self {
        Self { focus: None }
    }

    pub fn focus(mut self, emphasis: Focus) -> Self {
        self.focus = Some(emphasis);
        self
    }
}
