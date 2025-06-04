use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize};

/// The boundary gap on both sides of a coordinate axis. The setting and
/// behavior of category axes and non-category axes are different.
///
/// The `BoundaryGap` of category axis can be set to either `true` or `false`.
/// Default value is `true`.
///
/// For non-category axis, including time, numerical value, and log axes,
/// `BoundaryGap` is an array of two values, representing the spanning range
/// between minimum and maximum value.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BoundaryGap {
    CategoryAxis(bool),
    NonCategoryAxis(String, String),
}

impl Serialize for BoundaryGap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            BoundaryGap::CategoryAxis(b) => b.serialize(serializer),
            BoundaryGap::NonCategoryAxis(min, max) => {
                let mut s = serializer.serialize_seq(Some(2))?;
                s.serialize_element(min)?;
                s.serialize_element(max)?;
                s.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for BoundaryGap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BoundaryGapVisitor;

        impl<'de> Visitor<'de> for BoundaryGapVisitor {
            type Value = BoundaryGap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean or a sequence of two strings")
            }

            fn visit_bool<E>(self, value: bool) -> Result<BoundaryGap, E>
            where
                E: serde::de::Error,
            {
                Ok(BoundaryGap::CategoryAxis(value))
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<BoundaryGap, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let min: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let max: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(BoundaryGap::NonCategoryAxis(min, max))
            }
        }

        deserializer.deserialize_any(BoundaryGapVisitor)
    }
}

impl From<bool> for BoundaryGap {
    fn from(b: bool) -> Self {
        BoundaryGap::CategoryAxis(b)
    }
}

impl From<(&str, &str)> for BoundaryGap {
    fn from((min, max): (&str, &str)) -> Self {
        BoundaryGap::NonCategoryAxis(min.to_string(), max.to_string())
    }
}
