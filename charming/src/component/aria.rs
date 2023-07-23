use serde::Serialize;

use crate::{
    datatype::CompositeValue,
    element::{Color, Label, Symbol},
};

/**
A single decal pattern.
 */
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecalItem {
    /// The symbol type of the decal.
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    /// The size of symbol relative to decal, ranging from 0 to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f64>,

    /// Whether to keep the aspect ratio of the pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_keep_aspect: Option<bool>,

    ///The color of the decal pattern. it is recommended to use a translucent
    /// color, which can be superimposed on the color of the series itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    /// The background color of the decal will be over the color of the series
    /// itself, under the decal pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    /// Controls the horizontal pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    dash_array_x: Option<CompositeValue>,

    /// Controls the vertical pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    dash_array_y: Option<CompositeValue>,

    /// The overall rotation angle (in radians) of the pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    rotation: Option<f64>,

    /// The upper limit of the width of the generated pattern before it is
    /// duplicated.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tile_width: Option<f64>,

    /// The upper limit of the height of the generated pattern before it is
    /// duplicated.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tile_height: Option<f64>,
}

impl DecalItem {
    pub fn new() -> DecalItem {
        DecalItem {
            symbol: None,
            symbol_size: None,
            symbol_keep_aspect: None,
            color: None,
            background_color: None,
            dash_array_x: None,
            dash_array_y: None,
            rotation: None,
            max_tile_width: None,
            max_tile_height: None,
        }
    }

    pub fn symbol<S: Into<Symbol>>(mut self, symbol: S) -> DecalItem {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn symbol_size<F: Into<f64>>(mut self, symbol_size: F) -> DecalItem {
        self.symbol_size = Some(symbol_size.into());
        self
    }

    pub fn symbol_keep_aspect(mut self, symbol_keep_aspect: bool) -> DecalItem {
        self.symbol_keep_aspect = Some(symbol_keep_aspect);
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> DecalItem {
        self.color = Some(color.into());
        self
    }

    pub fn background_color<C: Into<Color>>(mut self, background_color: C) -> DecalItem {
        self.background_color = Some(background_color.into());
        self
    }

    pub fn dash_array_x<F: Into<CompositeValue>>(mut self, dash_array_x: F) -> DecalItem {
        self.dash_array_x = Some(dash_array_x.into());
        self
    }

    pub fn dash_array_y<F: Into<CompositeValue>>(mut self, dash_array_y: F) -> DecalItem {
        self.dash_array_y = Some(dash_array_y.into());
        self
    }

    pub fn rotation<F: Into<f64>>(mut self, rotation: F) -> DecalItem {
        self.rotation = Some(rotation.into());
        self
    }

    pub fn max_tile_width<F: Into<f64>>(mut self, max_tile_width: F) -> DecalItem {
        self.max_tile_width = Some(max_tile_width.into());
        self
    }

    pub fn max_tile_height<F: Into<f64>>(mut self, max_tile_height: F) -> DecalItem {
        self.max_tile_height = Some(max_tile_height.into());
        self
    }
}

/**
Decal patterns to be applied to series data.
 */
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Decal {
    /// Whether to show decal patterns. If `decals` is not set, this option is
    /// used to enable the default decal pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    /// The style of decal patterns. If multiple items are set, then each item
    /// in the array will have one style and the data will be looped through
    /// the array in order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    decals: Vec<DecalItem>,
}

impl Decal {
    pub fn new() -> Decal {
        Decal {
            show: None,
            decals: vec![],
        }
    }

    pub fn show(mut self, show: bool) -> Decal {
        self.show = Some(show);
        self
    }

    pub fn decals<D: Into<DecalItem>>(mut self, decals: Vec<D>) -> Decal {
        self.decals = decals.into_iter().map(|d| d.into()).collect();
        self
    }
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
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Aria {
    /// Whether to enable WAI-ARIA.
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    /// If `enabled` is set to `true`, `label` is enabled by default. When
    /// enabled, the description of the chart will be automatically and
    /// intelligently generated based on the chart, data title, etc. Users can
    /// also modify the description through `label`.
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    /// Decal patterns are added to series data as an additional hint other
    /// than colors to help differentiate the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    decal: Option<Decal>,
}

impl Aria {
    pub fn new() -> Aria {
        Aria {
            enabled: None,
            label: None,
            decal: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Aria {
        self.enabled = Some(enabled);
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Aria {
        self.label = Some(label.into());
        self
    }

    pub fn decal<D: Into<Decal>>(mut self, decal: D) -> Aria {
        self.decal = Some(decal.into());
        self
    }
}
