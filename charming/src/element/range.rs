use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum Range {
    Year(i64),
    Single(String),
    Range((String, String)),
}

impl From<i64> for Range {
    fn from(value: i64) -> Self {
        Self::Year(value)
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<String> for Range {
    fn from(value: String) -> Self {
        Self::Single(value)
    }
}

impl From<(&str, &str)> for Range {
    fn from(value: (&str, &str)) -> Self {
        Self::Range((value.0.to_string(), value.1.to_string()))
    }
}

impl From<(String, String)> for Range {
    fn from(value: (String, String)) -> Self {
        Self::Range((value.0, value.1))
    }
}
