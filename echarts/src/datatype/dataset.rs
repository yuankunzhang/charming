use serde::{ser::SerializeSeq, Serialize};

use super::{DataSource, Dimension};

#[derive(Debug, Serialize)]
pub struct DataSourceContainer {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    source: DataSource,
}

impl DataSourceContainer {
    pub fn new(source: DataSource) -> Self {
        DataSourceContainer { id: None, source }
    }

    pub fn new_with_id(source: DataSource, id: String) -> Self {
        DataSourceContainer {
            id: Some(id),
            source,
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a, D> From<D> for DataSourceContainer
where
    D: Into<DataSource>,
{
    fn from(source: D) -> Self {
        Self::new(source.into())
    }
}

impl<'a, D, F> From<(D, F)> for DataSourceContainer
where
    D: Into<DataSource>,
    F: Into<String>,
{
    fn from((source, id): (D, F)) -> Self {
        Self::new(source.into()).id(id.into())
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

pub struct Dataset {
    sources: Vec<DataSourceContainer>,
    transforms: Vec<Transform>,
    dimensions: Vec<Dimension>,
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

impl Dataset {
    pub fn new() -> Self {
        Self {
            sources: vec![],
            transforms: vec![],
            dimensions: vec![],
        }
    }

    pub fn source<S: Into<DataSourceContainer>>(mut self, source: S) -> Self {
        self.sources.push(source.into());
        self
    }

    pub fn transform<T: Into<Transform>>(mut self, transform: T) -> Self {
        self.transforms.push(transform.into());
        self
    }

    pub fn dimensions<D: Into<Dimension>>(mut self, dimensions: Vec<D>) -> Self {
        self.dimensions = dimensions.into_iter().map(|d| d.into()).collect();
        self
    }
}
