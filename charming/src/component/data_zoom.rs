use crate::{
    datatype::{CompositeValue, NumberOrArray, StringOrNumber},
    element::{Color, DataBackground, JsFunction, Orient, StrokeStyle, TextStyle},
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
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HandleLabel {
    show: Option<bool>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Emphasis {
    handle_style: Option<StrokeStyle>,
    // Since v5.6.0
    handle_label: Option<HandleLabel>,
    move_handle_style: Option<StrokeStyle>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataZoom {
    #[serde(rename = "type")]
    type_: Option<DataZoomType>,
    /// Component ID. Inside, Slider
    id: Option<String>,
    /// Whether to show the component. Slider
    show: Option<bool>,
    /// Whether to enable real-time view update. Slider
    realtime: Option<bool>,
    /// Background color of the component. Slider
    background_color: Option<Color>,
    /// Style of the data shadow. Slider
    data_background: Option<DataBackground>,
    /// Style of the selected data shadow. Slider
    selected_data_background: Option<DataBackground>,
    /// Color to fill selected area. Slider
    filler_color: Option<Color>,
    /// Color of border. Slider
    border_color: Option<Color>,
    /// Border radius. Slider
    border_radius: Option<NumberOrArray>,
    /// Inside, Slider
    start: Option<f64>,
    /// Inside, Slider
    end: Option<f64>,
    /// Inside, Slider
    start_value: Option<CompositeValue>,
    /// Inside, Slider
    end_value: Option<CompositeValue>,
    /// Inside, Slider
    min_span: Option<f64>,
    /// Inside, Slider
    max_span: Option<f64>,
    /// Inside, Slider
    min_value_span: Option<f64>,
    /// Inside, Slider
    max_value_span: Option<f64>,
    /// Inside, Slider
    orient: Option<Orient>,
    /// Inside, Slider
    zoom_lock: Option<bool>,
    /// Inside, Slider
    throttle: Option<f64>,
    /// Inside, Slider
    range_mode: Option<[RangeMode; 2]>,
    /// Slider
    left: Option<CompositeValue>,
    /// Slider
    top: Option<CompositeValue>,
    /// Slider
    right: Option<CompositeValue>,
    /// Slider
    bottom: Option<CompositeValue>,
    /// Slider
    width: Option<StringOrNumber>,
    /// Slider
    height: Option<StringOrNumber>,
    /// Inside, Slider, Select
    x_axis_index: Option<CompositeValue>,
    /// Inside, Slider, Select
    y_axis_index: Option<CompositeValue>,
    /// Inside
    disabled: Option<bool>,
    /// Inside, Slider
    radius_axis_index: Option<f64>,
    /// Inside, Slider
    angle_axis_index: Option<f64>,
    /// Slider
    filter_mode: Option<FilterMode>,
    /// Slider
    text_style: Option<TextStyle>,
    /// Slider
    handle_icon: Option<String>,
    /// Slider
    handle_size: Option<StringOrNumber>,
    /// Slider
    handle_style: Option<StrokeStyle>,
    /// Slider (since v5.6.0)
    handle_label: Option<HandleLabel>,
    /// Slider (since v5.0.0)
    move_handle_icon: Option<String>,
    /// Slider (since v5.0.0)
    move_handle_size: Option<f64>,
    /// Slider (since v5.0.0)
    move_handle_style: Option<StrokeStyle>,
    /// Slider
    label_precision: Option<StringOrNumber>,
    /// Slider
    label_formatter: Option<JsFunction>,
    /// Slider
    show_detail: Option<bool>,
    /// Slider
    show_data_shadow: Option<bool>,
    /// Slider
    brush_select: Option<bool>,
    /// Slider (since v5.0.0)
    brush_style: Option<StrokeStyle>,
    /// Slider (since v5.0.0)
    emphasis: Option<Emphasis>,
    /// Slider
    zlevel: Option<f64>,
    /// Slider
    z: Option<f64>,
    /// Inside
    zoom_on_mouse_wheel: Option<MouseZoom>,
    /// Inside
    move_on_mouse_move: Option<MouseZoom>,
    /// Inside
    move_on_mouse_wheel: Option<MouseZoom>,
    /// Inside
    prevent_default_mouse_move: Option<bool>,
}
