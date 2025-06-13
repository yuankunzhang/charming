use crate::{
    datatype::DataFrame,
    element::{
        ColorBy, CoordinateSystem, Cursor, Emphasis, ItemStyle, Label, LabelLayout, LabelLine,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PictorialBar {
    #[serde(rename = "type")]
    #[charming_type = "pictorialBar"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    legend_hover_link: Option<bool>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    cursor: Option<Cursor>,
    label: Option<Label>,
    label_line: Option<LabelLine>,
    label_layout: Option<LabelLayout>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    symbol_clip: Option<bool>,
    symbol_bounding_data: Option<f64>,
    #[charming_set_vec]
    data: Vec<DataFrame>,
}
