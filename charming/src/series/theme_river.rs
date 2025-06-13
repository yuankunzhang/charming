use crate::{
    datatype::CompositeValue,
    element::{BoundaryGap, ColorBy, CoordinateSystem, Label, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ThemeRiverData {
    date: CompositeValue,
    value: CompositeValue,
    name: CompositeValue,
}

impl ThemeRiverData {
    pub fn new<D, V, N>(date: D, value: V, name: N) -> Self
    where
        D: Into<CompositeValue>,
        V: Into<CompositeValue>,
        N: Into<CompositeValue>,
    {
        Self {
            date: date.into(),
            value: value.into(),
            name: name.into(),
        }
    }
}

impl Serialize for ThemeRiverData {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_seq(None)?;
        s.serialize_element(&self.date)?;
        s.serialize_element(&self.value)?;
        s.serialize_element(&self.name)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for ThemeRiverData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThemeRiverDataVisitor;

        impl<'de> Visitor<'de> for ThemeRiverDataVisitor {
            type Value = ThemeRiverData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of three CompositeValue elements")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ThemeRiverData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let date = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let value = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let name = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;

                Ok(ThemeRiverData { date, value, name })
            }
        }

        deserializer.deserialize_seq(ThemeRiverDataVisitor)
    }
}

impl<D, V, N> From<(D, V, N)> for ThemeRiverData
where
    D: Into<CompositeValue>,
    V: Into<CompositeValue>,
    N: Into<CompositeValue>,
{
    fn from(v: (D, V, N)) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThemeRiver {
    #[serde(rename = "type")]
    #[charming_type = "themeRiver"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    coordinate_system: Option<CoordinateSystem>,
    boundary_gap: Option<BoundaryGap>,
    label: Option<Label>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<ThemeRiverData>,
}
