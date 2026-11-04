use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

pub(crate) const FUNCTION_SERDE_NAME: &str = "__charming_js_function";

/// A complete JavaScript function expression, stored without executing it.
///
/// JSON uses `{"type":"function","source":"..."}` on every target.
/// Equality and ordering compare source text, not JavaScript behavior.
/// Only use trusted source; renderers execute it with the chart's privileges.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct JsFunction(String);

impl JsFunction {
    pub fn new_with_args(args: &str, body: &str) -> Self {
        format!("function({args}) {{ {body} }}").into()
    }
}

impl From<&str> for JsFunction {
    fn from(source: &str) -> Self {
        Self(source.to_owned())
    }
}

impl From<String> for JsFunction {
    fn from(source: String) -> Self {
        Self(source)
    }
}

impl Serialize for JsFunction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut value = serializer.serialize_struct(FUNCTION_SERDE_NAME, 2)?;
        value.serialize_field("type", "function")?;
        value.serialize_field("source", &self.0)?;
        value.end()
    }
}

impl<'de> Deserialize<'de> for JsFunction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum Source {
            #[serde(rename = "function")]
            Function { source: String },
        }
        let Source::Function { source } = Source::deserialize(deserializer)?;
        Ok(Self(source))
    }
}
