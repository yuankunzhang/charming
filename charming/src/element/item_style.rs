use super::{border_type::BorderType, color::Color};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemStyle {
    color: Option<Color>,
    border_color: Option<Color>,
    border_width: Option<f64>,
    border_radius: Option<f64>,
    border_type: Option<BorderType>,
    opacity: Option<f64>,
    shadow_color: Option<Color>,
    shadow_blur: Option<f64>,
    shadow_offset_x: Option<f64>,
    shadow_offset_y: Option<f64>,
}

impl From<Color> for ItemStyle {
    fn from(color: Color) -> Self {
        Self::new().color(color)
    }
}
