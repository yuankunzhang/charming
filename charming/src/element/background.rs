use super::{area_style::AreaStyle, border_type::BorderType, color::Color, line_style::LineStyle};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundStyle {
    color: Option<Color>,
    border_color: Option<Color>,
    border_width: Option<f64>,
    border_type: Option<BorderType>,
    border_radius: Option<f64>,
    opacity: Option<f64>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataBackground {
    line_style: Option<LineStyle>,
    area_style: Option<AreaStyle>,
}
