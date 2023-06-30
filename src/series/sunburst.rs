use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
}

pub type Data = Vec<Node>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sunburst {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z_level: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<f64>,

    data: Data,
}

impl Sunburst {
    pub fn new() -> Self {
        Self {
            type_: "sunburst".to_string(),
            name: None,
            z_level: None,
            z: None,
            center: None,
            radius: None,
            data: vec![],
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn z_level(mut self, z_level: u64) -> Self {
        self.z_level = Some(z_level);
        self
    }

    pub fn z(mut self, z: u64) -> Self {
        self.z = Some(z);
        self
    }

    pub fn center(mut self, center: (impl Into<String>, impl Into<String>)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn data(mut self, data: Data) -> Self {
        self.data = data;
        self
    }
}
