use serde::Serialize;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{BoundaryGap, ColorBy, CoordinateSystem, Label},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeRiver {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<BoundaryGap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl ThemeRiver {
    pub fn new() -> Self {
        Self {
            type_: "themeRiver".to_string(),
            id: None,
            name: None,
            color_by: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            coordinate_system: None,
            boundary_gap: None,
            label: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn left<S: Into<String>>(mut self, left: S) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<S: Into<String>>(mut self, top: S) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<S: Into<String>>(mut self, right: S) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<S: Into<String>>(mut self, bottom: S) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width<S: Into<String>>(mut self, width: S) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<S: Into<String>>(mut self, height: S) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Self {
        self.coordinate_system = Some(coordinate_system);
        self
    }

    pub fn boundary_gap(mut self, boundary_gap: BoundaryGap) -> Self {
        self.boundary_gap = Some(boundary_gap);
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
