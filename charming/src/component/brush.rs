use crate::{
    component::BrushType,
    datatype::CompositeValue,
    element::{Color, ItemStyle},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum BrushMode {
    Single,
    Multiple,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum BrushSelectionType {
    Rect,
    Polygon,
    LineX,
    LineY,
    Auto,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(untagged)]
pub enum BrushSelection {
    Type(BrushSelectionType),
    Disabled(bool),
}

impl From<BrushSelectionType> for BrushSelection {
    fn from(value: BrushSelectionType) -> Self {
        Self::Type(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum BrushThrottleType {
    FixRate,
    Debounce,
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrushVisual {
    color: Option<Color>,
    color_alpha: Option<f64>,
    opacity: Option<f64>,
    symbol_size: Option<f64>,
}

/// Chart-level brushing; `Brush` remains the toolbox feature configuration.
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrushComponent {
    id: Option<String>,
    #[charming_set_vec]
    toolbox: Vec<BrushType>,
    brush_link: Option<CompositeValue>,
    series_index: Option<CompositeValue>,
    geo_index: Option<CompositeValue>,
    geo_id: Option<CompositeValue>,
    x_axis_index: Option<CompositeValue>,
    x_axis_id: Option<CompositeValue>,
    y_axis_index: Option<CompositeValue>,
    y_axis_id: Option<CompositeValue>,
    brush_type: Option<BrushSelection>,
    brush_mode: Option<BrushMode>,
    brush_style: Option<ItemStyle>,
    transformable: Option<bool>,
    remove_on_click: Option<bool>,
    throttle_type: Option<BrushThrottleType>,
    throttle_delay: Option<f64>,
    in_brush: Option<BrushVisual>,
    out_of_brush: Option<BrushVisual>,
}
