use crate::{
    datatype::CompositeValue,
    element::{Blur, Emphasis, ItemStyle, Label, Select},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// Region overrides, also usable as a GeoJSON feature's `properties.echartsStyle`.
#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoRegion {
    name: Option<String>,
    selected: Option<bool>,
    silent: Option<bool>,
    item_style: Option<ItemStyle>,
    label: Option<Label>,
    emphasis: Option<Emphasis>,
    select: Option<Select>,
    blur: Option<Blur>,
    tooltip: Option<crate::element::Tooltip>,
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Geo {
    id: Option<String>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    roam_trigger: Option<crate::element::RoamTrigger>,
    clip: Option<bool>,
    preserve_aspect: Option<crate::element::PreserveAspect>,
    preserve_aspect_align: Option<crate::element::LabelAlign>,
    preserve_aspect_vertical_align: Option<crate::element::LabelVerticalAlign>,
    regions: Option<Vec<GeoRegion>>,
    tooltip: Option<crate::element::Tooltip>,
    coordinate_system: Option<crate::element::CoordinateSystem>,
    coordinate_system_usage: Option<crate::element::CoordinateSystemUsage>,
    coord: Option<crate::datatype::CompositeValue>,
    matrix_index: Option<f64>,
    matrix_id: Option<String>,
    calendar_index: Option<f64>,
    calendar_id: Option<String>,
    show: Option<bool>,
    map: Option<String>,
    roam: Option<crate::element::Roam>,
    #[charming_skip_setter]
    center: Option<(String, String)>,
    aspect_scale: Option<f64>,
    #[charming_skip_setter]
    bounding_coords: Option<((String, String), (String, String))>,
    zoom: Option<f64>,
    #[charming_skip_setter]
    scale_limit: Option<(f64, f64)>,
    #[charming_skip_setter]
    name_map: Option<(String, String)>,
    name_property: Option<String>,
    selected_mode: Option<bool>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    select: Option<Select>,
    blur: Option<Blur>,
    /// The `zlevel` value of all graphical elements in.
    zlevel: Option<f64>,
    /// The `z` value of all graphical elements in.
    z: Option<f64>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    #[charming_skip_setter]
    layout_center: Option<(String, String)>,
    layout_size: Option<String>,
    silent: Option<bool>,
}

impl Geo {
    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn bounding_coords<S: Into<String>>(mut self, bounding_coords: ((S, S), (S, S))) -> Self {
        self.bounding_coords = Some((
            ((bounding_coords.0).0.into(), (bounding_coords.0).1.into()),
            ((bounding_coords.1).0.into(), (bounding_coords.1).1.into()),
        ));
        self
    }

    pub fn scale_limit<F: Into<f64>>(mut self, scale_limit: (F, F)) -> Self {
        self.scale_limit = Some((scale_limit.0.into(), scale_limit.1.into()));
        self
    }

    pub fn name_map<S: Into<String>>(mut self, name_map: (S, S)) -> Self {
        self.name_map = Some((name_map.0.into(), name_map.1.into()));
        self
    }

    pub fn layout_center<S: Into<String>>(mut self, layout_center: (S, S)) -> Self {
        self.layout_center = Some((layout_center.0.into(), layout_center.1.into()));
        self
    }
}
