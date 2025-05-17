use crate::element::js_function::JsFunction;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
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
