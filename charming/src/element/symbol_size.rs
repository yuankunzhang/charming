use super::JsFunction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum SymbolSize {
    Number(f64),
    Function(JsFunction),
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

impl From<JsFunction> for SymbolSize {
    fn from(f: JsFunction) -> Self {
        SymbolSize::Function(f)
    }
}
