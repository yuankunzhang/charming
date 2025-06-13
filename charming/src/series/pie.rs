use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{ColorBy, CoordinateSystem, Emphasis, ItemStyle, Label, LabelLine, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PieRoseType {
    Radius,
    Area,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pie {
    #[serde(rename = "type")]
    #[charming_type = "pie"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    legend_hover_link: Option<bool>,
    coordiate_system: Option<CoordinateSystem>,
    geo_index: Option<f64>,
    calendar_index: Option<f64>,
    selected_mode: Option<bool>,
    selected_offset: Option<f64>,
    clockwise: Option<bool>,
    avoid_label_overlap: Option<bool>,
    start_angle: Option<f64>,
    rose_type: Option<PieRoseType>,
    label: Option<Label>,
    label_line: Option<LabelLine>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    center: Option<CompositeValue>,
    radius: Option<CompositeValue>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
