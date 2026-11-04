use super::{
    Formatter,
    color::Color,
    font_settings::{FontFamily, FontStyle, FontWeight},
    line_style::LineStyle,
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum LabelPosition {
    Top,
    Left,
    Right,
    Bottom,
    Inside,
    InsideLeft,
    InsideRight,
    InsideTop,
    InsideBottom,
    InsideTopLeft,
    InsideBottomLeft,
    InsideTopRight,
    InsideBottomRight,
    InsideStartTop,
    InsideStartBottom,
    InsideMiddleTop,
    InsideMiddleBottom,
    InsideEndTop,
    InsideEndBottom,
    Start,
    Outside,
    Middle,
    Center,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LabelAlign {
    Left,
    Center,
    Right,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LabelVerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    show: Option<bool>,
    position: Option<LabelPosition>,
    distance: Option<f64>,
    rotate: Option<String>,
    #[charming_skip_setter]
    offset: Option<(f64, f64)>,
    formatter: Option<Formatter>,
    color: Option<Color>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_family: Option<FontFamily>,
    font_size: Option<f64>,
    #[charming_skip_setter]
    padding: Option<(f64, f64, f64, f64)>,
    align: Option<LabelAlign>,
    vertical_align: Option<LabelVerticalAlign>,
    silent: Option<bool>,
    background_color: Option<Color>,
    border_color: Option<Color>,
    border_width: Option<f64>,
    shadow_blur: Option<f64>,
    shadow_offset_x: Option<f64>,
    shadow_offset_y: Option<f64>,

    height: Option<f64>,
    width: Option<f64>,
}

impl Label {
    pub fn offset<F: Into<f64>>(mut self, offset: (F, F)) -> Self {
        self.offset = Some((offset.0.into(), offset.1.into()));
        self
    }
    pub fn padding<F: Into<f64>>(mut self, padding: (F, F, F, F)) -> Self {
        self.padding = Some((
            padding.0.into(),
            padding.1.into(),
            padding.2.into(),
            padding.3.into(),
        ));
        self
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LabelLine {
    show: Option<bool>,
    show_above: Option<bool>,
    length: Option<f64>,
    smooth: Option<bool>,
    min_turn_angle: Option<f64>,
    line_style: Option<LineStyle>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LabelLayout {
    hide_overlap: Option<bool>,
    overlap: Option<String>,
    rotate: Option<f64>,
}
