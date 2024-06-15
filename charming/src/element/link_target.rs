use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkTarget {
    #[serde(rename = "self")]
    Self_,
    Blank,
}
