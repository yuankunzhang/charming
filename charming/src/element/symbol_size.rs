use serde::Serialize;

use super::{FormatterFunction, RawString};

#[derive(Serialize)]
#[serde(untagged)]
pub enum SymbolSize {
    Number(f64),
    Function(FormatterFunction),
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

#[cfg(not(target_arch = "wasm32"))]
impl From<&str> for SymbolSize {
    fn from(s: &str) -> Self {
        SymbolSize::Function(FormatterFunction {
            value: RawString::from(s),
        })
    }
}

impl From<FormatterFunction> for SymbolSize {
    fn from(f: FormatterFunction) -> Self {
        SymbolSize::Function(f)
    }
}
