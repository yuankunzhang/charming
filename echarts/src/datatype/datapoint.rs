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

impl<V> From<(V, &str)> for DataPoint
where
    V: Into<CompositeValue>,
{
    fn from(v: (V, &str)) -> Self {
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
