use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize};

use crate::element::RawString;

use super::{DataSource, Dimension};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct Source {
    source: DataSource,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    dimensions: Vec<Dimension>,
}

impl Source {
    pub fn new(source: DataSource) -> Self {
        Source {
            id: None,
            source,
            dimensions: vec![],
        }
    }

    pub fn new_with_id(source: DataSource, id: String) -> Self {
        Source {
            id: Some(id),
            source,
            dimensions: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn dimensions<D: Into<Dimension>>(mut self, dimensions: Vec<D>) -> Self {
        self.dimensions = dimensions.into_iter().map(|d| d.into()).collect();
        self
    }
}

impl<D> From<D> for Source
where
    D: Into<DataSource>,
{
    fn from(source: D) -> Self {
        Self::new(source.into())
    }
}

impl<D, F> From<(D, F)> for Source
where
    D: Into<DataSource>,
    F: Into<String>,
{
    fn from((source, id): (D, F)) -> Self {
        Self::new(source.into()).id(id.into())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transform: Option<RawString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_dataset_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_dataset_index: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    from_transform_result: Option<i32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn transform<R: Into<RawString>>(mut self, transform: R) -> Self {
        self.transform = Some(transform.into());
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Dataset {
    sources: Vec<Source>,
    transforms: Vec<Transform>,
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

impl<'de> Deserialize<'de> for Dataset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetVisitor;

        impl<'de> Visitor<'de> for DatasetVisitor {
            type Value = Dataset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of Sources followed by Transforms")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Dataset, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let mut sources = Vec::new();
                let mut transforms = Vec::new();
                let mut parsing_sources = true; // Initially, we are parsing Sources

                // Attempt to parse elements as `Source` until it fails
                while parsing_sources {
                    match seq.next_element::<Source>() {
                        Ok(Some(source)) => sources.push(source),
                        Ok(None) => {
                            // End of sequence
                            return Ok(Dataset {
                                sources,
                                transforms,
                            });
                        }
                        Err(_) => {
                            parsing_sources = false; // Switch to parsing Transforms
                        }
                    }
                }

                // Continue parsing the rest as `Transform`
                while let Some(transform) = seq.next_element::<Transform>()? {
                    transforms.push(transform);
                }

                Ok(Dataset {
                    sources,
                    transforms,
                })
            }
        }

        deserializer.deserialize_seq(DatasetVisitor)
    }
}

impl Default for Dataset {
    fn default() -> Self {
        Self::new()
    }
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            sources: vec![],
            transforms: vec![],
        }
    }

    pub fn source<S: Into<Source>>(mut self, source: S) -> Self {
        self.sources.push(source.into());
        self
    }

    pub fn transform<T: Into<Transform>>(mut self, transform: T) -> Self {
        self.transforms.push(transform.into());
        self
    }
}
