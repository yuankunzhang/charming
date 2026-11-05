use crate::element::js_function::JsFunction;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum Formatter {
    String(String),
    Function(JsFunction),
    TimeDictionary(BTreeMap<TimeUnit, TimeFormatterEntry>),
}

impl From<String> for Formatter {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<BTreeMap<TimeUnit, TimeFormatterEntry>> for Formatter {
    fn from(value: BTreeMap<TimeUnit, TimeFormatterEntry>) -> Self {
        Self::TimeDictionary(value)
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum TimeUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum TimeFormatterValue {
    String(String),
    Levels(Vec<String>),
}

impl From<&str> for TimeFormatterValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<Vec<String>> for TimeFormatterValue {
    fn from(value: Vec<String>) -> Self {
        Self::Levels(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum TimeFormatterEntry {
    Value(TimeFormatterValue),
    Upper(BTreeMap<TimeUnit, TimeFormatterValue>),
}

impl From<&str> for TimeFormatterEntry {
    fn from(value: &str) -> Self {
        Self::Value(value.into())
    }
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

// These values are object keys, including in the typed source-conversion path.
impl Serialize for TimeUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Year => "year",
            Self::Month => "month",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Minute => "minute",
            Self::Second => "second",
            Self::Millisecond => "millisecond",
        })
    }
}
