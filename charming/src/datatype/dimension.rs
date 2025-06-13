use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(untagged)]
pub enum DimensionType {
    Number,
    Float,
    Int,
    Ordinal,
    Time,
}

impl From<&str> for DimensionType {
    fn from(s: &str) -> Self {
        match s {
            "number" => Self::Number,
            "float" => Self::Float,
            "int" => Self::Int,
            "ordinal" => Self::Ordinal,
            "time" => Self::Time,
            _ => panic!("Invalid dimension type: {s}"),
        }
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dimension {
    #[serde(rename = "type")]
    type_: Option<DimensionType>,
    name: Option<String>,
    display_name: Option<String>,
}

impl From<&str> for Dimension {
    fn from(name: &str) -> Self {
        Self::new().name(name)
    }
}

impl From<(&str, &str)> for Dimension {
    fn from((name, type_): (&str, &str)) -> Self {
        Self::new().name(name).type_(type_)
    }
}

impl From<(&str, &str, &str)> for Dimension {
    fn from((name, type_, display_name): (&str, &str, &str)) -> Self {
        Self::new()
            .name(name)
            .type_(type_)
            .display_name(display_name)
    }
}

/// The `dim` macro can construct a Vec<[Dimension]>.
/// ```rust
/// use charming::datatype::Dimension;
/// use charming::dim;
///
/// let data: Vec<Dimension> = dim![
///    "name1",
///    ("name2", "number")
/// ];
/// ```
#[macro_export]
macro_rules! dim {
    ($($x:expr),*) => {
        vec![
            $(
                $crate::datatype::Dimension::from($x)
            ),*
        ]
    };
}
