use crate::element::{Emphasis, ItemStyle, Label, Sort, Tooltip};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SunburstLevel {
    r0: Option<String>,
    r: Option<String>,
    item_style: Option<ItemStyle>,
    label: Option<Label>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SunburstNode {
    name: String,
    value: Option<f64>,
    #[serde(skip_deserializing)]
    item_style: Option<ItemStyle>,
    children: Vec<SunburstNode>,
}

impl SunburstNode {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            value: None,
            item_style: None,
            children: vec![],
        }
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }

    pub fn children(mut self, children: Vec<SunburstNode>) -> Self {
        self.children = children;
        self
    }
}

impl From<&str> for SunburstNode {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<(&str, f64)> for SunburstNode {
    fn from((name, value): (&str, f64)) -> Self {
        Self::new(name).value(value)
    }
}

impl From<(&str, f64, &str)> for SunburstNode {
    fn from((name, value, color): (&str, f64, &str)) -> Self {
        Self::new(name)
            .value(value)
            .item_style(ItemStyle::new().color(color))
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sunburst {
    #[serde(rename = "type")]
    #[charming_type = "sunburst"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    z_level: Option<u64>,
    z: Option<u64>,
    #[charming_skip_setter]
    center: Option<(String, String)>,
    #[charming_skip_setter]
    radius: Option<(String, String)>,
    emphasis: Option<Emphasis>,
    sort: Option<Sort>,
    #[charming_set_vec]
    levels: Vec<SunburstLevel>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<SunburstNode>,
}

impl Sunburst {
    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn radius<S: Into<String>>(mut self, radius: (S, S)) -> Self {
        self.radius = Some((radius.0.into(), radius.1.into()));
        self
    }
}
