use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum Offset {
    Number(f64),
    String(String),
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct SymbolOffset(Offset, Offset);

impl From<f64> for Offset {
    fn from(f: f64) -> Self {
        Offset::Number(f)
    }
}

impl From<f32> for Offset {
    fn from(f: f32) -> Self {
        Offset::Number(f as f64)
    }
}

impl From<i32> for Offset {
    fn from(f: i32) -> Self {
        Offset::Number(f as f64)
    }
}

impl From<i64> for Offset {
    fn from(f: i64) -> Self {
        Offset::Number(f as f64)
    }
}

impl From<String> for Offset {
    fn from(f: String) -> Self {
        Offset::String(f)
    }
}
