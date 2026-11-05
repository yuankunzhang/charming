use crate::{datatype::CompositeValue, element::ItemStyle};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    coordinate_system: Option<crate::element::CoordinateSystem>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
    id: Option<String>,
    show: Option<bool>,
    z: Option<f64>,
    zlevel: Option<f64>,
    left: Option<CompositeValue>,
    right: Option<CompositeValue>,
    top: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    item_style: Option<ItemStyle>,
    window_style: Option<ItemStyle>,
    series_index: Option<CompositeValue>,
    series_id: Option<CompositeValue>,
}
