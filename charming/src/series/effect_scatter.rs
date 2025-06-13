use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        Color, ColorBy, CoordinateSystem, Emphasis, ItemStyle, Label, LabelLayout, LabelLine,
        Symbol, Tooltip,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    Ripple,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ShowEffectOn {
    Render,
    Emphasis,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RippleEffectBrushType {
    Fill,
    Stroke,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RippleEffect {
    color: Option<Color>,
    number: Option<f64>,
    period: Option<f64>,
    scale: Option<f64>,
    brush_type: Option<RippleEffectBrushType>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EffectScatter {
    #[serde(rename = "type")]
    #[charming_type = "effectScatter"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    legend_hover_link: Option<bool>,
    effect_type: Option<EffectType>,
    show_effect_on: Option<ShowEffectOn>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    polar_index: Option<f64>,
    geo_index: Option<f64>,
    calendar_index: Option<f64>,
    symbol: Option<Symbol>,
    symbol_size: Option<f64>,
    symbol_rotate: Option<f64>,
    symbol_keep_aspect: Option<bool>,
    symbol_offset: Option<(String, String)>,
    label: Option<Label>,
    label_line: Option<LabelLine>,
    label_layout: Option<LabelLayout>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
