use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum CellSize {
    Single(InnerCellType),
    Double((InnerCellType, InnerCellType)),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum InnerCellType {
    Num(f64),
    String(String),
}

impl From<&str> for InnerCellType {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for InnerCellType {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i32> for InnerCellType {
    fn from(value: i32) -> Self {
        Self::Num(value.into())
    }
}

impl From<f64> for InnerCellType {
    fn from(value: f64) -> Self {
        Self::Num(value)
    }
}

impl<V> From<V> for CellSize
where
    V: Into<InnerCellType>,
{
    fn from(value: V) -> Self {
        Self::Single(value.into())
    }
}

impl<V, W> From<(V, W)> for CellSize
where
    V: Into<InnerCellType>,
    W: Into<InnerCellType>,
{
    fn from(value: (V, W)) -> Self {
        Self::Double((value.0.into(), value.1.into()))
    }
}
