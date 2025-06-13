use crate::{
    datatype::CompositeValue,
    element::{
        font_settings::{FontFamily, FontStyle, FontWeight},
        AxisLabel, AxisLine, AxisTick, Color, Formatter, Padding, Shape, SplitArea, SplitLine,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// Name options for radar indicators.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadarAxisName {
    /// Whether to display the indicator's name.
    show: Option<bool>,
    /// Formatter of the indicator's name.
    formatter: Option<Formatter>,
    color: Option<Color>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_family: Option<FontFamily>,
    font_size: Option<f64>,
    line_height: Option<f64>,
    background_color: Option<Color>,
    border_color: Option<Color>,
    border_width: Option<f64>,
    border_type: Option<String>,
    border_dash_offset: Option<f64>,
    border_radius: Option<CompositeValue>,
    padding: Option<Padding>,
    shadow_color: Option<Color>,
    shadow_blur: Option<f64>,
    shadow_offset_x: Option<f64>,
    shadow_offset_y: Option<f64>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    text_border_color: Option<Color>,
    text_border_width: Option<f64>,
    text_border_type: Option<String>,
    text_border_dash_offset: Option<f64>,
    text_shadow_color: Option<Color>,
    text_shadow_blur: Option<f64>,
    text_shadow_offset_x: Option<f64>,
    text_shadow_offset_y: Option<f64>,
    overflow: Option<String>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadarIndicator {
    name: Option<String>,
    max: Option<f64>,
    min: Option<f64>,
    color: Option<Color>,
}

impl From<(&str, f64, f64)> for RadarIndicator {
    fn from((name, min, max): (&str, f64, f64)) -> Self {
        Self {
            name: Some(name.into()),
            min: Some(min),
            max: Some(max),
            color: None,
        }
    }
}

impl From<(&str, i64, i64)> for RadarIndicator {
    fn from((name, min, max): (&str, i64, i64)) -> Self {
        Self {
            name: Some(name.into()),
            min: Some(min as f64),
            max: Some(max as f64),
            color: None,
        }
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadarCoordinate {
    id: Option<String>,
    /// The `zlevel` value of all graphical elements in.
    zlevel: Option<f64>,
    /// The `z` value of all graphical elements in.
    z: Option<f64>,
    center: Option<CompositeValue>,
    radius: Option<CompositeValue>,
    start_angle: Option<f64>,
    axis_name: Option<RadarAxisName>,
    name_gap: Option<f64>,
    split_number: Option<f64>,
    shape: Option<Shape>,
    scale: Option<bool>,
    axis_line: Option<AxisLine>,
    axis_tick: Option<AxisTick>,
    axis_label: Option<AxisLabel>,
    split_line: Option<SplitLine>,
    split_area: Option<SplitArea>,
    #[charming_set_vec]
    indicator: Vec<RadarIndicator>,
}
