use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{
        BackgroundStyle, ColorBy, CoordinateSystem, Emphasis, ItemStyle, Label, MarkLine, Sampling,
        Tooltip,
    },
};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Bar {
    stack_order: Option<crate::element::StackOrder>,
    polar_id: Option<String>,
    x_axis_id: Option<String>,
    y_axis_id: Option<String>,
    #[serde(rename = "type")]
    #[charming_type = "bar"]
    type_: String,
    id: Option<String>,
    color_by: Option<ColorBy>,
    name: Option<String>,
    legend_hover_link: Option<bool>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    polar_index: Option<f64>,
    round_cap: Option<bool>,
    realtime_sort: Option<bool>,
    show_background: Option<bool>,
    background_style: Option<BackgroundStyle>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    mark_line: Option<MarkLine>,
    stack: Option<String>,
    sampling: Option<Sampling>,
    bar_width: Option<CompositeValue>,
    bar_gap: Option<CompositeValue>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
