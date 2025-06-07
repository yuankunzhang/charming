use crate::{
    datatype::CompositeValue,
    element::{Color, DataBackground, Orient, TextStyle},
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
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    x_axis_index: Option<CompositeValue>,
    y_axis_index: Option<CompositeValue>,
    disabled: Option<bool>,
    radius_axis_index: Option<f64>,
    angle_axis_index: Option<f64>,
    filter_mode: Option<FilterMode>,
    text_style: Option<TextStyle>,
    handle_icon: Option<String>,
    brush_select: Option<bool>,
}
