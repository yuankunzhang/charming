use serde::{Deserialize, Serialize};

use crate::element::{color::ColorBy, coordinate::CoordinateSystem, item_style::ItemStyle};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RoseType {
    Radius,
    Area,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPoint {
    pub value: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub type Data = Vec<DataPoint>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pie {
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
    geo_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_index: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected_mode: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selected_offset: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clockwise: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_angle: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rose_type: Option<RoseType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<(String, String)>,

    data: Data,
}

impl Pie {
    pub fn new() -> Pie {
        Pie {
            type_: "pie".to_string(),
            name: None,
            color_by: None,
            legend_hover_link: None,
            coordiate_system: None,
            geo_index: None,
            calendar_index: None,
            selected_mode: None,
            selected_offset: None,
            clockwise: None,
            start_angle: None,
            rose_type: None,
            item_style: None,
            center: None,
            radius: None,
            data: vec![],
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

    pub fn geo_index(mut self, geo_index: u64) -> Self {
        self.geo_index = Some(geo_index);
        self
    }

    pub fn calendar_index(mut self, calendar_index: u64) -> Self {
        self.calendar_index = Some(calendar_index);
        self
    }

    pub fn selected_mode(mut self, selected_mode: bool) -> Self {
        self.selected_mode = Some(selected_mode);
        self
    }

    pub fn selected_offset(mut self, selected_offset: f64) -> Self {
        self.selected_offset = Some(selected_offset);
        self
    }

    pub fn clockwise(mut self, clockwise: bool) -> Self {
        self.clockwise = Some(clockwise);
        self
    }

    pub fn start_angle<F: Into<f64>>(mut self, start_angle: F) -> Self {
        self.start_angle = Some(start_angle.into());
        self
    }

    pub fn rose_type(mut self, rose_type: RoseType) -> Self {
        self.rose_type = Some(rose_type);
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }

    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn radius<S: Into<String>>(mut self, radius: (S, S)) -> Self {
        self.radius = Some((radius.0.into(), radius.1.into()));
        self
    }

    pub fn data(mut self, data: Data) -> Self {
        self.data = data;
        self
    }
}
