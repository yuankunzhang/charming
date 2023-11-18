use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumericValue {
    Integer(i64),
    Float(f64),
}

impl From<i32> for NumericValue {
    fn from(n: i32) -> Self {
        NumericValue::Integer(n as i64)
    }
}

impl From<i64> for NumericValue {
    fn from(n: i64) -> Self {
        NumericValue::Integer(n)
    }
}

impl From<f32> for NumericValue {
    fn from(n: f32) -> Self {
        NumericValue::Float(n as f64)
    }
}

impl From<f64> for NumericValue {
    fn from(n: f64) -> Self {
        NumericValue::Float(n)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompositeValue {
    Number(NumericValue),
    String(String),
    Array(Vec<CompositeValue>),
}

impl<N> From<N> for CompositeValue
where
    N: Into<NumericValue>,
{
    fn from(n: N) -> Self {
        CompositeValue::Number(n.into())
    }
}

impl From<&str> for CompositeValue {
    fn from(s: &str) -> Self {
        CompositeValue::String(s.to_string())
    }
}

impl From<String> for CompositeValue {
    fn from(s: String) -> Self {
        CompositeValue::String(s)
    }
}

impl<V> From<Vec<V>> for CompositeValue
where
    V: Into<CompositeValue>,
{
    fn from(v: Vec<V>) -> Self {
        CompositeValue::Array(v.into_iter().map(|v| v.into()).collect())
    }
}

impl<V> FromIterator<V> for CompositeValue
where
    V: Into<CompositeValue>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        CompositeValue::Array(iter.into_iter().map(|v| v.into()).collect())
    }
}

#[macro_export]
macro_rules! val {
    ($($x:expr),*) => {
        $crate::datatype::CompositeValue::from(vec![
            $(
                $crate::datatype::CompositeValue::from($x)
            ),*
        ])
    };
}
