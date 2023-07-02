use serde::{Deserialize, Serialize};

pub mod anchor;
pub mod area_style;
pub mod axis_style;
pub mod background_style;
pub mod border_type;
pub mod boundary_gap;
pub mod color;
pub mod coordinate;
pub mod emphasis;
pub mod icon;
pub mod item_style;
pub mod label;
pub mod label_layout;
pub mod line_style;
pub mod link_target;
pub mod orient;
pub mod padding;
pub mod pointer;
pub mod scale_limit;
pub mod single_axis;
pub mod sort;
pub mod split_area;
pub mod split_line;
pub mod symbol;
pub mod text_align;
pub mod text_style;

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

#[derive(Serialize, Deserialize)]
pub struct DataPoint {
    pub value: Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<Value> for DataPoint {
    fn from(value: Value) -> Self {
        Self {
            value: vec![value],
            name: None,
        }
    }
}

impl From<f64> for DataPoint {
    fn from(value: f64) -> Self {
        Self {
            value: vec![value.into()],
            name: None,
        }
    }
}

impl From<i64> for DataPoint {
    fn from(value: i64) -> Self {
        Self {
            value: vec![value.into()],
            name: None,
        }
    }
}

impl From<Vec<Value>> for DataPoint {
    fn from(value: Vec<Value>) -> Self {
        Self { value, name: None }
    }
}

impl From<Vec<f64>> for DataPoint {
    fn from(value: Vec<f64>) -> Self {
        Self {
            value: value.into_iter().map(|f| f.into()).collect(),
            name: None,
        }
    }
}

impl From<Vec<i64>> for DataPoint {
    fn from(value: Vec<i64>) -> Self {
        Self {
            value: value.into_iter().map(|n| n.into()).collect(),
            name: None,
        }
    }
}

impl From<(&str, Vec<f64>)> for DataPoint {
    fn from((name, value): (&str, Vec<f64>)) -> Self {
        Self {
            value: value.into_iter().map(|f| f.into()).collect(),
            name: Some(name.to_string()),
        }
    }
}

impl From<(&str, Vec<i64>)> for DataPoint {
    fn from((name, value): (&str, Vec<i64>)) -> Self {
        Self {
            value: value.into_iter().map(|n| n.into()).collect(),
            name: Some(name.to_string()),
        }
    }
}

pub type DataFrame = Vec<DataPoint>;
