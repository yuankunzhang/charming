use super::{AreaStyle, Label, item_style::ItemStyle};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum EmphasisFocus {
    None,
    #[serde(rename = "self")]
    Self_,
    Series,
    Ancestor,
    Descendant,
    Relative,
    Adjacency,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Emphasis {
    line_style: Option<super::LineStyle>,
    edge_label: Option<Label>,
    blur_scope: Option<BlurScope>,
    scale: Option<EmphasisScale>,
    z2: Option<f64>,
    focus: Option<EmphasisFocus>,
    item_style: Option<ItemStyle>,
    area_style: Option<AreaStyle>,
    label: Option<Label>,
    disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum BlurScope {
    CoordinateSystem,
    Series,
    Global,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(untagged)]
pub enum EmphasisScale {
    Bool(bool),
    Number(f64),
}

impl From<bool> for EmphasisScale {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<f64> for EmphasisScale {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
