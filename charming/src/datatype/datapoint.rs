use std::fmt::Debug;

use serde::Serialize;

use crate::element::ItemStyle;

use super::CompositeValue;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPointItem {
    value: CompositeValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,
}

impl DataPointItem {
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

impl Debug for DataPointItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataPointItem")
            .field("value", &self.value)
            .field("name", &self.name)
            .finish()
    }
}

impl PartialEq for DataPointItem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.name == other.name
    }
}

impl<V> From<V> for DataPointItem
where
    V: Into<CompositeValue>,
{
    fn from(v: V) -> Self {
        DataPointItem::new(v)
    }
}

impl<V, S> From<(V, S)> for DataPointItem
where
    V: Into<CompositeValue>,
    S: Into<String>,
{
    fn from(v: (V, S)) -> Self {
        DataPointItem::new(v.0).name(v.1)
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DataPoint {
    Value(CompositeValue),
    Item(DataPointItem),
}

impl<V> From<V> for DataPoint
where
    V: Into<CompositeValue>,
{
    fn from(v: V) -> Self {
        DataPoint::Value(v.into())
    }
}

impl<V, S> From<(V, S)> for DataPoint
where
    V: Into<CompositeValue>,
    S: Into<String>,
{
    fn from(v: (V, S)) -> Self {
        DataPoint::Item(DataPointItem::new(v.0).name(v.1))
    }
}

impl From<DataPointItem> for DataPoint {
    fn from(item: DataPointItem) -> Self {
        DataPoint::Item(item)
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
        let q = DataPoint::Value(42.into());
        assert_eq!(p, q);

        let p: DataPoint = (-std::f32::consts::PI).into();
        let q = DataPoint::Value((-std::f32::consts::PI).into());
        assert_eq!(p, q);

        let p: DataPoint = "foo".into();
        let q = DataPoint::Value("foo".into());
        assert_eq!(p, q);

        let p: DataPoint = vec![
            CompositeValue::from(42),
            CompositeValue::from(-std::f32::consts::PI),
            CompositeValue::from("foo"),
        ]
        .into();
        let q = DataPoint::Value(
            vec![
                CompositeValue::from(42),
                CompositeValue::from(-std::f32::consts::PI),
                CompositeValue::from("foo"),
            ]
            .into(),
        );
        assert_eq!(p, q);
    }

    #[test]
    fn data_point_from_tuple() {
        let p: DataPoint = (42, "foo").into();
        let q = DataPoint::Item((42, "foo").into());
        assert_eq!(p, q);
    }
}
