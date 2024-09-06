use super::RawString;
use serde::Serialize;

#[derive(Serialize)]
#[serde(transparent)]
pub struct FormatterFunction {
    #[cfg(not(target_arch = "wasm32"))]
    pub value: RawString,
    #[cfg(target_arch = "wasm32")]
    pub value: web_sys::js_sys::Function,
}

impl FormatterFunction {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_no_args(body: &str) -> FormatterFunction {
        FormatterFunction {
            value: RawString::from(format!("function() {{ {} }}", body)),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_no_args(body: &str) -> FormatterFunction {
        FormatterFunction {
            value: web_sys::js_sys::Function::new_no_args(body),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_with_args(args: &str, body: &str) -> FormatterFunction {
        FormatterFunction {
            value: RawString::from(format!("function({}) {{ {} }}", args, body)),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_with_args(args: &str, body: &str) -> FormatterFunction {
        FormatterFunction {
            value: web_sys::js_sys::Function::new_with_args(args, body),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]

impl<S> From<S> for FormatterFunction
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        FormatterFunction {
            value: RawString::from(s),
        }
    }
}

#[derive(Serialize)]
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

#[cfg(not(target_arch = "wasm32"))]
impl From<RawString> for Formatter {
    fn from(s: RawString) -> Self {
        Formatter::Function(FormatterFunction { value: s })
    }
}

impl From<FormatterFunction> for Formatter {
    fn from(f: FormatterFunction) -> Self {
        Formatter::Function(f)
    }
}
