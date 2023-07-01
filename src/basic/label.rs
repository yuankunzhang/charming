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
    rotate: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<(f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<Align>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align: Option<VerticalAlign>,
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
            align: None,
            vertical_align: None,
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

    pub fn rotate(mut self, rotate: f64) -> Self {
        self.rotate = Some(rotate);
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

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn vertical_align(mut self, vertical_align: VerticalAlign) -> Self {
        self.vertical_align = Some(vertical_align);
        self
    }
}
