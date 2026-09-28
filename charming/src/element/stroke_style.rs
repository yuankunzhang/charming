use crate::element::{BorderCap, BorderJoin, BorderType};

use super::color::Color;
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StrokeStyle {
    color: Option<Color>,
    border_color: Option<Color>,
    border_width: Option<f64>,
    border_type: Option<BorderType>,
    border_dash_offset: Option<f64>,
    border_cap: Option<BorderCap>,
    border_join: Option<BorderJoin>,
    border_miter_limit: Option<f64>,
    shadow_blur: Option<f64>,
    shadow_color: Option<Color>,
    shadow_offset_x: Option<f64>,
    shadow_offset_y: Option<f64>,
    opacity: Option<f64>,
}
