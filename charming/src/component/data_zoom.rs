use crate::{
    datatype::{CompositeValue, NumberOrArray, StringOrNumber},
    element::{Color, DataBackground, Formatter, ItemStyle, Orient, TextStyle},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Filter,
    WeakFilter,
    Empty,
    None,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum DataZoomType {
    Inside,
    Slider,
    Select,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MouseWheelModifier {
    Ctrl,
    Shift,
    Alt,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(untagged)]
pub enum MouseZoom {
    Bool(bool),
    Modifier(MouseWheelModifier),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RangeMode {
    Value,
    Percent,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataZoomEmphasis {
    handle_style: Option<ItemStyle>,
    move_handle_style: Option<ItemStyle>,
}

/// DataZoom component is used for zooming a specific area, which enables user
/// to investigate data in detail, or get an overview of the data, or get rid
/// of outlier points.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataZoom {
    #[serde(rename = "type")]
    type_: Option<DataZoomType>,
    /// Component ID.
    id: Option<String>,
    /// Whether to show the component.
    show: Option<bool>,
    /// Whether to enable real-time view update.
    realtime: Option<bool>,
    /// Background color of the component.
    background_color: Option<Color>,
    /// Style of the data shadow.
    data_background: Option<DataBackground>,
    /// Style of the selected data shadow.
    selected_data_background: Option<DataBackground>,
    /// Color to fill selected area.
    filler_color: Option<Color>,
    /// Color of border.
    border_color: Option<Color>,
    /// Border radius of the slider, as a number or an array of corner radii.
    border_radius: Option<NumberOrArray>,
    start: Option<f64>,
    end: Option<f64>,
    start_value: Option<CompositeValue>,
    end_value: Option<CompositeValue>,
    min_span: Option<f64>,
    max_span: Option<f64>,
    min_value_span: Option<f64>,
    max_value_span: Option<f64>,
    orient: Option<Orient>,
    zoom_lock: Option<bool>,
    throttle: Option<f64>,
    /// Range interpretation for the start and end, respectively.
    range_mode: Option<[RangeMode; 2]>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    /// Slider width in pixels or as a percentage string.
    width: Option<StringOrNumber>,
    /// Slider height in pixels or as a percentage string.
    height: Option<StringOrNumber>,
    x_axis_index: Option<CompositeValue>,
    y_axis_index: Option<CompositeValue>,
    disabled: Option<bool>,
    radius_axis_index: Option<f64>,
    angle_axis_index: Option<f64>,
    filter_mode: Option<FilterMode>,
    text_style: Option<TextStyle>,
    handle_icon: Option<String>,
    /// Slider handle size in pixels or as a percentage of slider height.
    handle_size: Option<StringOrNumber>,
    /// Style of the slider handles.
    handle_style: Option<ItemStyle>,
    /// Icon of the slider's draggable panel.
    move_handle_icon: Option<String>,
    /// Height of the slider's move handle in pixels.
    move_handle_size: Option<f64>,
    /// Style of the slider's move handle.
    move_handle_style: Option<ItemStyle>,
    /// Decimal precision of slider labels: a number or `"auto"`.
    label_precision: Option<StringOrNumber>,
    /// Slider label template or callback receiving the value and formatted value.
    label_formatter: Option<Formatter>,
    /// Whether to show slider detail labels.
    show_detail: Option<bool>,
    /// Whether to show the slider data shadow; omission uses ECharts' `"auto"`.
    show_data_shadow: Option<bool>,
    brush_select: Option<bool>,
    /// Style of the slider's brush selection.
    brush_style: Option<ItemStyle>,
    /// Slider handle styles on emphasis.
    emphasis: Option<DataZoomEmphasis>,
    /// Canvas layer for the slider.
    zlevel: Option<f64>,
    /// Drawing order within the slider's canvas layer.
    z: Option<f64>,
    /// Whether inside zoom responds to the mouse wheel, optionally with a modifier.
    zoom_on_mouse_wheel: Option<MouseZoom>,
    /// Whether inside zoom pans on mouse move, optionally with a modifier.
    move_on_mouse_move: Option<MouseZoom>,
    /// Whether inside zoom pans on the mouse wheel, optionally with a modifier.
    move_on_mouse_wheel: Option<MouseZoom>,
    /// Whether inside zoom prevents the default mouse-move event behavior.
    prevent_default_mouse_move: Option<bool>,
}
