use serde::Serialize;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        AreaStyle, CoordinateSystem, DimensionEncode, Emphasis, ItemStyle, Label, LineStyle,
        MarkArea, MarkLine, MarkPoint, Symbol,
    },
};

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
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show_symbol: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stack: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    area_style: Option<AreaStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

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
    encode: Option<DimensionEncode>,

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
            coordinate_system: None,
            symbol: None,
            symbol_size: None,
            show_symbol: None,
            stack: None,
            label: None,
            line_style: None,
            area_style: None,
            item_style: None,
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

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
        self
    }

    pub fn symbol<S: Into<Symbol>>(mut self, symbol: S) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn symbol_size<F: Into<f64>>(mut self, symbol_size: F) -> Self {
        self.symbol_size = Some(symbol_size.into());
        self
    }

    pub fn show_symbol(mut self, show_symbol: bool) -> Self {
        self.show_symbol = Some(show_symbol);
        self
    }

    pub fn stack<S: Into<String>>(mut self, stack: S) -> Self {
        self.stack = Some(stack.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn line_style<L: Into<LineStyle>>(mut self, line_style: L) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn area_style<A: Into<AreaStyle>>(mut self, area_style: A) -> Self {
        self.area_style = Some(area_style.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    /// Smoothness.
    pub fn smooth<F: Into<f64>>(mut self, smooth: F) -> Self {
        self.smooth = Some(smooth.into());
        self
    }

    pub fn mark_point<M: Into<MarkPoint>>(mut self, mark_point: M) -> Self {
        self.mark_point = Some(mark_point.into());
        self
    }

    pub fn mark_line<M: Into<MarkLine>>(mut self, mark_line: M) -> Self {
        self.mark_line = Some(mark_line.into());
        self
    }

    pub fn mark_area<M: Into<MarkArea>>(mut self, mark_area: M) -> Self {
        self.mark_area = Some(mark_area.into());
        self
    }

    pub fn dataset_id<S: Into<String>>(mut self, dataset_id: S) -> Self {
        self.dataset_id = Some(dataset_id.into());
        self
    }

    pub fn encode<E: Into<DimensionEncode>>(mut self, encode: E) -> Self {
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
