use super::JsFunction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum SymbolRotate {
    Number(f64),
    Function(JsFunction),
}

impl From<i64> for SymbolRotate {
    fn from(n: i64) -> Self {
        SymbolRotate::Number(n as f64)
    }
}

impl From<f64> for SymbolRotate {
    fn from(n: f64) -> Self {
        SymbolRotate::Number(n)
    }
}

impl From<JsFunction> for SymbolRotate {
    fn from(f: JsFunction) -> Self {
        SymbolRotate::Function(f)
    }
}
