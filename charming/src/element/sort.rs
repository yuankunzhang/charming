use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Sort {
    Ascending,
    Descending,
    None,
}

impl Serialize for Sort {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Sort::Ascending => serializer.serialize_str("ascending"),
            Sort::Descending => serializer.serialize_str("descending"),
            Sort::None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Sort {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SortVisitor;

        impl Visitor<'_> for SortVisitor {
            type Value = Sort;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#""ascending", "descending", or null"#)
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Sort, E> {
                match value {
                    "ascending" => Ok(Sort::Ascending),
                    "descending" => Ok(Sort::Descending),
                    "" => Ok(Sort::None),
                    _ => Err(de::Error::unknown_variant(
                        value,
                        &["ascending", "descending"],
                    )),
                }
            }
        }

        deserializer.deserialize_any(SortVisitor)
    }
}
