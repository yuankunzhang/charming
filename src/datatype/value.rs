use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Serialize for Value {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Number(n) => serializer.serialize_f64(*n),
            Value::String(s) => serializer.serialize_str(s),
        }
    }
}

impl<'a> Deserialize<'a> for Value {
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(deserializer)?;
        match v {
            serde_json::Value::Number(n) => Ok(Value::Number(n.as_f64().unwrap())),
            serde_json::Value::String(s) => Ok(Value::String(s)),
            _ => Err(serde::de::Error::custom("invalid value")),
        }
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Number(n as f64)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

pub fn value<T: Into<Value>>(v: T) -> Value {
    v.into()
}
