use serde::{ser::SerializeSeq, Serialize};

/// The boundary gap on both sides of a coordinate axis. The setting and
/// behavior of category axes and non-category axes are different.
///
/// The `BoundaryGap` of category axis can be set to either `true` or `false`.
/// Default value is `true`.
///
/// For non-category axis, including time, numerical value, and log axes,
/// `BoundaryGap` is an array of two values, representing the spanning range
/// between minimum and maximum value.
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
