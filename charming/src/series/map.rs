use crate::{
    datatype::CompositeValue,
    element::{
        Blur, Cursor, Emphasis, ItemStyle, Label, LabelAlign, LabelVerticalAlign, PreserveAspect,
        Roam, RoamTrigger, ScaleLimit, Select, Tooltip,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    name: Option<String>,
    value: Option<CompositeValue>,
    silent: Option<bool>,
    cursor: Option<Cursor>,
    selected: Option<bool>,
    item_style: Option<ItemStyle>,
    label: Option<Label>,
    emphasis: Option<Emphasis>,
    blur: Option<Blur>,
    select: Option<Select>,
}

impl<V: Into<CompositeValue>, N: Into<String>> From<(V, N)> for MapData {
    fn from((value, name): (V, N)) -> Self {
        Self::new().name(name).value(value)
    }
}

#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    coordinate_system: Option<crate::element::CoordinateSystem>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
    #[serde(rename = "type")]
    #[charming_type = "map"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    map: Option<String>,
    geo_index: Option<f64>,
    geo_id: Option<String>,
    name_property: Option<String>,
    name_map: Option<std::collections::HashMap<String, String>>,
    roam: Option<Roam>,
    roam_trigger: Option<RoamTrigger>,
    clip: Option<bool>,
    center: Option<CompositeValue>,
    zoom: Option<f64>,
    scale_limit: Option<ScaleLimit>,
    aspect_scale: Option<f64>,
    layout_center: Option<CompositeValue>,
    layout_size: Option<CompositeValue>,
    preserve_aspect: Option<PreserveAspect>,
    preserve_aspect_align: Option<LabelAlign>,
    preserve_aspect_vertical_align: Option<LabelVerticalAlign>,
    left: Option<CompositeValue>,
    right: Option<CompositeValue>,
    top: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    item_style: Option<ItemStyle>,
    label: Option<Label>,
    emphasis: Option<Emphasis>,
    blur: Option<Blur>,
    select: Option<Select>,
    silent: Option<bool>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<MapData>,
}
