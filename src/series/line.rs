use serde::Serialize;

use crate::utility::color::*;
use crate::utility::data::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
}

impl LineStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            width: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginPosition {
    Auto,
    Start,
    End,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<OriginPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
}

impl AreaStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            origin: None,
            opacity: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn origin(mut self, origin: OriginPosition) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkPointDataType {
    Min,
    Max,
    Average,
}

impl From<&str> for MarkPointDataType {
    fn from(s: &str) -> Self {
        match s {
            "min" => Self::Min,
            "max" => Self::Max,
            "average" => Self::Average,
            _ => panic!("Invalid MarkPointDataType"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPointData {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<MarkPointDataType>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<MarkPointData>>,
}

impl MarkPoint {
    pub fn new() -> Self {
        Self { data: None }
    }

    pub fn data(mut self, data: Vec<MarkPointData>) -> Self {
        self.data = Some(data);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkLineDataType {
    Min,
    Max,
    Average,
    Median,
}

impl From<&str> for MarkLineDataType {
    fn from(s: &str) -> Self {
        match s {
            "min" => Self::Min,
            "max" => Self::Max,
            "average" => Self::Average,
            "median" => Self::Median,
            _ => panic!("Invalid MarkLineDataType"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkLineData {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<MarkLineDataType>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<MarkLineData>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_symbol: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    area_style: Option<AreaStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    smooth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mark_point: Option<MarkPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mark_line: Option<MarkLine>,
    data: DataFrame,
}

impl Line {
    pub fn new() -> Self {
        Self {
            type_: "line".to_string(),
            name: None,
            show_symbol: None,
            stack: None,
            line_style: None,
            area_style: None,
            smooth: None,
            mark_point: None,
            mark_line: None,
            data: DataFrame::new(),
        }
    }

    /// Series name used for displaying in `tooltip` and filtering with `legend`.
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn show_symbol(mut self, show_symbol: bool) -> Self {
        self.show_symbol = Some(show_symbol);
        self
    }

    pub fn stack(mut self, stack: &str) -> Self {
        self.stack = Some(stack.to_string());
        self
    }

    pub fn line_style(mut self, line_style: LineStyle) -> Self {
        self.line_style = Some(line_style);
        self
    }

    pub fn area_style(mut self, area_style: AreaStyle) -> Self {
        self.area_style = Some(area_style);
        self
    }

    /// Smoothness.
    pub fn smooth(mut self, smooth: f64) -> Self {
        self.smooth = Some(smooth);
        self
    }

    pub fn mark_point(mut self, mark_point: MarkPoint) -> Self {
        self.mark_point = Some(mark_point);
        self
    }

    pub fn mark_line(mut self, mark_line: MarkLine) -> Self {
        self.mark_line = Some(mark_line);
        self
    }

    pub fn data(mut self, data: DataFrameOneDimension) -> Self {
        for (i, d) in data.into_iter().enumerate() {
            self.data.push(vec![(i as f64).into(), d.into()]);
        }
        self
    }

    pub fn dataframe(mut self, data: DataFrame) -> Self {
        self.data = data;
        self
    }
}
