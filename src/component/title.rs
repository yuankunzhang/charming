use serde::Serialize;

use crate::utility::{color::Color, text_style::TextStyle};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subtext: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_style: Option<TextStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<f64>,
}

impl Title {
    pub fn new() -> Self {
        Self {
            text: None,
            subtext: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            text_style: None,
            border_color: None,
            border_width: None,
        }
    }

    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn subtext<S: Into<String>>(mut self, subtext: S) -> Self {
        self.subtext = Some(subtext.into());
        self
    }

    pub fn left<S: Into<String>>(mut self, left: S) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<S: Into<String>>(mut self, top: S) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<S: Into<String>>(mut self, right: S) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<S: Into<String>>(mut self, bottom: S) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn text_style(mut self, text_style: TextStyle) -> Self {
        self.text_style = Some(text_style);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn border_width<F: Into<f64>>(mut self, border_width: F) -> Self {
        self.border_width = Some(border_width.into());
        self
    }
}
