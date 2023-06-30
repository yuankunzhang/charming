use serde::Serialize;

use crate::component::color::Color;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarIndicator {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
}

impl RadarIndicator {
    pub fn new() -> Self {
        Self {
            name: None,
            max: None,
            min: None,
            color: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn max(mut self, max: impl Into<f64>) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn min(mut self, min: impl Into<f64>) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Radar {
    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_angle: Option<f64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    indicator: Vec<RadarIndicator>,
}

impl Radar {
    pub fn new() -> Self {
        Self {
            center: None,
            radius: None,
            start_angle: None,
            indicator: vec![],
        }
    }

    pub fn center(mut self, center: (impl Into<String>, impl Into<String>)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn radius(mut self, radius: impl Into<f64>) -> Self {
        self.radius = Some(radius.into());
        self
    }

    pub fn start_angle(mut self, start_angle: impl Into<f64>) -> Self {
        self.start_angle = Some(start_angle.into());
        self
    }

    pub fn indicator(mut self, indicator: Vec<RadarIndicator>) -> Self {
        self.indicator = indicator;
        self
    }
}
