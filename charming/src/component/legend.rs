use serde::Serialize;

use crate::{
    datatype::CompositeValue,
    element::{Color, Icon, ItemStyle, LabelAlign, LineStyle, Orient, Padding, TextStyle},
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
pub enum LegendSelectedMode {
    /// Multiple selection.
    Multiple,

    /// Single selection.
    Single,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LegendItem {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

impl From<&str> for LegendItem {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            icon: None,
        }
    }
}

impl From<String> for LegendItem {
    fn from(name: String) -> Self {
        Self { name, icon: None }
    }
}

impl From<(&str, &str)> for LegendItem {
    fn from((name, icon): (&str, &str)) -> Self {
        Self {
            name: name.to_string(),
            icon: Some(icon.into()),
        }
    }
}

impl From<(String, String)> for LegendItem {
    fn from((name, icon): (String, String)) -> Self {
        Self {
            name,
            icon: Some(icon.into()),
        }
    }
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
    left: Option<CompositeValue>,

    /// Distance between title component and the top side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<CompositeValue>,

    /// Distance between title component and the right side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<CompositeValue>,

    /// Distance between title component and the bottom side of the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<CompositeValue>,

    /// Width of legend component.
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<CompositeValue>,

    /// Height of legend component.
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<CompositeValue>,

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
    selected_mode: Option<LegendSelectedMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inactive_color: Option<Color>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<LegendItem>,
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
            border_color: None,
            inactive_color: None,
            data: vec![],
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

    pub fn selected_mode<S: Into<LegendSelectedMode>>(mut self, selected_mode: S) -> Self {
        self.selected_mode = Some(selected_mode.into());
        self
    }

    pub fn border_color<C: Into<Color>>(mut self, border_color: C) -> Self {
        self.border_color = Some(border_color.into());
        self
    }

    pub fn inactive_color<C: Into<Color>>(mut self, inactive_color: C) -> Self {
        self.inactive_color = Some(inactive_color.into());
        self
    }

    pub fn data<L: Into<LegendItem>>(mut self, data: Vec<L>) -> Self {
        self.data = data.into_iter().map(|s| s.into()).collect();
        self
    }
}
