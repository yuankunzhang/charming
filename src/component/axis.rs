use serde::Serialize;

use crate::element::{
    axis_label::AxisLabel, axis_type::AxisType, boundary_gap::BoundaryGap, split_area::SplitArea,
    split_line::SplitLine,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointer {
    #[serde(skip_serializing_if = "Option::is_none")]
    snap: Option<bool>,
}

impl AxisPointer {
    pub fn new() -> Self {
        Self { snap: None }
    }

    pub fn snap(mut self, snap: bool) -> Self {
        self.snap = Some(snap);
        self
    }
}

/// Axis in cartesian coordinate.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Axis {
    /// Type of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<AxisType>,

    /// Id of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// Name of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<BoundaryGap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label: Option<AxisLabel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_pointer: Option<AxisPointer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_area: Option<SplitArea>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_line: Option<SplitLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    grid_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<String>,
}

impl Axis {
    pub fn new() -> Self {
        Self {
            type_: None,
            id: None,
            name: None,
            boundary_gap: None,
            name_gap: None,
            axis_label: None,
            axis_pointer: None,
            split_area: None,
            split_line: None,
            name_location: None,
            scale: None,
            grid_index: None,
            min: None,
            max: None,
            data: vec![],
        }
    }

    pub fn type_<T: Into<AxisType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn boundary_gap<B: Into<BoundaryGap>>(mut self, boundary_gap: B) -> Self {
        self.boundary_gap = Some(boundary_gap.into());
        self
    }

    pub fn name_gap<F: Into<f64>>(mut self, name_gap: F) -> Self {
        self.name_gap = Some(name_gap.into());
        self
    }

    pub fn axis_label<L: Into<AxisLabel>>(mut self, axis_label: L) -> Self {
        self.axis_label = Some(axis_label.into());
        self
    }

    pub fn axis_pointer<P: Into<AxisPointer>>(mut self, axis_pointer: P) -> Self {
        self.axis_pointer = Some(axis_pointer.into());
        self
    }

    pub fn split_area<A: Into<SplitArea>>(mut self, split_area: A) -> Self {
        self.split_area = Some(split_area.into());
        self
    }

    pub fn split_line<A: Into<SplitLine>>(mut self, split_line: A) -> Self {
        self.split_line = Some(split_line.into());
        self
    }

    pub fn name_location<S: Into<String>>(mut self, name_location: S) -> Self {
        self.name_location = Some(name_location.into());
        self
    }

    pub fn scale(mut self, scale: bool) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn grid_index<F: Into<f64>>(mut self, grid_index: F) -> Self {
        self.grid_index = Some(grid_index.into());
        self
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }
}
