use super::{DataSource, Dimension};
use crate::element::RawString;
use charming_macros::CharmingSetters;
use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize};

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

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    id: Option<String>,
    transform: Option<RawString>,
    from_dataset_id: Option<String>,
    from_dataset_index: Option<i32>,
    from_transform_result: Option<i32>,
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
