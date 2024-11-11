use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Cursor {
    Pointer,
}
