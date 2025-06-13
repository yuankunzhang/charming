use crate::element::{AxisPointer, Color, Formatter, Padding};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOn {
    Mousemove,
    Click,
    #[serde(rename = "mousemove|click")]
    MousemoveAndClick,
    None,
}

/// Types of triggering.
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    Item,
    Axis,
    None,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {
    trigger: Option<Trigger>,
    trigger_on: Option<TriggerOn>,
    axis_pointer: Option<AxisPointer>,
    formatter: Option<Formatter>,
    value_formatter: Option<Formatter>,
    position: Option<String>,
    padding: Option<Padding>,
    background_color: Option<Color>,
    border_color: Option<Color>,
    border_width: Option<f64>,
}
