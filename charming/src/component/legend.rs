use crate::{
    datatype::CompositeValue,
    element::{
        AnimationTime, Color, Icon, ItemStyle, LabelAlign, LineStyle, Orient, Padding, TextStyle,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum LegendConfig {
    Single(Box<Legend>),
    Multiple(Vec<Legend>),
}

impl From<Legend> for LegendConfig {
    fn from(legend: Legend) -> Self {
        Self::Single(Box::new(legend))
    }
}

impl From<Vec<Legend>> for LegendConfig {
    fn from(legends: Vec<Legend>) -> Self {
        LegendConfig::Multiple(legends)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LegendType {
    /// Simple legend.
    Plain,

    /// Scrollable legend.
    Scroll,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LegendSelectedMode {
    /// Multiple selection.
    Multiple,

    /// Single selection.
    Single,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LegendItem {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

impl From<&str> for LegendItem {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            icon: None,
        }
    }
}

impl From<String> for LegendItem {
    fn from(name: String) -> Self {
        Self { name, icon: None }
    }
}

impl From<(&str, &str)> for LegendItem {
    fn from((name, icon): (&str, &str)) -> Self {
        Self {
            name: name.to_string(),
            icon: Some(icon.into()),
        }
    }
}

impl From<(String, String)> for LegendItem {
    fn from((name, icon): (String, String)) -> Self {
        Self {
            name,
            icon: Some(icon.into()),
        }
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Legend {
    /// Type of legend.
    #[serde(rename = "type")]
    type_: Option<LegendType>,
    /// Component ID.
    id: Option<String>,
    /// Whether to show the legend component.
    show: Option<bool>,
    /// The `zlevel` value of all graphical elements in.
    zlevel: Option<f64>,
    /// The `z` value of all graphical elements in.
    z: Option<f64>,
    /// Distance between title component and the left side of the container.
    left: Option<CompositeValue>,
    /// Distance between title component and the top side of the container.
    top: Option<CompositeValue>,
    /// Distance between title component and the right side of the container.
    right: Option<CompositeValue>,
    /// Distance between title component and the bottom side of the container.
    bottom: Option<CompositeValue>,
    /// Width of legend component.
    width: Option<CompositeValue>,
    /// Height of legend component.
    height: Option<CompositeValue>,
    /// The layout orientation of legend.
    orient: Option<Orient>,
    /// The align of legend.
    align: Option<LabelAlign>,
    /// Legend padding.
    padding: Option<Padding>,
    /// The gap between each legend.
    item_gap: Option<f64>,
    /// Width of legend symbol.
    item_width: Option<f64>,
    /// Height of legend symbol.
    item_height: Option<f64>,
    /// Legend item style.
    item_style: Option<ItemStyle>,
    /// Legend line style.
    line_style: Option<LineStyle>,
    text_style: Option<TextStyle>,
    /// Rotation of the symbol.
    symbol_rotate: Option<String>,
    /// Formatter is used to format label of legend.
    formatter: Option<String>,
    #[charming_skip_setter]
    selected: Option<BTreeMap<String, bool>>,
    selected_mode: Option<LegendSelectedMode>,
    border_color: Option<Color>,
    inactive_color: Option<Color>,
    #[charming_set_vec]
    data: Vec<LegendItem>,
    animation: Option<bool>,
    animation_duration_update: Option<AnimationTime>,
}

impl Legend {
    pub fn selected<S: Into<String>, I: IntoIterator<Item = (S, bool)>>(
        mut self,
        selected: I,
    ) -> Self {
        self.selected = Some(selected.into_iter().map(|(k, v)| (k.into(), v)).collect());
        self
    }
}
