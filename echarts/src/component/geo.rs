use serde::Serialize;

use crate::{
    datatype::CompositeValue,
    element::{Blur, Emphasis, ItemStyle, Label, Select},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Geo {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    map: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    roam: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_scale: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bounding_coords: Option<((String, String), (String, String))>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zoom: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale_limit: Option<(f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_map: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_property: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected_mode: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    select: Option<Select>,

    #[serde(skip_serializing_if = "Option::is_none")]
    blur: Option<Blur>,

    /// The `zlevel` value of all graphical elements in.
    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel: Option<f64>,

    /// The `z` value of all graphical elements in.
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout_center: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout_size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
}

impl Geo {
    pub fn new() -> Self {
        Self {
            show: None,
            map: None,
            roam: None,
            center: None,
            aspect_scale: None,
            bounding_coords: None,
            zoom: None,
            scale_limit: None,
            name_map: None,
            name_property: None,
            selected_mode: None,
            label: None,
            item_style: None,
            emphasis: None,
            select: None,
            blur: None,
            zlevel: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            layout_center: None,
            layout_size: None,
            silent: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn map<S: Into<String>>(mut self, map: S) -> Self {
        self.map = Some(map.into());
        self
    }

    pub fn roam(mut self, roam: bool) -> Self {
        self.roam = Some(roam);
        self
    }

    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn aspect_scale<F: Into<f64>>(mut self, aspect_scale: F) -> Self {
        self.aspect_scale = Some(aspect_scale.into());
        self
    }

    pub fn bounding_coords<S: Into<String>>(mut self, bounding_coords: ((S, S), (S, S))) -> Self {
        self.bounding_coords = Some((
            ((bounding_coords.0).0.into(), (bounding_coords.0).1.into()),
            ((bounding_coords.1).0.into(), (bounding_coords.1).1.into()),
        ));
        self
    }

    pub fn zoom<F: Into<f64>>(mut self, zoom: F) -> Self {
        self.zoom = Some(zoom.into());
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

    pub fn name_property<S: Into<String>>(mut self, name_property: S) -> Self {
        self.name_property = Some(name_property.into());
        self
    }

    pub fn selected_mode(mut self, selected_mode: bool) -> Self {
        self.selected_mode = Some(selected_mode);
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn item_style<S: Into<ItemStyle>>(mut self, item_style: S) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn select<S: Into<Select>>(mut self, select: S) -> Self {
        self.select = Some(select.into());
        self
    }

    pub fn blur<B: Into<Blur>>(mut self, blur: B) -> Self {
        self.blur = Some(blur.into());
        self
    }

    pub fn zlevel<F: Into<f64>>(mut self, zlevel: F) -> Self {
        self.zlevel = Some(zlevel.into());
        self
    }

    pub fn z<F: Into<f64>>(mut self, z: F) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn layout_center<S: Into<String>>(mut self, layout_center: (S, S)) -> Self {
        self.layout_center = Some((layout_center.0.into(), layout_center.1.into()));
        self
    }

    pub fn layout_size<S: Into<String>>(mut self, layout_size: S) -> Self {
        self.layout_size = Some(layout_size.into());
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
        self
    }
}
