use crate::{
    datatype::StringOrNumber,
    element::{BooleanOrAuto, ItemStyle},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisBreak {
    start: Option<StringOrNumber>,
    end: Option<StringOrNumber>,
    gap: Option<StringOrNumber>,
    is_expanded: Option<bool>,
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BreakArea {
    show: Option<bool>,
    item_style: Option<ItemStyle>,
    zigzag_amplitude: Option<f64>,
    zigzag_min_span: Option<f64>,
    zigzag_max_span: Option<f64>,
    zigzag_z: Option<f64>,
    expand_on_click: Option<bool>,
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BreakLabelLayout {
    move_overlap: Option<BooleanOrAuto>,
}

/// Enables tooltips for axis labels and names.
#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisTooltip {
    show: Option<bool>,
}
