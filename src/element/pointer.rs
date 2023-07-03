use serde::Serialize;

use super::{icon::Icon, item_style::ItemStyle};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pointer {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show_above: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset_center: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    keep_aspect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,
}

impl Pointer {
    pub fn new() -> Self {
        Self {
            show: None,
            show_above: None,
            icon: None,
            offset_center: None,
            length: None,
            width: None,
            keep_aspect: None,
            item_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn show_above(mut self, show_above: bool) -> Self {
        self.show_above = Some(show_above);
        self
    }

    pub fn icon<S: Into<Icon>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn offset_center<S: Into<String>>(mut self, offset_center: (S, S)) -> Self {
        self.offset_center = Some((offset_center.0.into(), offset_center.1.into()));
        self
    }

    pub fn length<S: Into<String>>(mut self, length: S) -> Self {
        self.length = Some(length.into());
        self
    }

    pub fn width<F: Into<f64>>(mut self, width: F) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn keep_aspect(mut self, keep_aspect: bool) -> Self {
        self.keep_aspect = Some(keep_aspect);
        self
    }

    pub fn item_style<S: Into<ItemStyle>>(mut self, item_style: S) -> Self {
        self.item_style = Some(item_style.into());
        self
    }
}
