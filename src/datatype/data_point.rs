use serde::{ser::SerializeStruct, Serialize};

use super::Value;

#[derive(PartialEq, Debug)]
pub enum DataPoint {
    Value(Value),
    Values(Vec<Value>),
    ValueItem {
        value: Value,
        name: Option<String>,
    },
    ValuesItem {
        values: Vec<Value>,
        name: Option<String>,
    },
}

impl Serialize for DataPoint {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DataPoint::Value(v) => v.serialize(serializer),
            DataPoint::Values(v) => v.serialize(serializer),
            DataPoint::ValueItem { value, name } => {
                let mut s = serializer
                    .serialize_struct("DataItem", if let Some(_) = name { 2 } else { 1 })?;
                s.serialize_field("value", value)?;
                if let Some(name) = name {
                    s.serialize_field("name", name)?;
                }
                s.end()
            }
            DataPoint::ValuesItem {
                values: value,
                name,
            } => {
                let mut s = serializer
                    .serialize_struct("DataItem", if let Some(_) = name { 2 } else { 1 })?;
                s.serialize_field("value", value)?;
                if let Some(name) = name {
                    s.serialize_field("name", name)?;
                }
                s.end()
            }
        }
    }
}

impl<V> From<V> for DataPoint
where
    V: Into<Value>,
{
    fn from(v: V) -> Self {
        DataPoint::Value(v.into())
    }
}

impl<V> From<Vec<V>> for DataPoint
where
    V: Into<Value>,
{
    fn from(v: Vec<V>) -> Self {
        DataPoint::Values(v.into_iter().map(Into::into).collect())
    }
}

impl<V> From<(V, &str)> for DataPoint
where
    V: Into<Value>,
{
    fn from(v: (V, &str)) -> Self {
        DataPoint::ValueItem {
            value: v.0.into(),
            name: Some(v.1.to_string()),
        }
    }
}

impl<V> From<(Vec<V>, &str)> for DataPoint
where
    V: Into<Value>,
{
    fn from(v: (Vec<V>, &str)) -> Self {
        DataPoint::ValuesItem {
            values: v.0.into_iter().map(Into::into).collect(),
            name: Some(v.1.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::datatype::value;

    use super::*;

    #[test]
    fn serialize_value() {
        assert_eq!(DataPoint::from(42), DataPoint::Value(42.into()));
        assert_eq!(DataPoint::from(3.14), DataPoint::Value(3.14.into()));
        assert_eq!(DataPoint::from("Monday"), DataPoint::Value("Monday".into()));
    }

    #[test]
    fn serialize_values() {
        assert_eq!(
            DataPoint::from(vec![value(42), value(3.14), value("Monday")]),
            DataPoint::Values(vec![42.into(), 3.14.into(), "Monday".into()])
        );
        assert_eq!(
            DataPoint::from(vec![1, 2, 3]),
            DataPoint::Values(vec![1.into(), 2.into(), 3.into()])
        );
        assert_eq!(
            DataPoint::from(vec![1.414, 1.732, 2.236]),
            DataPoint::Values(vec![1.414.into(), 1.732.into(), 2.236.into()])
        );
        assert_eq!(
            DataPoint::from(vec!["Mon", "Tue", "Wed"]),
            DataPoint::Values(vec!["Mon".into(), "Tue".into(), "Wed".into(),])
        );
    }

    #[test]
    fn serialize_value_item() {
        assert_eq!(
            DataPoint::from((1, "Today")),
            DataPoint::ValueItem {
                value: 1.into(),
                name: Some("Today".to_string())
            }
        );
        assert_eq!(
            DataPoint::from((3.14, "Today")),
            DataPoint::ValueItem {
                value: 3.14.into(),
                name: Some("Today".to_string())
            }
        );
        assert_eq!(
            DataPoint::from(("Monday", "Today")),
            DataPoint::ValueItem {
                value: "Monday".into(),
                name: Some("Today".to_string())
            }
        );
    }

    #[test]
    fn serialize_values_item() {
        assert_eq!(
            DataPoint::from((vec![1, 2, 3], "Today")),
            DataPoint::ValuesItem {
                values: vec![1, 2, 3].into_iter().map(Value::from).collect(),
                name: Some("Today".to_string())
            }
        );
        assert_eq!(
            DataPoint::from((vec![value(42), value(3.14), value("Monday")], "Today")),
            DataPoint::ValuesItem {
                values: vec![value(42), value(3.14), value("Monday")]
                    .into_iter()
                    .map(Value::from)
                    .collect(),
                name: Some("Today".to_string())
            }
        );
    }
}
