use std::fmt;

use charming_macros::CharmingSetters;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::element::JsFunction;

#[derive(Debug, PartialEq, Clone)]
pub enum SeriesKey {
    String(String),
    Array(Vec<String>),
}

impl Serialize for SeriesKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SeriesKey::String(s) => serializer.serialize_str(s),
            SeriesKey::Array(items) => items.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for SeriesKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SeriesKeyVisitor;

        impl<'de> Visitor<'de> for SeriesKeyVisitor {
            type Value = SeriesKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an array of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SeriesKey::String(value.to_owned()))
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let values: Vec<String> =
                    Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))?;
                Ok(SeriesKey::Array(values))
            }
        }

        deserializer.deserialize_any(SeriesKeyVisitor)
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransition {
    enabled: Option<bool>,
    series_key: Option<SeriesKey>,
    divide_shape: Option<String>,
    delay: Option<JsFunction>,
}
