use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Formatter {
    String(String),
    Function(FormatterFunction),
}

impl From<&str> for Formatter {
    fn from(s: &str) -> Self {
        Formatter::String(s.to_string())
    }
}

impl From<FormatterFunction> for Formatter {
    fn from(f: FormatterFunction) -> Self {
        Formatter::Function(f)
    }
}
