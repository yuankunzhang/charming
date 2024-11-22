use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize,
};

use crate::{
    datatype::CompositeValue,
    element::{BoundaryGap, ColorBy, CoordinateSystem, Label, Tooltip},
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

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThemeRiver {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<BoundaryGap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip: Option<Tooltip>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<ThemeRiverData>,
}

impl Default for ThemeRiver {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeRiver {
    pub fn new() -> Self {
        Self {
            type_: "themeRiver".to_string(),
            id: None,
            name: None,
            color_by: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            coordinate_system: None,
            boundary_gap: None,
            label: None,
            tooltip: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width<C: Into<CompositeValue>>(mut self, width: C) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<C: Into<CompositeValue>>(mut self, height: C) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
        self
    }

    pub fn boundary_gap<B: Into<BoundaryGap>>(mut self, boundary_gap: B) -> Self {
        self.boundary_gap = Some(boundary_gap.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn tooltip(mut self, tooltip: Tooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn data<T: Into<ThemeRiverData>>(mut self, data: Vec<T>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
