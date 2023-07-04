use serde::Serialize;

use crate::datatype::{DataFrame, DataPoint};
use crate::element::area_style::AreaStyle;
use crate::element::emphasis::Emphasis;
use crate::element::line_style::LineStyle;
use crate::element::mark_area::MarkArea;
use crate::element::mark_line::MarkLine;
use crate::element::mark_point::MarkPoint;
use crate::element::symbol::Symbol;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Encode {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_name: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tooltip: Vec<String>,
}

impl Encode {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            item_name: None,
            tooltip: vec![],
        }
    }

    pub fn x<S: Into<String>>(mut self, x: S) -> Self {
        self.x = Some(x.into());
        self
    }

    pub fn y<S: Into<String>>(mut self, y: S) -> Self {
        self.y = Some(y.into());
        self
    }

    pub fn item_name<S: Into<String>>(mut self, item_name: S) -> Self {
        self.item_name = Some(item_name.into());
        self
    }

    pub fn tooltip<S: Into<String>>(mut self, tooltip: Vec<S>) -> Self {
        self.tooltip = tooltip.into_iter().map(|s| s.into()).collect();
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    mark_area: Option<MarkArea>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    encode: Option<Encode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl Line {
    pub fn new() -> Self {
        Self {
            type_: "line".to_string(),
            id: None,
            name: None,
            symbol: None,
            show_symbol: None,
            stack: None,
            line_style: None,
            area_style: None,
            emphasis: None,
            smooth: None,
            mark_point: None,
            mark_line: None,
            mark_area: None,
            dataset_id: None,
            encode: None,
            x_axis_index: None,
            y_axis_index: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Series name used for displaying in `tooltip` and filtering with `legend`.
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn symbol(mut self, symbol: Symbol) -> Self {
        self.symbol = Some(symbol);
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

    pub fn mark_area(mut self, mark_area: MarkArea) -> Self {
        self.mark_area = Some(mark_area);
        self
    }

    pub fn dataset_id<S: Into<String>>(mut self, dataset_id: S) -> Self {
        self.dataset_id = Some(dataset_id.into());
        self
    }

    pub fn encode<E: Into<Encode>>(mut self, encode: E) -> Self {
        self.encode = Some(encode.into());
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

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
