use serde::Serialize;

use crate::basic::color::Color;

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

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
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

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl From<(&str, f64, f64)> for RadarIndicator {
    fn from((name, min, max): (&str, f64, f64)) -> Self {
        Self {
            name: Some(name.into()),
            min: Some(min),
            max: Some(max),
            color: None,
        }
    }
}

impl From<(&str, i64, i64)> for RadarIndicator {
    fn from((name, min, max): (&str, i64, i64)) -> Self {
        Self {
            name: Some(name.into()),
            min: Some(min as f64),
            max: Some(max as f64),
            color: None,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadarCoordinate {
    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_angle: Option<f64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    indicator: Vec<RadarIndicator>,
}

impl RadarCoordinate {
    pub fn new() -> Self {
        Self {
            center: None,
            radius: None,
            start_angle: None,
            indicator: vec![],
        }
    }

    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn radius<F: Into<f64>>(mut self, radius: F) -> Self {
        self.radius = Some(radius.into());
        self
    }

    pub fn start_angle<F: Into<f64>>(mut self, start_angle: F) -> Self {
        self.start_angle = Some(start_angle.into());
        self
    }

    pub fn indicator<I: Into<RadarIndicator>>(mut self, indicator: Vec<I>) -> Self {
        self.indicator = indicator.into_iter().map(|i| i.into()).collect();
        self
    }
}
