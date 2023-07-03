use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkPointDataType {
    Min,
    Max,
    Average,
}

impl From<&str> for MarkPointDataType {
    fn from(s: &str) -> Self {
        match s {
            "min" => Self::Min,
            "max" => Self::Max,
            "avg" | "average" => Self::Average,
            _ => panic!("Invalid MarkPointDataType"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPointData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<MarkPointDataType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<f64>,
}

impl MarkPointData {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            x_axis: None,
            y_axis: None,
            value: None,
        }
    }

    pub fn type_<T: Into<MarkPointDataType>>(mut self, type_: T) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn x_axis<F: Into<f64>>(mut self, x_axis: F) -> Self {
        self.x_axis = Some(x_axis.into());
        self
    }

    pub fn y_axis<F: Into<f64>>(mut self, y_axis: F) -> Self {
        self.y_axis = Some(y_axis.into());
        self
    }

    pub fn value<F: Into<f64>>(mut self, value: F) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl From<(&str, &str)> for MarkPointData {
    fn from((type_, name): (&str, &str)) -> Self {
        Self::new().type_(type_).name(name)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPoint {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<MarkPointData>,
}

impl MarkPoint {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn data<D: Into<MarkPointData>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
