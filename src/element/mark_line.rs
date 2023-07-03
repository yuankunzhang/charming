use serde::{ser::SerializeSeq, Serialize};

use crate::datatype::Value;

use super::{label::Label, symbol::Symbol};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkLineDataType {
    Min,
    Max,
    Average,
    Median,
}

impl From<&str> for MarkLineDataType {
    fn from(s: &str) -> Self {
        match s {
            "min" => Self::Min,
            "max" => Self::Max,
            "avg" | "average" => Self::Average,
            "med" | "median" => Self::Median,
            _ => panic!("Invalid MarkLineDataType"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkLineData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<MarkLineDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,
}

impl MarkLineData {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            symbol: None,
            x: None,
            x_axis: None,
            y_axis: None,
            label: None,
        }
    }

    pub fn type_<T: Into<MarkLineDataType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn symbol(mut self, symbol: Symbol) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn x<S: Into<String>>(mut self, x: S) -> Self {
        self.x = Some(x.into());
        self
    }

    pub fn x_axis<V: Into<Value>>(mut self, x_axis: V) -> Self {
        self.x_axis = Some(x_axis.into());
        self
    }

    pub fn y_axis<V: Into<Value>>(mut self, y_axis: V) -> Self {
        self.y_axis = Some(y_axis.into());
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }
}

impl From<(&str, &str)> for MarkLineData {
    fn from((type_, name): (&str, &str)) -> Self {
        Self::new().type_(type_).name(name)
    }
}

pub enum MarkLineVariant {
    Simple(MarkLineData),
    StartToEnd(MarkLineData, MarkLineData),
}

impl Serialize for MarkLineVariant {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarkLineVariant::Simple(data) => data.serialize(serializer),
            MarkLineVariant::StartToEnd(start, end) => {
                let mut s = serializer.serialize_seq(Some(2))?;
                s.serialize_element(start)?;
                s.serialize_element(end)?;
                s.end()
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    symbol: Vec<Symbol>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<MarkLineVariant>,
}

impl MarkLine {
    pub fn new() -> Self {
        Self {
            label: None,
            symbol: vec![],
            data: vec![],
        }
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn symbol(mut self, symbol: Vec<Symbol>) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn data(mut self, data: Vec<MarkLineVariant>) -> Self {
        self.data = data;
        self
    }
}
