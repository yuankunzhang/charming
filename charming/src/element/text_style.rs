use super::{
    color::Color,
    font_settings::{FontFamily, FontStyle, FontWeight},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    opacity: Option<f64>,
    text_border_color: Option<Color>,
    text_border_width: Option<f64>,
    text_border_type: Option<super::BorderType>,
    text_border_dash_offset: Option<f64>,
    text_shadow_color: Option<Color>,
    text_shadow_blur: Option<f64>,
    text_shadow_offset_x: Option<f64>,
    text_shadow_offset_y: Option<f64>,
    width: Option<crate::datatype::StringOrNumber>,
    height: Option<f64>,
    background_color: Option<Color>,
    border_color: Option<Color>,
    border_width: Option<f64>,
    rich_inherit_plain_label: Option<bool>,
    min_margin: Option<f64>,
    text_margin: Option<crate::datatype::NumberOrArray>,
    line_overflow: Option<crate::element::LineOverflow>,
    overflow: Option<crate::element::TextOverflow>,
    ellipsis: Option<String>,
    color: Option<Color>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_family: Option<FontFamily>,
    font_size: Option<crate::datatype::StringOrNumber>,
    line_height: Option<f64>,
    align: Option<String>,
    #[charming_skip_setter]
    padding: Option<[f64; 4]>,
}

impl TextStyle {
    pub fn padding<F: Into<f64> + Copy>(mut self, padding: [F; 4]) -> Self {
        self.padding = Some([
            padding[0].into(),
            padding[1].into(),
            padding[2].into(),
            padding[3].into(),
        ]);
        self
    }

    pub fn padding_all<F: Into<f64> + Copy>(mut self, padding: F) -> Self {
        self.padding = Some([
            padding.into(),
            padding.into(),
            padding.into(),
            padding.into(),
        ]);
        self
    }

    pub fn padding_pair<F: Into<f64> + Copy>(mut self, padding: [F; 2]) -> Self {
        self.padding = Some([
            padding[0].into(),
            padding[1].into(),
            padding[0].into(),
            padding[1].into(),
        ]);
        self
    }
}
