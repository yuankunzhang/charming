use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        font_settings::{FontFamily, FontStyle, FontWeight},
        Anchor, AxisLabel, AxisLine, AxisTick, Color, ColorBy, Formatter, ItemStyle, Pointer,
        SplitLine, Tooltip,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GaugeDetail {
    show: Option<bool>,
    color: Option<Color>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_family: Option<FontFamily>,
    font_size: Option<f64>,
    precision: Option<f64>,
    value_animation: Option<bool>,
    formatter: Option<Formatter>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GaugeTitle {
    show: Option<bool>,
    #[charming_skip_setter]
    offset_center: Option<(String, String)>,
}

impl GaugeTitle {
    pub fn offset_center<S: Into<String>>(mut self, offset_center: (S, S)) -> Self {
        self.offset_center = Some((offset_center.0.into(), offset_center.1.into()));
        self
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GaugeProgress {
    show: Option<bool>,
    overlap: Option<bool>,
    width: Option<f64>,
    round_cap: Option<bool>,
    clip: Option<bool>,
    item_style: Option<ItemStyle>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gauge {
    #[serde(rename = "type")]
    #[charming_type = "gauge"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    zlevel: Option<f64>,
    z: Option<f64>,
    #[charming_skip_setter]
    center: Option<(String, String)>,
    legend_hover_link: Option<bool>,
    start_angle: Option<f64>,
    end_angle: Option<f64>,
    clockwise: Option<bool>,
    min: Option<f64>,
    max: Option<f64>,
    split_number: Option<f64>,
    radius: Option<String>,
    progress: Option<GaugeProgress>,
    axis_line: Option<AxisLine>,
    axis_tick: Option<AxisTick>,
    axis_label: Option<AxisLabel>,
    split_line: Option<SplitLine>,
    pointer: Option<Pointer>,
    anchor: Option<Anchor>,
    detail: Option<GaugeDetail>,
    title: Option<GaugeTitle>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}

impl Gauge {
    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }
}
