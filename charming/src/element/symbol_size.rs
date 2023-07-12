use serde::Serialize;

use super::RawString;

#[derive(Serialize)]
#[serde(untagged)]
pub enum SymbolSize {
    Number(f64),
    Function(RawString),
}

impl From<i64> for SymbolSize {
    fn from(n: i64) -> Self {
        SymbolSize::Number(n as f64)
    }
}

impl From<f64> for SymbolSize {
    fn from(n: f64) -> Self {
        SymbolSize::Number(n)
    }
}

impl From<&str> for SymbolSize {
    fn from(s: &str) -> Self {
        SymbolSize::Function(RawString::from(s))
    }
}
