use crate::{datatype::CompositeValue, element::Orient};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SaveAsImageType {
    Png,
    Jpg,
    Svg,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SaveAsImage {
    show: Option<bool>,
    #[serde(rename = "type")]
    type_: Option<SaveAsImageType>,
    name: Option<String>,
    background_color: Option<String>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Restore {
    show: Option<bool>,
    title: Option<String>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataView {
    show: Option<bool>,
    title: Option<String>,
    read_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum MagicTypeType {
    /// For line charts.
    Line,
    /// For bar charts.
    Bar,
    /// For stacked charts.
    Stack,
}

impl From<&str> for MagicTypeType {
    fn from(s: &str) -> Self {
        match s {
            "line" => Self::Line,
            "bar" => Self::Bar,
            "stack" => Self::Stack,
            _ => panic!("Invalid magic type type: {s}"),
        }
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MagicType {
    type_: Option<Vec<MagicTypeType>>,
    title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum BrushType {
    Rect,
    Polygon,
    LineX,
    LineY,
    Keep,
    Clear,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Brush {
    #[serde(rename = "type")]
    #[charming_set_vec]
    type_: Vec<BrushType>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxDataZoom {
    y_axis_index: Option<CompositeValue>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    save_as_image: Option<SaveAsImage>,
    restore: Option<Restore>,
    data_view: Option<DataView>,
    magic_type: Option<MagicType>,
    data_zoom: Option<ToolboxDataZoom>,
    brush: Option<Brush>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Toolbox {
    show: Option<bool>,
    feature: Option<Feature>,
    orient: Option<Orient>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
}

impl Toolbox {
    pub fn save_as_image_type(&self) -> Option<&SaveAsImageType> {
        self.feature
            .as_ref()
            .and_then(|f| f.save_as_image.as_ref())
            .and_then(|s| s.type_.as_ref())
    }
}
