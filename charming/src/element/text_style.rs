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
    color: Option<Color>,
    font_style: Option<FontStyle>,
    font_weight: Option<FontWeight>,
    font_family: Option<FontFamily>,
    font_size: Option<f64>,
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
