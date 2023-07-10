use serde::Serialize;

use super::RawString;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ContentFormatter {
    String(String),
    Function(RawString),
}

impl From<&str> for ContentFormatter {
    fn from(s: &str) -> Self {
        ContentFormatter::String(s.to_string())
    }
}

impl From<RawString> for ContentFormatter {
    fn from(s: RawString) -> Self {
        ContentFormatter::Function(s)
    }
}
