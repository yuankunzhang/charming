use serde::Serialize;

use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        Color, ColorBy, CoordinateSystem, Emphasis, ItemStyle, Label, LabelLayout, LabelLine,
        Symbol,
    },
};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    Ripple,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShowEffectOn {
    Render,
    Emphasis,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RippleEffectBrushType {
    Fill,
    Stroke,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RippleEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    brush_type: Option<RippleEffectBrushType>,
}

impl RippleEffect {
    pub fn new() -> Self {
        Self {
            color: None,
            number: None,
            period: None,
            scale: None,
            brush_type: None,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn number<F: Into<f64>>(mut self, number: F) -> Self {
        self.number = Some(number.into());
        self
    }

    pub fn period<F: Into<f64>>(mut self, period: F) -> Self {
        self.period = Some(period.into());
        self
    }

    pub fn scale<F: Into<f64>>(mut self, scale: F) -> Self {
        self.scale = Some(scale.into());
        self
    }

    pub fn brush_type<B: Into<RippleEffectBrushType>>(mut self, brush_type: B) -> Self {
        self.brush_type = Some(brush_type.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectScatter {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    effect_type: Option<EffectType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show_effect_on: Option<ShowEffectOn>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    polar_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    geo_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<Symbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_rotate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_keep_aspect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    symbol_offset: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label_line: Option<LabelLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label_layout: Option<LabelLayout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: DataFrame,
}

impl EffectScatter {
    pub fn new() -> Self {
        Self {
            type_: "effectScatter".to_string(),
            id: None,
            name: None,
            color_by: None,
            legend_hover_link: None,
            effect_type: None,
            show_effect_on: None,
            coordinate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            polar_index: None,
            geo_index: None,
            calendar_index: None,
            symbol: None,
            symbol_size: None,
            symbol_rotate: None,
            symbol_keep_aspect: None,
            symbol_offset: None,
            label: None,
            label_line: None,
            label_layout: None,
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

    pub fn color_by<C: Into<ColorBy>>(mut self, color_by: C) -> Self {
        self.color_by = Some(color_by.into());
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn effect_type<E: Into<EffectType>>(mut self, effect_type: E) -> Self {
        self.effect_type = Some(effect_type.into());
        self
    }

    pub fn show_effect_on<S: Into<ShowEffectOn>>(mut self, show_effect_on: S) -> Self {
        self.show_effect_on = Some(show_effect_on.into());
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

    pub fn polar_index<F: Into<f64>>(mut self, polar_index: F) -> Self {
        self.polar_index = Some(polar_index.into());
        self
    }

    pub fn geo_index<F: Into<f64>>(mut self, geo_index: F) -> Self {
        self.geo_index = Some(geo_index.into());
        self
    }

    pub fn calendar_index<F: Into<f64>>(mut self, calendar_index: F) -> Self {
        self.calendar_index = Some(calendar_index.into());
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

    pub fn symbol_rotate<F: Into<f64>>(mut self, symbol_rotate: F) -> Self {
        self.symbol_rotate = Some(symbol_rotate.into());
        self
    }

    pub fn symbol_keep_aspect(mut self, symbol_keep_aspect: bool) -> Self {
        self.symbol_keep_aspect = Some(symbol_keep_aspect);
        self
    }

    pub fn symbol_offset<S: Into<String>>(mut self, symbol_offset: (S, S)) -> Self {
        self.symbol_offset = Some((symbol_offset.0.into(), symbol_offset.1.into()));
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn label_line<L: Into<LabelLine>>(mut self, label_line: L) -> Self {
        self.label_line = Some(label_line.into());
        self
    }

    pub fn label_layout<L: Into<LabelLayout>>(mut self, label_layout: L) -> Self {
        self.label_layout = Some(label_layout.into());
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
