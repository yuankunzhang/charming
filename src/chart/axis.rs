use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisType {
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
    type_: Option<AxisType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boundary_gap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    axis_label: Option<AxisLabel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<String>>,
}

impl Axis {
    pub fn new() -> Self {
        Self {
            type_: None,
            boundary_gap: None,
            axis_label: None,
            data: None,
        }
    }

    pub fn type_(mut self, type_: AxisType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn boundary_gap(mut self, boundary_gap: bool) -> Self {
        self.boundary_gap = Some(boundary_gap);
        self
    }

    pub fn axis_label(mut self, axis_label: AxisLabel) -> Self {
        self.axis_label = Some(axis_label);
        self
    }

    pub fn data<S: Into<String>>(mut self, data: Vec<S>) -> Self {
        let data = data.into_iter().map(|s| s.into()).collect();
        self.data = Some(data);
        self
    }
}
