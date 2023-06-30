use serde::Serialize;

use crate::component::color::*;
use crate::component::coordinate::*;
use crate::component::data::*;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorBy {
    Series,
    Data,
}

impl From<&str> for ColorBy {
    fn from(s: &str) -> Self {
        match s {
            "series" => Self::Series,
            "data" => Self::Data,
            _ => panic!("Invalid ColorBy"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BorderType {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_width: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_type: Option<BorderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border_radius: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
}

impl BackgroundStyle {
    pub fn new() -> Self {
        Self {
            color: None,
            border_color: None,
            border_width: None,
            border_type: None,
            border_radius: None,
            opacity: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn border_width(mut self, border_width: u64) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = Some(border_type);
        self
    }

    pub fn border_radius(mut self, border_radius: u64) -> Self {
        self.border_radius = Some(border_radius);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bar {
    #[serde(rename = "type")]
    type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coordiate_system: Option<CoordinateSystem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polar_index: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    round_cap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_sort: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_style: Option<BackgroundStyle>,
    data: DataFrame,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            type_: "bar".to_string(),
            name: None,
            color_by: None,
            legend_hover_link: None,
            coordiate_system: None,
            x_axis_index: None,
            y_axis_index: None,
            polar_index: None,
            round_cap: None,
            realtime_sort: None,
            show_background: None,
            background_style: None,
            data: DataFrame::new(),
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn coordiate_system(mut self, coordiate_system: CoordinateSystem) -> Self {
        self.coordiate_system = Some(coordiate_system);
        self
    }

    pub fn x_axis_index(mut self, x_axis_index: u64) -> Self {
        self.x_axis_index = Some(x_axis_index);
        self
    }

    pub fn y_axis_index(mut self, y_axis_index: u64) -> Self {
        self.y_axis_index = Some(y_axis_index);
        self
    }

    pub fn polar_index(mut self, polar_index: u64) -> Self {
        self.polar_index = Some(polar_index);
        self
    }

    pub fn round_cap(mut self, round_cap: bool) -> Self {
        self.round_cap = Some(round_cap);
        self
    }

    pub fn realtime_sort(mut self, realtime_sort: bool) -> Self {
        self.realtime_sort = Some(realtime_sort);
        self
    }

    pub fn show_background(mut self, show_background: bool) -> Self {
        self.show_background = Some(show_background);
        self
    }

    pub fn background_style(mut self, background_style: BackgroundStyle) -> Self {
        self.background_style = Some(background_style);
        self
    }

    pub fn data(mut self, data: DataFrameOneDimension) -> Self {
        for (i, d) in data.into_iter().enumerate() {
            self.data.push(vec![(i as f64).into(), d.into()]);
        }
        self
    }

    pub fn dataframe(mut self, data: DataFrame) -> Self {
        self.data = data;
        self
    }
}
