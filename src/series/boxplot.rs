use serde::Serialize;

use crate::element::{color::ColorBy, coordinate::CoordinateSystem};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Boxplot {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coordinate_system: Option<CoordinateSystem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hover_animation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_index: Option<u64>,
}

impl Boxplot {
    pub fn new() -> Boxplot {
        Boxplot {
            type_: String::from("boxplot"),
            name: None,
            coordinate_system: None,
            color_by: None,
            legend_hover_link: None,
            hover_animation: None,
            dataset_index: None,
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Boxplot {
        self.name = Some(name.into());
        self
    }

    pub fn coordinate_system(mut self, coordinate_system: CoordinateSystem) -> Boxplot {
        self.coordinate_system = Some(coordinate_system);
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Boxplot {
        self.color_by = Some(color_by);
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Boxplot {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn hover_animation(mut self, hover_animation: bool) -> Boxplot {
        self.hover_animation = Some(hover_animation);
        self
    }

    pub fn dataset_index(mut self, dataset_index: u64) -> Boxplot {
        self.dataset_index = Some(dataset_index);
        self
    }
}
