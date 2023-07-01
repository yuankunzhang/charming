use serde::{Deserialize, Serialize};

use crate::utility::area_style::AreaStyle;
use crate::utility::emphasis::Emphasis;
use crate::utility::line_style::LineStyle;

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

#[derive(Serialize, Deserialize)]
pub struct DataPoint {
    pub value: Vec<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub type Data = Vec<DataPoint>;

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
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    smooth: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mark_point: Option<MarkPoint>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mark_line: Option<MarkLine>,

    data: Data,
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
            emphasis: None,
            smooth: None,
            mark_point: None,
            mark_line: None,
            data: vec![],
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

    pub fn emphasis(mut self, emphasis: Emphasis) -> Self {
        self.emphasis = Some(emphasis);
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

    pub fn data<S: Into<f64>>(mut self, data: Vec<S>) -> Self {
        for (i, d) in data.into_iter().enumerate() {
            self.data.push(DataPoint {
                value: vec![(i as f64).into(), d.into()],
                name: None,
            });
        }
        self
    }
}
