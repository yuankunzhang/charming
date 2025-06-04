use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
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
