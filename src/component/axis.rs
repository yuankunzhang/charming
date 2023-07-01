use serde::Serialize;

use crate::basic::{split_area::SplitArea, split_line::SplitLine};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Value,
    Category,
    Time,
    Log,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>,
}

impl AxisLabel {
    pub fn new() -> Self {
        Self {
            interval: None,
            formatter: None,
        }
    }

    pub fn interval(mut self, interval: f64) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn formatter(mut self, formatter: &str) -> Self {
        self.formatter = Some(formatter.to_string());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Axis {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<Type>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label: Option<AxisLabel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_area: Option<SplitArea>,

    #[serde(skip_serializing_if = "Option::is_none")]
    split_line: Option<SplitLine>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<String>>,
}

impl Axis {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            boundary_gap: None,
            name_gap: None,
            axis_label: None,
            split_area: None,
            split_line: None,
            data: None,
        }
    }

    pub fn type_(mut self, type_: Type) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn boundary_gap(mut self, boundary_gap: bool) -> Self {
        self.boundary_gap = Some(boundary_gap);
        self
    }

    pub fn name_gap<F: Into<f64>>(mut self, name_gap: F) -> Self {
        self.name_gap = Some(name_gap.into());
        self
    }

    pub fn axis_label(mut self, axis_label: AxisLabel) -> Self {
        self.axis_label = Some(axis_label);
        self
    }

    pub fn split_area(mut self, split_area: SplitArea) -> Self {
        self.split_area = Some(split_area);
        self
    }

    pub fn split_line(mut self, split_line: SplitLine) -> Self {
        self.split_line = Some(split_line);
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        let data = data.into_iter().map(|s| s.into()).collect();
        self.data = Some(data);
        self
    }
}
