use serde::{ser::SerializeStruct, Deserialize, Serialize};

use super::Value;

#[derive(PartialEq, Debug)]
pub enum DataPoint {
    Value(Value),
    Item { value: Value, name: Option<String> },
}

impl Serialize for DataPoint {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DataPoint::Value(v) => v.serialize(serializer),
            DataPoint::Item { value, name } => {
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

impl<'de> Deserialize<'de> for DataPoint {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(deserializer)?;
        match v {
            serde_json::Value::Number(n) => Ok(DataPoint::Value(n.as_f64().unwrap().into())),
            serde_json::Value::String(s) => Ok(DataPoint::Value(s.into())),
            serde_json::Value::Array(v) => Ok(DataPoint::Value(
                v.into_iter()
                    .map(|v| serde_json::from_value::<Value>(v).unwrap())
                    .collect(),
            )),
            serde_json::Value::Object(o) => {
                let name = o.get("name");
                let value = o.get("value").unwrap();

                if let Some(name) = name {
                    Ok(DataPoint::Item {
                        value: serde_json::from_value(value.clone()).unwrap(),
                        name: Some(name.as_str().unwrap().to_string()),
                    })
                } else {
                    Ok(DataPoint::Value(
                        serde_json::from_value(value.clone()).unwrap(),
                    ))
                }
            }
            _ => unreachable!(),
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

impl<V> From<(V, &str)> for DataPoint
where
    V: Into<Value>,
{
    fn from(v: (V, &str)) -> Self {
        DataPoint::Item {
            value: v.0.into(),
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

        assert_eq!(
            DataPoint::from(vec![1, 2, 3]),
            DataPoint::Value(vec![1, 2, 3].into())
        );
        assert_eq!(
            DataPoint::from(vec![1.414, 1.732, 2.236]),
            DataPoint::Value(vec![1.414, 1.732, 2.236].into())
        );
        assert_eq!(
            DataPoint::from(vec!["Mon", "Tue", "Wed"]),
            DataPoint::Value(vec!["Mon", "Tue", "Wed"].into())
        );
        assert_eq!(
            DataPoint::from(vec![value(42), value(3.14), value("Monday")]),
            DataPoint::Value(vec![value(42), value(3.14), value("Monday")].into())
        );
    }

    #[test]
    fn serialize_value_item() {
        assert_eq!(
            DataPoint::from((1, "Today")),
            DataPoint::Item {
                value: 1.into(),
                name: Some("Today".to_string())
            }
        );
        assert_eq!(
            DataPoint::from((3.14, "Today")),
            DataPoint::Item {
                value: 3.14.into(),
                name: Some("Today".to_string())
            }
        );
        assert_eq!(
            DataPoint::from(("Monday", "Today")),
            DataPoint::Item {
                value: "Monday".into(),
                name: Some("Today".to_string())
            }
        );

        assert_eq!(
            DataPoint::from((vec![1, 2, 3], "Today")),
            DataPoint::Item {
                value: vec![1, 2, 3].into(),
                name: Some("Today".to_string())
            }
        );
        assert_eq!(
            DataPoint::from((vec![1.414, 1.732, 2.236], "Today")),
            DataPoint::Item {
                value: vec![1.414, 1.732, 2.236].into(),
                name: Some("Today".to_string())
            }
        );
        assert_eq!(
            DataPoint::from((vec![value(42), value(3.14), value("Monday")], "Today")),
            DataPoint::Item {
                value: vec![value(42), value(3.14), value("Monday")].into(),
                name: Some("Today".to_string())
            }
        );
    }
}
