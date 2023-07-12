use serde::{Deserialize, Serialize};

use crate::element::{Emphasis, ItemStyle, Label, Sort};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SunburstLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    r0: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,
}

impl SunburstLevel {
    pub fn new() -> Self {
        Self {
            r0: None,
            r: None,
            item_style: None,
            label: None,
        }
    }

    pub fn r0<S: Into<String>>(mut self, r0: S) -> Self {
        self.r0 = Some(r0.into());
        self
    }

    pub fn r<S: Into<String>>(mut self, r: S) -> Self {
        self.r = Some(r.into());
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunburstNode {
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    item_style: Option<ItemStyle>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<SunburstNode>,
}

impl SunburstNode {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            value: None,
            item_style: None,
            children: vec![],
        }
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }

    pub fn children(mut self, children: Vec<SunburstNode>) -> Self {
        self.children = children;
        self
    }
}

impl From<&str> for SunburstNode {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<(&str, f64)> for SunburstNode {
    fn from((name, value): (&str, f64)) -> Self {
        Self::new(name).value(value)
    }
}

impl From<(&str, f64, &str)> for SunburstNode {
    fn from((name, value, color): (&str, f64, &str)) -> Self {
        Self::new(name)
            .value(value)
            .item_style(ItemStyle::new().color(color))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sunburst {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z_level: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    center: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<(String, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    emphasis: Option<Emphasis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    levels: Vec<SunburstLevel>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Vec<SunburstNode>,
}

impl Sunburst {
    pub fn new() -> Self {
        Self {
            type_: "sunburst".to_string(),
            id: None,
            name: None,
            z_level: None,
            z: None,
            center: None,
            radius: None,
            emphasis: None,
            sort: None,
            levels: vec![],
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
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

    pub fn center<S: Into<String>>(mut self, center: (S, S)) -> Self {
        self.center = Some((center.0.into(), center.1.into()));
        self
    }

    pub fn radius<S: Into<String>>(mut self, radius: (S, S)) -> Self {
        self.radius = Some((radius.0.into(), radius.1.into()));
        self
    }

    pub fn emphasis(mut self, emphasis: Emphasis) -> Self {
        self.emphasis = Some(emphasis);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn levels(mut self, levels: Vec<SunburstLevel>) -> Self {
        self.levels = levels;
        self
    }

    pub fn data(mut self, data: Vec<SunburstNode>) -> Self {
        self.data = data;
        self
    }
}
