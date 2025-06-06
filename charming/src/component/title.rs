use crate::{
    datatype::CompositeValue,
    element::{Color, LinkTarget, Padding, TextAlign, TextStyle, TextVerticalAlign},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// Title component, including main title and subtitle.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    /// Component ID.
    id: Option<String>,
    /// Whether to show the title component.
    show: Option<bool>,
    /// The main title text, supporting for `\n` for newlines.
    text: Option<String>,
    /// The hyper link of main title text.
    link: Option<String>,
    /// Open the hyper link of main title in specified target.
    target: Option<LinkTarget>,
    /// The text style of main title.
    text_style: Option<TextStyle>,
    /// The sub title text, supporting for `\n` for newlines.
    subtext: Option<String>,
    /// The hyper link of sub title text.
    sublink: Option<String>,
    /// Open the hyper link of sub title in specified target.
    subtarget: Option<LinkTarget>,
    /// The text style of sub title.
    subtext_style: Option<TextStyle>,
    /// The horizontal align of the component.
    text_align: Option<TextAlign>,
    /// The vertical align of the component.
    text_vertical_align: Option<TextVerticalAlign>,
    /// Title padding, the unit is px.
    padding: Option<Padding>,
    /// The gap between the main title and the sub title, the unit is px.
    item_gap: Option<f64>,
    /// The `zlevel` value of all graphical elements in the title.
    zlevel: Option<f64>,
    /// The `z` value of all graphical elements in the title.
    z: Option<f64>,
    /// Distance between title component and the left side of the container.
    left: Option<CompositeValue>,
    /// Distance between title component and the top side of the container.
    top: Option<CompositeValue>,
    /// Distance between title component and the right side of the container.
    right: Option<CompositeValue>,
    /// Distance between title component and the bottom side of the container.
    bottom: Option<CompositeValue>,
    /// Background color of title, default to be transparent.
    background_color: Option<Color>,
    /// Border color of title.
    border_color: Option<Color>,
    /// Border width of title.
    border_width: Option<f64>,
    /// Border radius of title.
    border_radius: Option<f64>,
    /// Shadow color of title.
    shadow_color: Option<Color>,
    /// Size of shadow blur.
    shadow_blur: Option<f64>,
    /// Offset distance on the horizontal direction of shadow.
    shadow_offset_x: Option<f64>,
    /// Offset distance on the vertical direction of shadow.
    shadow_offset_y: Option<f64>,
}
