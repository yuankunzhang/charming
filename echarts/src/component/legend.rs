use serde::Serialize;

use crate::element::{
    ItemStyle, LabelAlign, LineStyle, Orient, Padding, PositionProperty, TextStyle,
};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LegendType {
    /// Simple legend.
    Plain,

    /// Scrollable legend.
    Scroll,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectedMode {
    /// Multiple selection.
    Multiple,

    /// Single selection.
    Single,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Legend {
    /// Type of legend.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<LegendType>,

    /// Component ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// Whether to show the legend component.
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    /// The `zlevel` value of all graphical elements in.
    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel: Option<f64>,

    /// The `z` value of all graphical elements in.
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<f64>,

    /// Distance between title component and the left side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<PositionProperty>,

    /// Distance between title component and the top side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<PositionProperty>,

    /// Distance between title component and the right side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<PositionProperty>,

    /// Distance between title component and the bottom side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<PositionProperty>,

    /// Width of legend component.
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,

    /// Height of legend component.
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,

    /// The layout orientation of legend.
    #[serde(skip_serializing_if = "Option::is_none")]
    orient: Option<Orient>,

    /// The align of legend.
    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<LabelAlign>,

    /// Legend padding.
    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<Padding>,

    /// The gap between each legend.
    #[serde(skip_serializing_if = "Option::is_none")]
    item_gap: Option<f64>,

    /// Width of legend symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    item_width: Option<f64>,

    /// Height of legend symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    item_height: Option<f64>,

    /// Legend item style.
    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    /// Legend line style.
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    text_style: Option<TextStyle>,

    /// Rotation of the symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_rotate: Option<String>,

    /// Formatter is used to format label of legend.
    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected_mode: Option<SelectedMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<String>>,
}

impl Legend {
    pub fn new() -> Self {
        Self {
            type_: None,
            id: None,
            show: None,
            zlevel: None,
            z: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            orient: None,
            align: None,
            padding: None,
            item_gap: None,
            item_width: None,
            item_height: None,
            item_style: None,
            line_style: None,
            text_style: None,
            symbol_rotate: None,
            formatter: None,
            selected_mode: None,
            data: None,
        }
    }

    pub fn type_<T: Into<LegendType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
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

    pub fn left<P: Into<PositionProperty>>(mut self, left: P) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<P: Into<PositionProperty>>(mut self, top: P) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<P: Into<PositionProperty>>(mut self, right: P) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<P: Into<PositionProperty>>(mut self, bottom: P) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width<F: Into<f64>>(mut self, width: F) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<F: Into<f64>>(mut self, height: F) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn orient<O: Into<Orient>>(mut self, orient: O) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn align<A: Into<LabelAlign>>(mut self, align: A) -> Self {
        self.align = Some(align.into());
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn item_gap<F: Into<f64>>(mut self, item_gap: F) -> Self {
        self.item_gap = Some(item_gap.into());
        self
    }

    pub fn item_width<F: Into<f64>>(mut self, item_width: F) -> Self {
        self.item_width = Some(item_width.into());
        self
    }

    pub fn item_height<F: Into<f64>>(mut self, item_height: F) -> Self {
        self.item_height = Some(item_height.into());
        self
    }

    pub fn item_style<S: Into<ItemStyle>>(mut self, item_style: S) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn line_style<S: Into<LineStyle>>(mut self, line_style: S) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn text_style<S: Into<TextStyle>>(mut self, text_style: S) -> Self {
        self.text_style = Some(text_style.into());
        self
    }

    pub fn symbol_rotate<S: Into<String>>(mut self, symbol_rotate: S) -> Self {
        self.symbol_rotate = Some(symbol_rotate.into());
        self
    }

    pub fn formatter<S: Into<String>>(mut self, formatter: S) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn selected_mode<S: Into<SelectedMode>>(mut self, selected_mode: S) -> Self {
        self.selected_mode = Some(selected_mode.into());
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        let data = data.into_iter().map(|s| s.into()).collect();
        self.data = Some(data);
        self
    }
}
