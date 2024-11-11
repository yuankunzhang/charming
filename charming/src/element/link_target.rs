use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LinkTarget {
    #[serde(rename = "self")]
    Self_,
    Blank,
}
