use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{ColorBy, Emphasis, ItemStyle, Label, LabelLine, Orient, Sort, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    Left,
    Right,
    Center,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Funnel {
    #[serde(rename = "type")]
    #[charming_type = "funnel"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    min: Option<f64>,
    max: Option<f64>,
    min_size: Option<String>,
    max_size: Option<String>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    orient: Option<Orient>,
    sort: Option<Sort>,
    gap: Option<f64>,
    legend_hover_link: Option<bool>,
    funnel_align: Option<Align>,
    label: Option<Label>,
    label_line: Option<LabelLine>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
