use serde::Serialize;

use crate::{
    datatype::CompositeValue,
    element::{
        AxisLabel, AxisLine, AxisTick, AxisType, BoundaryGap, NameLocation, ParallelLayout,
        SplitLine, TextStyle,
    },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParallelAxisDefault {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<AxisType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_location: Option<NameLocation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_text_style: Option<TextStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_rotate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inverse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<BoundaryGap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

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
    axis_label: Option<AxisLabel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_line: Option<SplitLine>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<String>,
}

impl ParallelAxisDefault {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            name_location: None,
            name_text_style: None,
            name_gap: None,
            name_rotate: None,
            inverse: None,
            boundary_gap: None,
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
            axis_label: None,
            split_line: None,
            data: vec![],
        }
    }

    pub fn type_<S: Into<AxisType>>(mut self, type_: S) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn name_location<S: Into<NameLocation>>(mut self, name_location: S) -> Self {
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

    pub fn name_rotate<F: Into<f64>>(mut self, name_rotate: F) -> Self {
        self.name_rotate = Some(name_rotate.into());
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }

    pub fn boundary_gap<B: Into<BoundaryGap>>(mut self, boundary_gap: B) -> Self {
        self.boundary_gap = Some(boundary_gap.into());
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

    pub fn axis_label<A: Into<AxisLabel>>(mut self, axis_label: A) -> Self {
        self.axis_label = Some(axis_label.into());
        self
    }

    pub fn split_line<S: Into<SplitLine>>(mut self, split_line: S) -> Self {
        self.split_line = Some(split_line.into());
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParallelCoordinate {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel: Option<f64>,

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
    width: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<ParallelLayout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parallel_axis_default: Option<ParallelAxisDefault>,
}

impl ParallelCoordinate {
    pub fn new() -> Self {
        Self {
            id: None,
            zlevel: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            layout: None,
            parallel_axis_default: None,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
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

    pub fn width<C: Into<CompositeValue>>(mut self, width: C) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<C: Into<CompositeValue>>(mut self, height: C) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn layout<L: Into<ParallelLayout>>(mut self, layout: L) -> Self {
        self.layout = Some(layout.into());
        self
    }

    pub fn parallel_axis_default<A: Into<ParallelAxisDefault>>(
        mut self,
        parallel_axis_default: A,
    ) -> Self {
        self.parallel_axis_default = Some(parallel_axis_default.into());
        self
    }
}
