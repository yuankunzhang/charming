use serde::{ser::SerializeSeq, Serialize};

use super::DataFrame;

#[derive(Debug, Serialize)]
pub struct Source<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    source: DataFrame<'a>,
}

impl<'a> Source<'a> {
    pub fn new(source: DataFrame<'a>) -> Self {
        Source { id: None, source }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_dataset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_dataset_index: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_transform_result: Option<i32>,
}

impl Transform {
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

    pub fn from_dataset_index<I: Into<i32>>(mut self, from_dataset_index: I) -> Self {
        self.from_dataset_index = Some(from_dataset_index.into());
        self
    }

    pub fn from_transform_result<I: Into<i32>>(mut self, from_transform_result: I) -> Self {
        self.from_transform_result = Some(from_transform_result.into());
        self
    }
}

impl From<&str> for Transform {
    fn from(transform: &str) -> Self {
        Self::new().transform(transform)
    }
}

pub struct Dataset<'a> {
    sources: Vec<Source<'a>>,
    transforms: Vec<Transform>,
}

impl<'a> Serialize for Dataset<'a> {
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
