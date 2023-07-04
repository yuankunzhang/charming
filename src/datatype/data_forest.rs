use serde::{Deserialize, Serialize};

use crate::element::{item_style::ItemStyle, label::Label};

use super::DataPoint;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataNode {
    value: DataPoint,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<DataNode>,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,
}

impl DataNode {
    pub fn new(value: DataPoint) -> Self {
        Self {
            id: None,
            name: None,
            value,
            label: None,
            item_style: None,
            children: vec![],
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

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn children<D: Into<DataNode>>(mut self, children: Vec<D>) -> Self {
        self.children = children.into_iter().map(|d| d.into()).collect();
        self
    }
}

pub type DataForest = Vec<DataNode>;
