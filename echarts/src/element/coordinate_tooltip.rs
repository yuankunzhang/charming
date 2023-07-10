use serde::Serialize;

use super::{
    Color, ContentFormatter, PaddingProperty, PositionProperty, TextStyle, TooltipTrigger,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoordinateTooltip {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<TooltipTrigger>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<PositionProperty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<ContentFormatter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value_formatter: Option<ContentFormatter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<PaddingProperty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_style: Option<TextStyle>,
}
