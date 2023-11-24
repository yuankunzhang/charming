use serde::Serialize;

use super::color::Color;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_style: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_weight: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_height: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<[f64; 4]>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            font_style: None,
            font_weight: None,
            font_family: None,
            font_size: None,
            line_height: None,
            align: None,
            padding: None,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn font_style<S: Into<String>>(mut self, font_style: S) -> Self {
        self.font_style = Some(font_style.into());
        self
    }

    pub fn font_weight<S: Into<String>>(mut self, font_weight: S) -> Self {
        self.font_weight = Some(font_weight.into());
        self
    }

    pub fn font_family<S: Into<String>>(mut self, font_family: S) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    pub fn font_size<F: Into<f64>>(mut self, font_size: F) -> Self {
        self.font_size = Some(font_size.into());
        self
    }

    pub fn line_height<F: Into<f64>>(mut self, line_height: F) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn align<S: Into<String>>(mut self, align: S) -> Self {
        self.align = Some(align.into());
        self
    }

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
