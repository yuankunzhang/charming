use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};

use crate::utility::area_style::AreaStyle;
use crate::utility::emphasis::Emphasis;
use crate::utility::label::Label;
use crate::utility::line_style::LineStyle;
use crate::utility::symbol::Symbol;

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
            "avg" | "average" => Self::Average,
            _ => panic!("Invalid MarkPointDataType"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPointData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<MarkPointDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<f64>,
}

impl MarkPointData {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            x_axis: None,
            y_axis: None,
            value: None,
        }
    }

    pub fn type_<T: Into<MarkPointDataType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn x_axis<F: Into<f64>>(mut self, x_axis: F) -> Self {
        self.x_axis = Some(x_axis.into());
        self
    }

    pub fn y_axis<F: Into<f64>>(mut self, y_axis: F) -> Self {
        self.y_axis = Some(y_axis.into());
        self
    }

    pub fn value<F: Into<f64>>(mut self, value: F) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl From<(&str, &str)> for MarkPointData {
    fn from((type_, name): (&str, &str)) -> Self {
        Self::new().type_(type_).name(name)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPoint {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<MarkPointData>,
}

impl MarkPoint {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn data<D: Into<MarkPointData>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
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
            "avg" | "average" => Self::Average,
            "med" | "median" => Self::Median,
            _ => panic!("Invalid MarkLineDataType"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkLineData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<MarkLineDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,
}

impl MarkLineData {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            symbol: None,
            x: None,
            y_axis: None,
            label: None,
        }
    }

    pub fn type_<T: Into<MarkLineDataType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn symbol(mut self, symbol: Symbol) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn x<S: Into<String>>(mut self, x: S) -> Self {
        self.x = Some(x.into());
        self
    }

    pub fn y_axis<S: Into<String>>(mut self, y_axis: S) -> Self {
        self.y_axis = Some(y_axis.into());
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }
}

impl From<(&str, &str)> for MarkLineData {
    fn from((type_, name): (&str, &str)) -> Self {
        Self::new().type_(type_).name(name)
    }
}

pub enum MarkLineVariant {
    Simple(MarkLineData),
    StartToEnd(MarkLineData, MarkLineData),
}

impl Serialize for MarkLineVariant {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MarkLineVariant::Simple(data) => data.serialize(serializer),
            MarkLineVariant::StartToEnd(start, end) => {
                let mut s = serializer.serialize_seq(Some(2))?;
                s.serialize_element(start)?;
                s.serialize_element(end)?;
                s.end()
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkLine {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<MarkLineVariant>,
}

impl MarkLine {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn data(mut self, data: Vec<MarkLineVariant>) -> Self {
        self.data = data;
        self
    }
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub value: Vec<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub type Data = Vec<Datum>;

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
            self.data.push(Datum {
                value: vec![(i as f64).into(), d.into()],
                name: None,
            });
        }
        self
    }
}
