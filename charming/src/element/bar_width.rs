use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum BarWidth {
    Number(f64),
    String(String),
}

impl From<f64> for BarWidth {
    fn from(f: f64) -> Self {
        BarWidth::Number(f)
    }
}

impl From<f32> for BarWidth {
    fn from(f: f32) -> Self {
        BarWidth::Number(f as f64)
    }
}

impl From<i32> for BarWidth {
    fn from(f: i32) -> Self {
        BarWidth::Number(f as f64)
    }
}

impl From<i64> for BarWidth {
    fn from(f: i64) -> Self {
        BarWidth::Number(f as f64)
    }
}

impl From<String> for BarWidth {
    fn from(f: String) -> Self {
        BarWidth::String(f)
    }
}
