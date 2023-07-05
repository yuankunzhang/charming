use serde::Serialize;

use crate::element::{Color, DataBackground, Orient};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterMode {
    Filter,
    WeakFilter,
    Empty,
    None,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsideDataZoom {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radius_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    angle_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filter_mode: Option<FilterMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_value: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_value: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_span: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_span: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_value_span: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_value_span: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    orient: Option<Orient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zoom_lock: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    throttle: Option<f64>,
}

impl InsideDataZoom {
    pub fn new() -> Self {
        Self {
            type_: "inside".to_string(),
            id: None,
            disabled: None,
            x_axis_index: None,
            y_axis_index: None,
            radius_axis_index: None,
            angle_axis_index: None,
            filter_mode: None,
            start: None,
            end: None,
            start_value: None,
            end_value: None,
            min_span: None,
            max_span: None,
            min_value_span: None,
            max_value_span: None,
            orient: None,
            zoom_lock: None,
            throttle: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    pub fn x_axis_index<F: Into<f64>>(mut self, x_axis_index: F) -> Self {
        self.x_axis_index = Some(x_axis_index.into());
        self
    }

    pub fn y_axis_index<F: Into<f64>>(mut self, y_axis_index: F) -> Self {
        self.y_axis_index = Some(y_axis_index.into());
        self
    }

    pub fn radius_axis_index<F: Into<f64>>(mut self, radius_axis_index: F) -> Self {
        self.radius_axis_index = Some(radius_axis_index.into());
        self
    }

    pub fn angle_axis_index<F: Into<f64>>(mut self, angle_axis_index: F) -> Self {
        self.angle_axis_index = Some(angle_axis_index.into());
        self
    }

    pub fn filter_mode<F: Into<FilterMode>>(mut self, filter_mode: F) -> Self {
        self.filter_mode = Some(filter_mode.into());
        self
    }

    pub fn start<F: Into<f64>>(mut self, start: F) -> Self {
        self.start = Some(start.into());
        self
    }

    pub fn end<F: Into<f64>>(mut self, end: F) -> Self {
        self.end = Some(end.into());
        self
    }

    pub fn start_value<F: Into<f64>>(mut self, start_value: F) -> Self {
        self.start_value = Some(start_value.into());
        self
    }

    pub fn end_value<F: Into<f64>>(mut self, end_value: F) -> Self {
        self.end_value = Some(end_value.into());
        self
    }

    pub fn min_span<F: Into<f64>>(mut self, min_span: F) -> Self {
        self.min_span = Some(min_span.into());
        self
    }

    pub fn max_span<F: Into<f64>>(mut self, max_span: F) -> Self {
        self.max_span = Some(max_span.into());
        self
    }

    pub fn min_value_span<F: Into<f64>>(mut self, min_value_span: F) -> Self {
        self.min_value_span = Some(min_value_span.into());
        self
    }

    pub fn max_value_span<F: Into<f64>>(mut self, max_value_span: F) -> Self {
        self.max_value_span = Some(max_value_span.into());
        self
    }

    pub fn orient<F: Into<Orient>>(mut self, orient: F) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn zoom_lock(mut self, zoom_lock: bool) -> Self {
        self.zoom_lock = Some(zoom_lock);
        self
    }

    pub fn throttle<F: Into<f64>>(mut self, throttle: F) -> Self {
        self.throttle = Some(throttle.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SliderDataZoom {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data_background: Option<DataBackground>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected_data_background: Option<DataBackground>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_value: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_value: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_span: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_span: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_value_span: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_value_span: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    orient: Option<Orient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zoom_lock: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    throttle: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,
}

impl SliderDataZoom {
    pub fn new() -> Self {
        Self {
            type_: "slider".to_string(),
            id: None,
            show: None,
            background_color: None,
            data_background: None,
            selected_data_background: None,
            start: None,
            end: None,
            start_value: None,
            end_value: None,
            min_span: None,
            max_span: None,
            min_value_span: None,
            max_value_span: None,
            orient: None,
            zoom_lock: None,
            throttle: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            x_axis_index: None,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn background_color<C: Into<Color>>(mut self, background_color: C) -> Self {
        self.background_color = Some(background_color.into());
        self
    }

    pub fn data_background<D: Into<DataBackground>>(mut self, data_background: D) -> Self {
        self.data_background = Some(data_background.into());
        self
    }

    pub fn selected_data_background<D: Into<DataBackground>>(
        mut self,
        selected_data_background: D,
    ) -> Self {
        self.selected_data_background = Some(selected_data_background.into());
        self
    }

    pub fn start<F: Into<f64>>(mut self, start: F) -> Self {
        self.start = Some(start.into());
        self
    }

    pub fn end<F: Into<f64>>(mut self, end: F) -> Self {
        self.end = Some(end.into());
        self
    }

    pub fn start_value<F: Into<f64>>(mut self, start_value: F) -> Self {
        self.start_value = Some(start_value.into());
        self
    }

    pub fn end_value<F: Into<f64>>(mut self, end_value: F) -> Self {
        self.end_value = Some(end_value.into());
        self
    }

    pub fn min_span<F: Into<f64>>(mut self, min_span: F) -> Self {
        self.min_span = Some(min_span.into());
        self
    }

    pub fn max_span<F: Into<f64>>(mut self, max_span: F) -> Self {
        self.max_span = Some(max_span.into());
        self
    }

    pub fn min_value_span<F: Into<f64>>(mut self, min_value_span: F) -> Self {
        self.min_value_span = Some(min_value_span.into());
        self
    }

    pub fn max_value_span<F: Into<f64>>(mut self, max_value_span: F) -> Self {
        self.max_value_span = Some(max_value_span.into());
        self
    }

    pub fn orient<O: Into<Orient>>(mut self, orient: O) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn zoom_lock(mut self, zoom_lock: bool) -> Self {
        self.zoom_lock = Some(zoom_lock);
        self
    }

    pub fn throttle<F: Into<f64>>(mut self, throttle: F) -> Self {
        self.throttle = Some(throttle.into());
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

    pub fn x_axis_index<F: Into<f64>>(mut self, x_axis_index: F) -> Self {
        self.x_axis_index = Some(x_axis_index.into());
        self
    }
}

pub enum DataZoom {
    Inside(InsideDataZoom),
    Slider(SliderDataZoom),
}

impl Serialize for DataZoom {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            DataZoom::Inside(inside) => inside.serialize(serializer),
            DataZoom::Slider(slider) => slider.serialize(serializer),
        }
    }
}
