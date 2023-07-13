use serde::Serialize;

use crate::{
    datatype::CompositeValue,
    element::{Color, LinkTarget, Padding, TextAlign, TextStyle, TextVerticalAlign},
};

/// Title component, including main title and subtitle.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    /// Component ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// Whether to show the title component.
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    /// The main title text, supporting for `\n` for newlines.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    /// The hyper link of main title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<String>,

    /// Open the hyper link of main title in specified target.
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<LinkTarget>,

    /// The text style of main title.
    #[serde(skip_serializing_if = "Option::is_none")]
    text_style: Option<TextStyle>,

    /// The sub title text, supporting for `\n` for newlines.
    #[serde(skip_serializing_if = "Option::is_none")]
    subtext: Option<String>,

    /// The hyper link of sub title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    sublink: Option<String>,

    /// Open the hyper link of sub title in specified target.
    #[serde(skip_serializing_if = "Option::is_none")]
    subtarget: Option<LinkTarget>,

    /// The text style of sub title.
    #[serde(skip_serializing_if = "Option::is_none")]
    subtext_style: Option<TextStyle>,

    /// The horizontal align of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    text_align: Option<TextAlign>,

    /// The vertical align of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    text_vertical_align: Option<TextVerticalAlign>,

    /// Title padding, the unit is px.
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<Padding>,

    /// The gap between the main title and the sub title, the unit is px.
    #[serde(skip_serializing_if = "Option::is_none")]
    item_gap: Option<f64>,

    /// The `zlevel` value of all graphical elements in the title.
    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel: Option<f64>,

    /// The `z` value of all graphical elements in the title.
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<f64>,

    /// Distance between title component and the left side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<CompositeValue>,

    /// Distance between title component and the top side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<CompositeValue>,

    /// Distance between title component and the right side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<CompositeValue>,

    /// Distance between title component and the bottom side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<CompositeValue>,

    /// Background color of title, default to be transparent.
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    /// Border color of title.
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    /// Border width of title.
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<f64>,

    /// Border radius of title.
    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<f64>,

    /// Shadow color of title.
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_color: Option<Color>,

    /// Size of shadow blur.
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_blur: Option<f64>,

    /// Offset distance on the horizontal direction of shadow.
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_x: Option<f64>,

    /// Offset distance on the vertical direction of shadow.
    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_y: Option<f64>,
}

impl Title {
    pub fn new() -> Self {
        Self {
            id: None,
            show: None,
            text: None,
            link: None,
            target: None,
            text_style: None,
            subtext: None,
            sublink: None,
            subtarget: None,
            subtext_style: None,
            text_align: None,
            text_vertical_align: None,
            padding: None,
            item_gap: None,
            zlevel: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            background_color: None,
            border_color: None,
            border_width: None,
            border_radius: None,
            shadow_color: None,
            shadow_blur: None,
            shadow_offset_x: None,
            shadow_offset_y: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn link<S: Into<String>>(mut self, link: S) -> Self {
        self.link = Some(link.into());
        self
    }

    pub fn target<T: Into<LinkTarget>>(mut self, target: T) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn text_style<S: Into<TextStyle>>(mut self, text_style: S) -> Self {
        self.text_style = Some(text_style.into());
        self
    }

    pub fn subtext<S: Into<String>>(mut self, subtext: S) -> Self {
        self.subtext = Some(subtext.into());
        self
    }

    pub fn sublink<S: Into<String>>(mut self, sublink: S) -> Self {
        self.sublink = Some(sublink.into());
        self
    }

    pub fn subtarget<T: Into<LinkTarget>>(mut self, subtarget: T) -> Self {
        self.subtarget = Some(subtarget.into());
        self
    }

    pub fn subtext_style<S: Into<TextStyle>>(mut self, subtext_style: S) -> Self {
        self.subtext_style = Some(subtext_style.into());
        self
    }

    pub fn text_align<A: Into<TextAlign>>(mut self, text_align: A) -> Self {
        self.text_align = Some(text_align.into());
        self
    }

    pub fn text_vertical_align<A: Into<TextVerticalAlign>>(
        mut self,
        text_vertical_align: A,
    ) -> Self {
        self.text_vertical_align = Some(text_vertical_align.into());
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn item_gap<F: Into<f64>>(mut self, item_gap: F) -> Self {
        self.item_gap = Some(item_gap.into());
        self
    }

    pub fn zlevel<F: Into<f64>>(mut self, zlevel: F) -> Self {
        self.zlevel = Some(zlevel.into());
        self
    }

    pub fn z<F: Into<f64>>(mut self, z: F) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn background_color<C: Into<Color>>(mut self, background_color: C) -> Self {
        self.background_color = Some(background_color.into());
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
