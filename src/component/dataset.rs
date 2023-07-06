use serde::ser::SerializeSeq;
use serde::Serialize;

use crate::datatype::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    source: Vec<Vec<Value>>,
}

impl DatasetSource {
    pub fn new() -> Self {
        Self {
            id: None,
            source: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn source<V: Into<Value>>(mut self, source: Vec<Vec<V>>) -> Self {
        let source = source
            .into_iter()
            .map(|row| row.into_iter().map(|d| d.into()).collect())
            .collect();
        self.source = source;
        self
    }
}

impl<V> From<Vec<Vec<V>>> for DatasetSource
where
    V: Into<Value>,
{
    fn from(source: Vec<Vec<V>>) -> Self {
        Self::new().source(source)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetTransform {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_dataset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_dataset_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_transform_result: Option<f64>,
}

impl DatasetTransform {
    pub fn new() -> Self {
        Self {
            id: None,
            transform: None,
            from_dataset_id: None,
            from_dataset_index: None,
            from_transform_result: None,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn transform(mut self, transform: &str) -> Self {
        self.transform = Some(serde_json::from_str(transform).unwrap());
        self
    }

    pub fn from_dataset_id<S: Into<String>>(mut self, from_dataset_id: S) -> Self {
        self.from_dataset_id = Some(from_dataset_id.into());
        self
    }

    pub fn from_dataset_index<F: Into<f64>>(mut self, from_dataset_index: F) -> Self {
        self.from_dataset_index = Some(from_dataset_index.into());
        self
    }

    pub fn from_transform_result<F: Into<f64>>(mut self, from_transform_result: F) -> Self {
        self.from_transform_result = Some(from_transform_result.into());
        self
    }
}

impl From<&str> for DatasetTransform {
    fn from(transform: &str) -> Self {
        Self::new().transform(transform)
    }
}

pub struct Dataset {
    sources: Vec<DatasetSource>,
    transforms: Vec<DatasetTransform>,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            sources: vec![],
            transforms: vec![],
        }
    }

    pub fn source<S: Into<DatasetSource>>(mut self, source: S) -> Self {
        self.sources.push(source.into());
        self
    }

    pub fn transform<T: Into<DatasetTransform>>(mut self, transform: T) -> Self {
        self.transforms.push(transform.into());
        self
    }
}

impl Serialize for Dataset {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_seq(Some(self.sources.len() + self.transforms.len()))?;
        for source in &self.sources {
            s.serialize_element(&source)?;
        }
        for transform in &self.transforms {
            s.serialize_element(&transform)?;
        }
        s.end()
    }
}
