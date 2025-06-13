use crate::{
    datatype::CompositeValue,
    element::{Color, Label, Symbol},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/**
A single decal pattern.
 */
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecalItem {
    /// The symbol type of the decal.
    symbol: Option<Symbol>,
    /// The size of symbol relative to decal, ranging from 0 to 1.
    symbol_size: Option<f64>,
    /// Whether to keep the aspect ratio of the pattern.
    symbol_keep_aspect: Option<bool>,
    ///The color of the decal pattern. it is recommended to use a translucent
    /// color, which can be superimposed on the color of the series itself.
    color: Option<Color>,
    /// The background color of the decal will be over the color of the series
    /// itself, under the decal pattern.
    background_color: Option<Color>,
    /// Controls the horizontal pattern.
    dash_array_x: Option<CompositeValue>,
    /// Controls the vertical pattern.
    dash_array_y: Option<CompositeValue>,
    /// The overall rotation angle (in radians) of the pattern.
    rotation: Option<f64>,
    /// The upper limit of the width of the generated pattern before it is
    /// duplicated.
    max_tile_width: Option<f64>,
    /// The upper limit of the height of the generated pattern before it is
    /// duplicated.
    max_tile_height: Option<f64>,
}

/**
Decal patterns to be applied to series data.
 */
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
pub struct Decal {
    /// Whether to show decal patterns. If `decals` is not set, this option is
    /// used to enable the default decal pattern.
    show: Option<bool>,
    /// The style of decal patterns. If multiple items are set, then each item
    /// in the array will have one style and the data will be looped through
    /// the array in order.
    #[charming_set_vec]
    decals: Vec<DecalItem>,
}

/**
The WAI-ARIA (Accessible Rich Internet Applications Suite) is a W3C standard
that dedicates to make web content and web applications more accessible.

It is turned off by default, and needs to be turned on by setting `enabled` to
`true`.

Here's a simple example that enables default decal pattern on a bar chart:

```rust
use charming::{
    component::{Aria, Axis, Decal},
    element::AxisType,
    series::Bar,
    Chart,
};

Chart::new()
    .x_axis(
        Axis::new()
            .type_(AxisType::Category)
            .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
    )
    .y_axis(Axis::new().type_(AxisType::Value))
    .aria(Aria::new().enabled(true).decal(Decal::new().show(true)))
    .series(Bar::new().data(vec![120, 200, 150, 80, 70, 110, 130]))
    .series(Bar::new().data(vec![20, 40, 90, 40, 30, 70, 120]))
    .series(Bar::new().data(vec![140, 230, 120, 50, 30, 150, 120]));
```
 */
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
pub struct Aria {
    /// Whether to enable WAI-ARIA.
    enabled: Option<bool>,
    /// If `enabled` is set to `true`, `label` is enabled by default. When
    /// enabled, the description of the chart will be automatically and
    /// intelligently generated based on the chart, data title, etc. Users can
    /// also modify the description through `label`.
    label: Option<Label>,
    /// Decal patterns are added to series data as an additional hint other
    /// than colors to help differentiate the data.
    decal: Option<Decal>,
}
