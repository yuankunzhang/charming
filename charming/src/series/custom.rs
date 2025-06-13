use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint, Dimension},
    element::{
        ColorBy, CoordinateSystem, DimensionEncode, ItemStyle, LabelLayout, LabelLine, RawString,
        Tooltip,
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
pub struct Custom {
    #[serde(rename = "type")]
    #[charming_type = "custom"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    legend_hover_link: Option<bool>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<CompositeValue>,
    y_axis_index: Option<CompositeValue>,
    polar_index: Option<CompositeValue>,
    geo_index: Option<CompositeValue>,
    calendar_index: Option<CompositeValue>,
    render_item: Option<RawString>,
    item_style: Option<ItemStyle>,
    label_line: Option<LabelLine>,
    label_layout: Option<LabelLayout>,
    selected_mode: Option<bool>,
    #[charming_set_vec]
    dimensions: Vec<Dimension>,
    encode: Option<DimensionEncode>,
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}
