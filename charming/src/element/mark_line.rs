use charming_macros::CharmingSetters;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, SeqAccess, Visitor},
    ser::SerializeSeq,
};

use crate::datatype::CompositeValue;

use super::{label::Label, line_style::LineStyle, symbol::Symbol};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
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

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkLineData {
    relative_to: Option<super::MarkerRelativeTo>,
    z2: Option<f64>,
    line_style: Option<LineStyle>,
    item_style: Option<super::ItemStyle>,
    symbol_offset: Option<CompositeValue>,
    symbol_size: Option<CompositeValue>,
    symbol_rotate: Option<CompositeValue>,
    #[serde(rename = "type")]
    type_: Option<MarkLineDataType>,

    name: Option<String>,

    symbol: Option<Symbol>,

    x: Option<CompositeValue>,

    y: Option<CompositeValue>,

    x_axis: Option<CompositeValue>,

    y_axis: Option<CompositeValue>,

    coord: Option<CompositeValue>,

    label: Option<Label>,
}

impl From<(&str, &str)> for MarkLineData {
    fn from((type_, name): (&str, &str)) -> Self {
        Self::new().type_(type_).name(name)
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
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

impl<'de> Deserialize<'de> for MarkLineVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MarkLineVariantVisitor;

        impl<'de> Visitor<'de> for MarkLineVariantVisitor {
            type Value = MarkLineVariant;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a single MarkLineData or a sequence of two MarkLineData")
            }

            fn visit_newtype_struct<E>(self, value: E) -> Result<Self::Value, E::Error>
            where
                E: Deserializer<'de>,
            {
                let data = MarkLineData::deserialize(value)?;
                Ok(MarkLineVariant::Simple(data))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                // Deserialize a single object using a map access
                let value: MarkLineData =
                    Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(MarkLineVariant::Simple(value))
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let first: MarkLineData = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let second: MarkLineData = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                if (seq.next_element::<MarkLineData>()?).is_some() {
                    return Err(de::Error::invalid_length(
                        3,
                        &"expected array of exactly two elements",
                    ));
                }
                Ok(MarkLineVariant::StartToEnd(first, second))
            }
        }

        deserializer.deserialize_any(MarkLineVariantVisitor)
    }
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkLine {
    z2: Option<f64>,
    symbol_offset: Option<CompositeValue>,
    symbol_size: Option<CompositeValue>,
    symbol_rotate: Option<CompositeValue>,
    emphasis: Option<Box<super::Emphasis>>,
    blur: Option<Box<super::Blur>>,
    select: Option<Box<super::Select>>,
    label: Option<Label>,

    line_style: Option<LineStyle>,

    zlevel: Option<f64>,

    z: Option<f64>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[charming_set_vec]
    symbol: Vec<Symbol>,

    precision: Option<f64>,

    silent: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    #[charming_set_vec]
    data: Vec<MarkLineVariant>,
}
