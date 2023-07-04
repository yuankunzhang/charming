use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Number::Integer(n)
    }
}

impl From<f64> for Number {
    fn from(n: f64) -> Self {
        Number::Float(n)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Number(Number),
    String(String),
    Array(Vec<Value>),
}

pub fn value<T: Into<Value>>(v: T) -> Value {
    v.into()
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Number(_) => None,
            Value::String(s) => Some(s),
            Value::Array(_) => None,
        }
    }

    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Number(_) => None,
            Value::String(_) => None,
            Value::Array(v) => Some(v),
        }
    }
}

impl Serialize for Value {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Number(n) => match n {
                Number::Integer(n) => serializer.serialize_i64(*n),
                Number::Float(n) => serializer.serialize_f64(*n),
            },
            Value::String(s) => serializer.serialize_str(s),
            Value::Array(v) => serializer.collect_seq(v.iter()),
        }
    }
}

impl<'a> Deserialize<'a> for Value {
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(deserializer)?;
        match v {
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    Ok(Value::Number(n.as_i64().unwrap().into()))
                } else {
                    Ok(Value::Number(n.as_f64().unwrap().into()))
                }
            }
            serde_json::Value::String(s) => Ok(Value::String(s)),
            serde_json::Value::Array(v) => Ok(Value::Array(
                v.into_iter()
                    .map(|v| serde_json::from_value(v).unwrap())
                    .collect(),
            )),
            _ => Err(serde::de::Error::custom("invalid value")),
        }
    }
}

impl<N> From<N> for Value
where
    N: Into<Number>,
{
    fn from(n: N) -> Self {
        Value::Number(n.into())
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

impl<V> From<Vec<V>> for Value
where
    V: Into<Value>,
{
    fn from(v: Vec<V>) -> Self {
        Value::Array(v.into_iter().map(|v| v.into()).collect())
    }
}

impl<V> FromIterator<V> for Value
where
    V: Into<Value>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Value::Array(iter.into_iter().map(|v| v.into()).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_number() {
        assert_eq!(Value::from(42), Value::Number(42.into()));
        assert_eq!(Value::from(3.14), Value::Number(3.14.into()));
        assert_eq!(Value::from(-1.618), Value::Number((-1.618).into()));
    }

    #[test]
    fn from_string() {
        assert_eq!(Value::from("Monday"), Value::String("Monday".to_string()));
        assert_eq!(
            Value::from("Monday".to_string()),
            Value::String("Monday".to_string())
        );
    }

    #[test]
    fn from_vec() {
        assert_eq!(
            Value::from(vec![1.414, 1.732, 2.236]),
            Value::Array(vec![
                Value::from(1.414),
                Value::from(1.732),
                Value::from(2.236)
            ])
        );
        assert_eq!(
            Value::from(vec![value(42), value(3.14), value("Monday")]),
            Value::Array(vec![
                Value::from(42),
                Value::from(3.14),
                Value::from("Monday")
            ])
        );
    }
}
