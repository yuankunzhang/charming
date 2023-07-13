use serde::Serialize;

use crate::{
    datatype::CompositeValue,
    element::{
        AxisLabel, AxisLine, AxisTick, Color, Formatter, Padding, Shape, SplitArea, SplitLine,
    },
};

/// Name options for radar indicators.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarAxisName {
    /// Whether to display the indicator's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    /// Formatter of the indicator's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<Formatter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_style: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_weight: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_family: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_height: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_dash_offset: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<Padding>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_blur: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shadow_offset_y: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_border_dash_offset: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_blur: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_offset_x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_shadow_offset_y: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    overflow: Option<String>,
}

impl RadarAxisName {
    pub fn new() -> Self {
        Self {
            show: None,
            formatter: None,
            color: None,
            font_style: None,
            font_weight: None,
            font_family: None,
            font_size: None,
            line_height: None,
            background_color: None,
            border_color: None,
            border_width: None,
            border_type: None,
            border_dash_offset: None,
            border_radius: None,
            padding: None,
            shadow_color: None,
            shadow_blur: None,
            shadow_offset_x: None,
            shadow_offset_y: None,
            width: None,
            height: None,
            text_border_color: None,
            text_border_width: None,
            text_border_type: None,
            text_border_dash_offset: None,
            text_shadow_color: None,
            text_shadow_blur: None,
            text_shadow_offset_x: None,
            text_shadow_offset_y: None,
            overflow: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn formatter<C: Into<Formatter>>(mut self, formatter: C) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn font_style<S: Into<String>>(mut self, font_style: S) -> Self {
        self.font_style = Some(font_style.into());
        self
    }

    pub fn font_weight<S: Into<String>>(mut self, font_weight: S) -> Self {
        self.font_weight = Some(font_weight.into());
        self
    }

    pub fn font_family<S: Into<String>>(mut self, font_family: S) -> Self {
        self.font_family = Some(font_family.into());
        self
    }

    pub fn font_size<F: Into<f64>>(mut self, font_size: F) -> Self {
        self.font_size = Some(font_size.into());
        self
    }

    pub fn line_height<F: Into<f64>>(mut self, line_height: F) -> Self {
        self.line_height = Some(line_height.into());
        self
    }

    pub fn background_color<C: Into<Color>>(mut self, background_color: C) -> Self {
        self.background_color = Some(background_color.into());
        self
    }

    pub fn border_color<C: Into<Color>>(mut self, border_color: C) -> Self {
        self.border_color = Some(border_color.into());
        self
    }

    pub fn border_width<F: Into<f64>>(mut self, border_width: F) -> Self {
        self.border_width = Some(border_width.into());
        self
    }

    pub fn border_type<S: Into<String>>(mut self, border_type: S) -> Self {
        self.border_type = Some(border_type.into());
        self
    }

    pub fn border_dash_offset<F: Into<f64>>(mut self, border_dash_offset: F) -> Self {
        self.border_dash_offset = Some(border_dash_offset.into());
        self
    }

    pub fn border_radius<C: Into<CompositeValue>>(mut self, border_radius: C) -> Self {
        self.border_radius = Some(border_radius.into());
        self
    }

    pub fn padding<C: Into<Padding>>(mut self, padding: C) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn shadow_color<C: Into<Color>>(mut self, shadow_color: C) -> Self {
        self.shadow_color = Some(shadow_color.into());
        self
    }

    pub fn shadow_blur<F: Into<f64>>(mut self, shadow_blur: F) -> Self {
        self.shadow_blur = Some(shadow_blur.into());
        self
    }

    pub fn shadow_offset_x<F: Into<f64>>(mut self, shadow_offset_x: F) -> Self {
        self.shadow_offset_x = Some(shadow_offset_x.into());
        self
    }

    pub fn shadow_offset_y<F: Into<f64>>(mut self, shadow_offset_y: F) -> Self {
        self.shadow_offset_y = Some(shadow_offset_y.into());
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

    pub fn text_border_color<C: Into<Color>>(mut self, text_border_color: C) -> Self {
        self.text_border_color = Some(text_border_color.into());
        self
    }

    pub fn text_border_width<F: Into<f64>>(mut self, text_border_width: F) -> Self {
        self.text_border_width = Some(text_border_width.into());
        self
    }

    pub fn text_border_type<S: Into<String>>(mut self, text_border_type: S) -> Self {
        self.text_border_type = Some(text_border_type.into());
        self
    }

    pub fn text_border_dash_offset<F: Into<f64>>(mut self, text_border_dash_offset: F) -> Self {
        self.text_border_dash_offset = Some(text_border_dash_offset.into());
        self
    }

    pub fn text_shadow_color<C: Into<Color>>(mut self, text_shadow_color: C) -> Self {
        self.text_shadow_color = Some(text_shadow_color.into());
        self
    }

    pub fn text_shadow_blur<F: Into<f64>>(mut self, text_shadow_blur: F) -> Self {
        self.text_shadow_blur = Some(text_shadow_blur.into());
        self
    }

    pub fn text_shadow_offset_x<F: Into<f64>>(mut self, text_shadow_offset_x: F) -> Self {
        self.text_shadow_offset_x = Some(text_shadow_offset_x.into());
        self
    }

    pub fn text_shadow_offset_y<F: Into<f64>>(mut self, text_shadow_offset_y: F) -> Self {
        self.text_shadow_offset_y = Some(text_shadow_offset_y.into());
        self
    }

    pub fn overflow<S: Into<String>>(mut self, overflow: S) -> Self {
        self.overflow = Some(overflow.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarIndicator {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
}

impl RadarIndicator {
    pub fn new() -> Self {
        Self {
            name: None,
            max: None,
            min: None,
            color: None,
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl From<(&str, f64, f64)> for RadarIndicator {
    fn from((name, min, max): (&str, f64, f64)) -> Self {
        Self {
            name: Some(name.into()),
            min: Some(min),
            max: Some(max),
            color: None,
        }
    }
}

impl From<(&str, i64, i64)> for RadarIndicator {
    fn from((name, min, max): (&str, i64, i64)) -> Self {
        Self {
            name: Some(name.into()),
            min: Some(min as f64),
            max: Some(max as f64),
            color: None,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarCoordinate {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// The `zlevel` value of all graphical elements in.
    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel: Option<f64>,

    /// The `z` value of all graphical elements in.
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_angle: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_name: Option<RadarAxisName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_number: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    shape: Option<Shape>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_line: Option<AxisLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_tick: Option<AxisTick>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label: Option<AxisLabel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_line: Option<SplitLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_area: Option<SplitArea>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    indicator: Vec<RadarIndicator>,
}

impl RadarCoordinate {
    pub fn new() -> Self {
        Self {
            id: None,
            zlevel: None,
            z: None,
            center: None,
            radius: None,
            start_angle: None,
            axis_name: None,
            name_gap: None,
            split_number: None,
            shape: None,
            scale: None,
            axis_line: None,
            axis_tick: None,
            axis_label: None,
            split_line: None,
            split_area: None,
            indicator: vec![],
        }
    }

    pub fn center<C: Into<CompositeValue>>(mut self, center: C) -> Self {
        self.center = Some(center.into());
        self
    }

    pub fn radius<C: Into<CompositeValue>>(mut self, radius: C) -> Self {
        self.radius = Some(radius.into());
        self
    }

    pub fn start_angle<F: Into<f64>>(mut self, start_angle: F) -> Self {
        self.start_angle = Some(start_angle.into());
        self
    }

    pub fn axis_name<R: Into<RadarAxisName>>(mut self, axis_name: R) -> Self {
        self.axis_name = Some(axis_name.into());
        self
    }

    pub fn name_gap<F: Into<f64>>(mut self, name_gap: F) -> Self {
        self.name_gap = Some(name_gap.into());
        self
    }

    pub fn split_number<F: Into<f64>>(mut self, split_number: F) -> Self {
        self.split_number = Some(split_number.into());
        self
    }

    pub fn shape<S: Into<Shape>>(mut self, shape: S) -> Self {
        self.shape = Some(shape.into());
        self
    }

    pub fn scale(mut self, scale: bool) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn axis_line<L: Into<AxisLine>>(mut self, axis_line: L) -> Self {
        self.axis_line = Some(axis_line.into());
        self
    }

    pub fn axis_tick<T: Into<AxisTick>>(mut self, axis_tick: T) -> Self {
        self.axis_tick = Some(axis_tick.into());
        self
    }

    pub fn axis_label<L: Into<AxisLabel>>(mut self, axis_label: L) -> Self {
        self.axis_label = Some(axis_label.into());
        self
    }

    pub fn split_line<L: Into<SplitLine>>(mut self, split_line: L) -> Self {
        self.split_line = Some(split_line.into());
        self
    }

    pub fn split_area<A: Into<SplitArea>>(mut self, split_area: A) -> Self {
        self.split_area = Some(split_area.into());
        self
    }

    pub fn indicator<I: Into<RadarIndicator>>(mut self, indicator: Vec<I>) -> Self {
        self.indicator = indicator.into_iter().map(|i| i.into()).collect();
        self
    }
}
