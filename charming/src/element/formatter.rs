use super::RawString;
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Formatter {
    String(String),

    #[cfg(target_arch = "wasm32")]
    Function(#[serde(with = "serde_wasm_bindgen::preserve")] web_sys::js_sys::Function),

    #[cfg(not(target_arch = "wasm32"))]
    Function(RawString),
}

impl From<&str> for Formatter {
    fn from(s: &str) -> Self {
        Formatter::String(s.to_string())
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<RawString> for Formatter {
    fn from(s: RawString) -> Self {
        Formatter::Function(s)
    }
}
