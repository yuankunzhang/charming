use serde::Serialize;

use crate::element::ItemStyle;

use super::CompositeValue;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPoint {
    value: CompositeValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,
}

impl DataPoint {
    pub fn new<C: Into<CompositeValue>>(value: C) -> Self {
        Self {
            value: value.into(),
            name: None,
            item_style: None,
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }
}

impl<V> From<V> for DataPoint
where
    V: Into<CompositeValue>,
{
    fn from(v: V) -> Self {
        DataPoint::new(v)
    }
}

impl<V, S> From<(V, S)> for DataPoint
where
    V: Into<CompositeValue>,
    S: Into<String>,
{
    fn from(v: (V, S)) -> Self {
        DataPoint::new(v.0).name(v.1)
    }
}

#[macro_export]
macro_rules! dp {
    ($v:expr) => {
        DataPoint::new($v)
    };
    ($v:expr, $name:expr) => {
        DataPoint::new($v).name($name)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn data_point_from_composite_value() {
        let p: DataPoint = 42.into();
        let q = DataPoint {
            value: 42.into(),
            name: None,
            item_style: None,
        };
        assert_eq!(p.value, q.value);

        let p: DataPoint = (-3.14).into();
        let q = DataPoint {
            value: (-3.14).into(),
            name: None,
            item_style: None,
        };
        assert_eq!(p.value, q.value);

        let p: DataPoint = "foo".into();
        let q = DataPoint {
            value: "foo".into(),
            name: None,
            item_style: None,
        };
        assert_eq!(p.value, q.value);

        let p: DataPoint = vec![
            CompositeValue::from(42),
            CompositeValue::from(-3.14),
            CompositeValue::from("foo"),
        ]
        .into();
        let q = DataPoint {
            value: vec![
                CompositeValue::from(42),
                CompositeValue::from(-3.14),
                CompositeValue::from("foo"),
            ]
            .into(),
            name: None,
            item_style: None,
        };
        assert_eq!(p.value, q.value);
    }

    #[test]
    fn data_point_from_tuple() {
        let p: DataPoint = (42, "foo").into();
        let q = DataPoint {
            value: 42.into(),
            name: Some("foo".to_string()),
            item_style: None,
        };
        assert_eq!(p.value, q.value);
        assert_eq!(p.name, q.name);
    }
}
