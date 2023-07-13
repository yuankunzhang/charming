use serde::Serialize;

use crate::{
    datatype::CompositeValue,
    element::{
        AxisLabel, AxisLine, AxisPointer, AxisTick, AxisType, BoundaryGap, NameLocation, SplitArea,
        SplitLine, TextStyle,
    },
};

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

    /// Whether to show the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    /// Index of grid which is used to place this axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    grid_index: Option<f64>,

    /// Offset of this axis relative to default position.
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<f64>,

    /// Name of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// Location of axis name.
    #[serde(skip_serializing_if = "Option::is_none")]
    name_location: Option<NameLocation>,

    /// Text style of axis name.
    #[serde(skip_serializing_if = "Option::is_none")]
    name_text_style: Option<TextStyle>,

    /// Gap between axis name and axis line.
    #[serde(skip_serializing_if = "Option::is_none")]
    name_gap: Option<f64>,

    /// Rotation of axis name
    #[serde(skip_serializing_if = "Option::is_none")]
    name_rotation: Option<f64>,

    /// Set this to `true` to invert the axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    inverse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    align_ticks: Option<bool>,

    /// The boundary gap on both sides of a coordinate axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<BoundaryGap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<CompositeValue>,

    /// The mimimum value of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<CompositeValue>,

    /// The maximum value of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<bool>,

    /// Number of segments that the axis is split into.
    #[serde(skip_serializing_if = "Option::is_none")]
    split_number: Option<f64>,

    /// Minimum gap between split lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    min_interval: Option<f64>,

    /// Maximum gap between split lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_interval: Option<f64>,

    /// Compulsively set segmentation interval for axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<f64>,

    /// Base of logarithm, which is valid only for numeric axes with `log` type.
    #[serde(skip_serializing_if = "Option::is_none")]
    log_base: Option<f64>,

    /// Settings related to axis label.
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label: Option<AxisLabel>,

    /// Settings related to axis tick.
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_tick: Option<AxisTick>,

    /// Settings related to axis line.
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_line: Option<AxisLine>,

    /// Settings related to axis pointer.
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_pointer: Option<AxisPointer>,

    /// Settings related to split area.
    #[serde(skip_serializing_if = "Option::is_none")]
    split_area: Option<SplitArea>,

    /// Settings related to split line.
    #[serde(skip_serializing_if = "Option::is_none")]
    split_line: Option<SplitLine>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<String>,
}

impl Axis {
    pub fn new() -> Self {
        Self {
            type_: None,
            id: None,
            show: None,
            grid_index: None,
            offset: None,
            name: None,
            name_location: None,
            name_text_style: None,
            name_gap: None,
            name_rotation: None,
            inverse: None,
            boundary_gap: None,
            position: None,
            min: None,
            max: None,
            scale: None,
            split_number: None,
            min_interval: None,
            max_interval: None,
            interval: None,
            align_ticks: None,
            log_base: None,
            axis_label: None,
            axis_tick: None,
            axis_line: None,
            axis_pointer: None,
            split_area: None,
            split_line: None,
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

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn grid_index<F: Into<f64>>(mut self, grid_index: F) -> Self {
        self.grid_index = Some(grid_index.into());
        self
    }

    pub fn offset<F: Into<f64>>(mut self, offset: F) -> Self {
        self.offset = Some(offset.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn name_location<E: Into<NameLocation>>(mut self, name_location: E) -> Self {
        self.name_location = Some(name_location.into());
        self
    }

    pub fn name_text_style<T: Into<TextStyle>>(mut self, name_text_style: T) -> Self {
        self.name_text_style = Some(name_text_style.into());
        self
    }

    pub fn name_gap<F: Into<f64>>(mut self, name_gap: F) -> Self {
        self.name_gap = Some(name_gap.into());
        self
    }

    pub fn name_rotation<F: Into<f64>>(mut self, name_rotation: F) -> Self {
        self.name_rotation = Some(name_rotation.into());
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }

    pub fn align_ticks(mut self, align_ticks: bool) -> Self {
        self.align_ticks = Some(align_ticks);
        self
    }

    pub fn boundary_gap<B: Into<BoundaryGap>>(mut self, boundary_gap: B) -> Self {
        self.boundary_gap = Some(boundary_gap.into());
        self
    }

    pub fn position<C: Into<CompositeValue>>(mut self, position: C) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn min<C: Into<CompositeValue>>(mut self, min: C) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<C: Into<CompositeValue>>(mut self, max: C) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn scale(mut self, scale: bool) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn split_number<F: Into<f64>>(mut self, split_number: F) -> Self {
        self.split_number = Some(split_number.into());
        self
    }

    pub fn min_interval<F: Into<f64>>(mut self, min_interval: F) -> Self {
        self.min_interval = Some(min_interval.into());
        self
    }

    pub fn max_interval<F: Into<f64>>(mut self, max_interval: F) -> Self {
        self.max_interval = Some(max_interval.into());
        self
    }

    pub fn interval<F: Into<f64>>(mut self, interval: F) -> Self {
        self.interval = Some(interval.into());
        self
    }

    pub fn log_base<F: Into<f64>>(mut self, log_base: F) -> Self {
        self.log_base = Some(log_base.into());
        self
    }

    pub fn axis_label<L: Into<AxisLabel>>(mut self, axis_label: L) -> Self {
        self.axis_label = Some(axis_label.into());
        self
    }

    pub fn axis_tick<T: Into<AxisTick>>(mut self, axis_tick: T) -> Self {
        self.axis_tick = Some(axis_tick.into());
        self
    }

    pub fn axis_line<L: Into<AxisLine>>(mut self, axis_line: L) -> Self {
        self.axis_line = Some(axis_line.into());
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

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }
}
