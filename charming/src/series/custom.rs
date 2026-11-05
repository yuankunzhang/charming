use crate::{
    datatype::{CompositeValue, DataFrame, DataPoint, Dimension},
    element::{
        ColorBy, CoordinateSystem, DimensionEncode, ItemStyle, JsFunction, LabelLayout, LabelLine,
        Tooltip,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// A registered render-item name or explicitly trusted JavaScript source.
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum RenderItem {
    Function(JsFunction),
    Name(String),
}

impl From<&str> for RenderItem {
    fn from(value: &str) -> Self {
        Self::Name(value.to_owned())
    }
}

impl From<String> for RenderItem {
    fn from(value: String) -> Self {
        Self::Name(value)
    }
}

impl From<JsFunction> for RenderItem {
    fn from(value: JsFunction) -> Self {
        Self::Function(value)
    }
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    single_axis_index: Option<f64>,
    single_axis_id: Option<String>,
    polar_id: Option<String>,
    geo_id: Option<String>,
    x_axis_id: Option<String>,
    y_axis_id: Option<String>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_id: Option<String>,
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
    render_item: Option<RenderItem>,
    item_payload: Option<serde_json::Map<String, serde_json::Value>>,
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
