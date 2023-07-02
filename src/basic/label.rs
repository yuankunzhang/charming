use serde::Serialize;

use super::color::Color;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Position {
    Top,
    Left,
    Right,
    Bottom,
    Inside,
    InsideLeft,
    InsideRight,
    InsideTop,
    InsideBottom,
    InsideTopLeft,
    InsideBottomLeft,
    InsideTopRight,
    InsideBottomRight,
    Start,
    Outside,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rotate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<(f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<(f64, f64, f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<Align>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align: Option<VerticalAlign>,

    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
}

impl Label {
    pub fn new() -> Self {
        Self {
            show: None,
            position: None,
            distance: None,
            rotate: None,
            offset: None,
            formatter: None,
            color: None,
            font_size: None,
            padding: None,
            align: None,
            vertical_align: None,
            silent: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    pub fn distance(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self
    }

    pub fn rotate<S: Into<String>>(mut self, rotate: S) -> Self {
        self.rotate = Some(rotate.into());
        self
    }

    pub fn offset(mut self, offset: (f64, f64)) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn formatter<S: Into<String>>(mut self, formatter: S) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn font_size(mut self, font_size: f64) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn padding<F: Into<f64>>(mut self, padding: (F, F, F, F)) -> Self {
        self.padding = Some((
            padding.0.into(),
            padding.1.into(),
            padding.2.into(),
            padding.3.into(),
        ));
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn vertical_align(mut self, vertical_align: VerticalAlign) -> Self {
        self.vertical_align = Some(vertical_align);
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
        self
    }
}
