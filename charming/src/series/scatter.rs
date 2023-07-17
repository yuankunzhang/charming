use serde::Serialize;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        ColorBy, CoordinateSystem, DimensionEncode, Emphasis, ItemStyle, MarkArea, MarkLine,
        Symbol, SymbolSize,
    },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scatter {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<SymbolSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    encode: Option<DimensionEncode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mark_line: Option<MarkLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mark_area: Option<MarkArea>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl Scatter {
    pub fn new() -> Self {
        Self {
            type_: String::from("scatter"),
            id: None,
            name: None,
            color_by: None,
            dataset_index: None,
            coordinate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            symbol: None,
            symbol_size: None,
            encode: None,
            mark_line: None,
            mark_area: None,
            item_style: None,
            emphasis: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn dataset_index<F: Into<f64>>(mut self, dataset_index: F) -> Self {
        self.dataset_index = Some(dataset_index.into());
        self
    }

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
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

    pub fn symbol(mut self, symbol: Symbol) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn symbol_size<S: Into<SymbolSize>>(mut self, symbol_size: S) -> Self {
        self.symbol_size = Some(symbol_size.into());
        self
    }

    pub fn encode<D: Into<DimensionEncode>>(mut self, encode: D) -> Self {
        self.encode = Some(encode.into());
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

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
