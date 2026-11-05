use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint},
    element::{AxisType, ItemStyle, Label, LineStyle, Orient, Symbol, SymbolSize, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimelineControlStyle {
    show: Option<bool>,
    show_play_btn: Option<bool>,
    show_prev_btn: Option<bool>,
    show_next_btn: Option<bool>,
    item_size: Option<f64>,
    item_gap: Option<f64>,
    position: Option<crate::element::LabelPosition>,
    color: Option<crate::element::Color>,
    border_color: Option<crate::element::Color>,
    border_width: Option<f64>,
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    coordinate_system: Option<crate::element::CoordinateSystem>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
    id: Option<String>,
    show: Option<bool>,
    axis_type: Option<AxisType>,
    current_index: Option<usize>,
    auto_play: Option<bool>,
    rewind: Option<bool>,
    #[serde(rename = "loop")]
    loop_: Option<bool>,
    play_interval: Option<f64>,
    realtime: Option<bool>,
    orient: Option<Orient>,
    inverse: Option<bool>,
    left: Option<CompositeValue>,
    right: Option<CompositeValue>,
    top: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    line_style: Option<LineStyle>,
    item_style: Option<ItemStyle>,
    checkpoint_style: Option<ItemStyle>,
    control_style: Option<TimelineControlStyle>,
    label: Option<Label>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
