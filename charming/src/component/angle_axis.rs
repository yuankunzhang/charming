use serde::Serialize;

use crate::element::{
    AxisLabel, AxisLine, AxisPointer, AxisTick, AxisType, BoundaryGap, MinorSplitLine, MinorTick,
    SplitArea, SplitLine,
};

/// The angle axis in Polar Coordinate.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AngleAxis {
    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<BoundaryGap>,

    /// Component ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// The index of angle axis in Polar Coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    polar_index: Option<f64>,

    /// Starting angle of axis, default to 90.
    #[serde(skip_serializing_if = "Option::is_none")]
    start_angle: Option<f64>,

    /// Whether the direction of axis is clockwise, default to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    clockwise: Option<bool>,

    /// Type of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<AxisType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<f64>,

    /// The minimun value of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    /// The maximum value of axis.
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_number: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_interval: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_interval: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    log_base: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_event: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_line: Option<AxisLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_tick: Option<AxisTick>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_pointer: Option<AxisPointer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minor_tick: Option<MinorTick>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label: Option<AxisLabel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_line: Option<SplitLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minor_split_line: Option<MinorSplitLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_area: Option<SplitArea>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<String>,
}

impl AngleAxis {
    pub fn new() -> Self {
        Self {
            boundary_gap: None,
            id: None,
            polar_index: None,
            start_angle: None,
            clockwise: None,
            type_: None,
            zlevel: None,
            z: None,
            min: None,
            max: None,
            scale: None,
            split_number: None,
            min_interval: None,
            max_interval: None,
            interval: None,
            log_base: None,
            silent: None,
            trigger_event: None,
            axis_line: None,
            axis_tick: None,
            axis_pointer: None,
            minor_tick: None,
            axis_label: None,
            split_line: None,
            minor_split_line: None,
            split_area: None,
            data: vec![],
        }
    }

    pub fn boundary_gap<B: Into<BoundaryGap>>(mut self, boundary_gap: B) -> Self {
        self.boundary_gap = Some(boundary_gap.into());
        self
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn polar_index<F: Into<f64>>(mut self, polar_index: F) -> Self {
        self.polar_index = Some(polar_index.into());
        self
    }

    pub fn start_angle<F: Into<f64>>(mut self, start_angle: F) -> Self {
        self.start_angle = Some(start_angle.into());
        self
    }

    pub fn clockwise(mut self, clockwise: bool) -> Self {
        self.clockwise = Some(clockwise);
        self
    }

    pub fn type_<T: Into<AxisType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
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

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
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

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
        self
    }

    pub fn trigger_event(mut self, trigger_event: bool) -> Self {
        self.trigger_event = Some(trigger_event);
        self
    }

    pub fn axis_line<A: Into<AxisLine>>(mut self, axis_line: A) -> Self {
        self.axis_line = Some(axis_line.into());
        self
    }

    pub fn axis_tick<A: Into<AxisTick>>(mut self, axis_tick: A) -> Self {
        self.axis_tick = Some(axis_tick.into());
        self
    }

    pub fn axis_pointer<A: Into<AxisPointer>>(mut self, axis_pointer: A) -> Self {
        self.axis_pointer = Some(axis_pointer.into());
        self
    }

    pub fn minor_tick<M: Into<MinorTick>>(mut self, minor_tick: M) -> Self {
        self.minor_tick = Some(minor_tick.into());
        self
    }

    pub fn axis_label<A: Into<AxisLabel>>(mut self, axis_label: A) -> Self {
        self.axis_label = Some(axis_label.into());
        self
    }

    pub fn split_line<S: Into<SplitLine>>(mut self, split_line: S) -> Self {
        self.split_line = Some(split_line.into());
        self
    }

    pub fn minor_split_line<M: Into<MinorSplitLine>>(mut self, minor_split_line: M) -> Self {
        self.minor_split_line = Some(minor_split_line.into());
        self
    }

    pub fn split_area<S: Into<SplitArea>>(mut self, split_area: S) -> Self {
        self.split_area = Some(split_area.into());
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }
}
