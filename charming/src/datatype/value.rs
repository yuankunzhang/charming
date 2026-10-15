use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    OptionalNumber(Option<NumericValue>),
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

impl<N> From<Option<N>> for CompositeValue
where
    N: Into<NumericValue>,
{
    fn from(n: Option<N>) -> Self {
        CompositeValue::OptionalNumber(n.map(Into::into))
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

/// The `val` macro can construct a [CompositeValue]::Array.
/// ```rust
/// use charming::datatype::CompositeValue;
/// use charming::val;
///
/// let data: CompositeValue = val![
///    1,
///    "name1",
///    Some(1)
/// ];
/// ```
#[macro_export]
macro_rules! val {
    ($($x:expr_2021),*) => {
        $crate::datatype::CompositeValue::from(vec![
            $(
                $crate::datatype::CompositeValue::from($x)
            ),*
        ])
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberOrArray {
    Number(NumericValue),
    Array(Vec<NumericValue>),
}

impl<N> From<N> for NumberOrArray
where
    N: Into<NumericValue>,
{
    fn from(n: N) -> Self {
        NumberOrArray::Number(n.into())
    }
}

impl<V> From<Vec<V>> for NumberOrArray
where
    V: Into<NumericValue>,
{
    fn from(v: Vec<V>) -> Self {
        NumberOrArray::Array(v.into_iter().map(|v| v.into()).collect())
    }
}

/// The `num_arr` macro constructs a [NumberOrArray]::Array.
/// ```rust
/// use charming::datatype::NumberOrArray;
/// use charming::num_arr;
///
/// let data: NumberOrArray = num_arr![1, 2, 3, 4];
/// ```
#[macro_export]
macro_rules! num_arr {
    ($($x:expr_2021),*) => {
        $crate::datatype::NumberOrArray::from(vec![
            $(
                $crate::datatype::NumericValue::from($x)
            ),*
        ])
    };
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    Number(NumericValue),
    String(String),
}

impl<N> From<N> for StringOrNumber
where
    N: Into<NumericValue>,
{
    fn from(n: N) -> Self {
        StringOrNumber::Number(n.into())
    }
}

impl From<&str> for StringOrNumber {
    fn from(s: &str) -> Self {
        StringOrNumber::String(s.to_string())
    }
}

impl From<String> for StringOrNumber {
    fn from(s: String) -> Self {
        StringOrNumber::String(s)
    }
}
