use serde::Serialize;

use super::{blur::Blur, emphasis::Emphasis, item_style::ItemStyle, label::Label};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkArea {
    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    blur: Option<Blur>,
}
