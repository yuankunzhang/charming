use crate::element::js_function::JsFunction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(transparent)]
pub struct FormatterFunction {
    #[cfg(not(target_arch = "wasm32"))]
    value: super::RawString,
    #[cfg(target_arch = "wasm32")]
    #[serde(with = "serde_wasm_bindgen::preserve")]
    value: js_sys::Function,
}

impl FormatterFunction {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_with_args(args: &str, body: &str) -> FormatterFunction {
        FormatterFunction {
            value: super::RawString::from(format!("function({}) {{ {} }}", args, body)),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_with_args(args: &str, body: &str) -> FormatterFunction {
        FormatterFunction {
            value: js_sys::Function::new_with_args(args, body),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum Formatter {
    String(String),
    Function(JsFunction),
}

impl From<&str> for Formatter {
    fn from(s: &str) -> Self {
        Formatter::String(s.to_string())
    }
}

impl From<JsFunction> for Formatter {
    fn from(f: JsFunction) -> Self {
        Formatter::Function(f)
    }
}
