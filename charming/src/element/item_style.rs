use serde::{Serialize, Deserialize};
use super::{border_type::BorderType, color::Color};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_type: Option<BorderType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_blur: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_y: Option<f64>,
}

impl ItemStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            border_color: None,
            border_width: None,
            border_radius: None,
            border_type: None,
            opacity: None,
            shadow_color: None,
            shadow_blur: None,
            shadow_offset_x: None,
            shadow_offset_y: None,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn border_color<C: Into<Color>>(mut self, border_color: C) -> Self {
        self.border_color = Some(border_color.into());
        self
    }

    pub fn border_width<F: Into<f64>>(mut self, border_width: F) -> Self {
        self.border_width = Some(border_width.into());
        self
    }

    pub fn border_radius<F: Into<f64>>(mut self, border_radius: F) -> Self {
        self.border_radius = Some(border_radius.into());
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = Some(border_type);
        self
    }

    pub fn opacity<F: Into<f64>>(mut self, opacity: F) -> Self {
        self.opacity = Some(opacity.into());
        self
    }

    pub fn shadow_color<C: Into<Color>>(mut self, shadow_color: C) -> Self {
        self.shadow_color = Some(shadow_color.into());
        self
    }

    pub fn shadow_blur<F: Into<f64>>(mut self, shadow_blur: F) -> Self {
        self.shadow_blur = Some(shadow_blur.into());
        self
    }

    pub fn shadow_offset_x<F: Into<f64>>(mut self, shadow_offset_x: F) -> Self {
        self.shadow_offset_x = Some(shadow_offset_x.into());
        self
    }

    pub fn shadow_offset_y<F: Into<f64>>(mut self, shadow_offset_y: F) -> Self {
        self.shadow_offset_y = Some(shadow_offset_y.into());
        self
    }
}

impl From<Color> for ItemStyle {
    fn from(color: Color) -> Self {
        Self::new().color(color)
    }
}
