use super::color::Color;
use charming_macros::CharmingSetters;
use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ColorSegment(f64, Color);

impl Serialize for ColorSegment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.0)?;
        seq.serialize_element(&self.1)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for ColorSegment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorSegmentVisitor;

        impl<'de> Visitor<'de> for ColorSegmentVisitor {
            type Value = ColorSegment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence with a float and a Color")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ColorSegment, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let position: f64 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let color: Color = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(ColorSegment(position, color))
            }
        }

        deserializer.deserialize_seq(ColorSegmentVisitor)
    }
}

impl From<(f64, &str)> for ColorSegment {
    fn from(tuple: (f64, &str)) -> Self {
        Self(tuple.0, tuple.1.into())
    }
}

impl From<(f64, Color)> for ColorSegment {
    fn from(tuple: (f64, Color)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisLineStyle {
    color: Vec<ColorSegment>,
    width: Option<f64>,
    shadow_blur: Option<f64>,
    shadow_color: Option<Color>,
    shadow_offset_x: Option<f64>,
    shadow_offset_y: Option<f64>,
    opacity: Option<f64>,
}

impl From<(f64, &str)> for AxisLineStyle {
    fn from(tuple: (f64, &str)) -> Self {
        Self::new().color(tuple)
    }
}

impl From<(f64, &str, f64)> for AxisLineStyle {
    fn from(tuple: (f64, &str, f64)) -> Self {
        Self::new().color((tuple.0, tuple.1)).width(tuple.2)
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisLine {
    show: Option<bool>,
    on_zero: Option<bool>,
    round_cap: Option<bool>,
    line_style: Option<AxisLineStyle>,
}
