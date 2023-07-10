use serde::{ser::SerializeStruct, Serialize};

use super::CompositeValue;

#[derive(Debug, PartialEq)]
pub enum DataPoint {
    Value(CompositeValue),
    Item {
        value: CompositeValue,
        name: Option<String>,
    },
}

impl Serialize for DataPoint {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DataPoint::Value(v) => v.serialize(serializer),
            DataPoint::Item { value, name } => {
                let mut s = serializer
                    .serialize_struct("DataPoint", if let Some(_) = name { 2 } else { 1 })?;
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
    V: Into<CompositeValue>,
{
    fn from(v: V) -> Self {
        DataPoint::Value(v.into())
    }
}

impl<V> From<(V, &str)> for DataPoint
where
    V: Into<CompositeValue>,
{
    fn from(v: (V, &str)) -> Self {
        DataPoint::Item {
            value: v.0.into(),
            name: Some(v.1.to_string()),
        }
    }
}

#[macro_export]
macro_rules! dp {
    ($v:expr) => {
        DataPoint::Value($v.into())
    };
    ($v:expr, $name:expr) => {
        DataPoint::Item {
            value: $v.into(),
            name: Some($name.to_string()),
        }
    };
}
