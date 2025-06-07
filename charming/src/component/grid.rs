use crate::{
    datatype::CompositeValue,
    element::{Color, Padding, TextStyle, Trigger},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GridTooltip {
    /// Whether to show the tooltip component.
    show: Option<bool>,
    /// Type of triggering.
    trigger: Option<Trigger>,
    /// The position of the tooltip's floating layer, which would follow the
    /// position of mouse by default.
    #[charming_skip_setter]
    position: Option<(String, String)>,
    /// The content formatter of tooltip's floating layer.
    formatter: Option<String>,
    /// The value formatter of tooltip's floating layer.
    value_formatter: Option<String>,
    /// The background color of tooltip's floating layer.
    background_color: Option<Color>,
    /// The border color of tooltip's floating layer.
    border_color: Option<Color>,
    /// The border width of tooltip's floating layer.
    border_width: Option<f64>,
    /// The floating layer of tooltip space around content.
    padding: Option<Padding>,
    /// Text style of tooltip's floating layer.
    text_style: Option<TextStyle>,
    /// Extra CSS style for the tooltip's floating layer.
    extra_css_text: Option<String>,
}

impl GridTooltip {
    pub fn position<S: Into<String>>(mut self, position: (S, S)) -> Self {
        self.position = Some((position.0.into(), position.1.into()));
        self
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    /// Component ID.
    id: Option<String>,
    /// Whether to show the grid in rectangular coordinate.
    show: Option<bool>,
    /// The `zlevel` value of all graphical elements in.
    zlevel: Option<f64>,
    /// The `z` value of all graphical elements in.
    z: Option<f64>,
    /// Distance between grid component and the left side of the container.
    left: Option<CompositeValue>,
    /// Distance between grid component and the top side of the container.
    top: Option<CompositeValue>,
    /// Distance between grid component and the right side of the container.
    right: Option<CompositeValue>,
    /// Distance between grid component and the bottom side of the container.
    bottom: Option<CompositeValue>,
    /// Width of grid component.
    width: Option<CompositeValue>,
    /// Height of grid component.
    height: Option<CompositeValue>,
    /// Whether the grid region contains axis tick label of axis.
    contain_label: Option<bool>,
    /// Background color of grid, which is transparent by default.
    background_color: Option<Color>,
    /// Border color of grid.
    border_color: Option<Color>,
    /// Border width of grid.
    border_width: Option<f64>,
    /// Size of shadow blue.
    shadow_blur: Option<f64>,
    /// Shadow color.
    shadow_color: Option<Color>,
    /// Offset distance on the horizontal direction of shadow.
    shadow_offset_x: Option<f64>,
    /// Offset distance on the vertical direction of shadow.
    shadow_offset_y: Option<f64>,
    /// Tooltip settings in the grid.
    tooltip: Option<GridTooltip>,
}
