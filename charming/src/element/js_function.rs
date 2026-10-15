use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use crate::element::RawString;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct JsFunction {
    #[cfg(not(target_arch = "wasm32"))]
    value: super::RawString,
    #[cfg(target_arch = "wasm32")]
    #[serde(with = "serde_wasm_bindgen::preserve")]
    value: js_sys::Function,
}

impl JsFunction {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_with_args(args: &str, body: &str) -> JsFunction {
        JsFunction {
            value: super::RawString::from(format!("function({args}) {{ {body} }}")),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_with_args(args: &str, body: &str) -> JsFunction {
        JsFunction {
            value: js_sys::Function::new_with_args(args, body),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<&str> for JsFunction {
    fn from(s: &str) -> Self {
        JsFunction {
            value: RawString::from(s),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<String> for JsFunction {
    fn from(s: String) -> Self {
        JsFunction {
            value: RawString::from(s),
        }
    }
}

impl PartialOrd for JsFunction {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}
