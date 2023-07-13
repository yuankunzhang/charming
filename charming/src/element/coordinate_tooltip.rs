use serde::Serialize;

use crate::datatype::CompositeValue;

use super::{Color, Formatter, Padding, TextStyle, Trigger};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateTooltip {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Trigger>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<Formatter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value_formatter: Option<Formatter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<Padding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_style: Option<TextStyle>,
}
