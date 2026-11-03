use super::{DataSource, Dimension};
use crate::element::JsExpression;
use charming_macros::CharmingSetters;
use serde::{Deserialize, Deserializer, Serialize, de::Visitor, ser::SerializeSeq};

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
    #[charming_skip_setter]
    transform: Option<JsExpression>,
    from_dataset_id: Option<String>,
    from_dataset_index: Option<i32>,
    from_transform_result: Option<i32>,
}

impl Transform {
    /// Set a trusted JavaScript object expression, saved with the `expression` tag.
    pub fn transform<S: Into<String>>(mut self, source: S) -> Self {
        self.transform = Some(JsExpression::from(source));
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
                while let Some(value) = seq.next_element::<serde_json::Value>()? {
                    if value.get("source").is_some() {
                        if !transforms.is_empty() {
                            return Err(serde::de::Error::custom(
                                "dataset sources must precede transforms",
                            ));
                        }
                        sources
                            .push(serde_json::from_value(value).map_err(serde::de::Error::custom)?);
                    } else {
                        transforms
                            .push(serde_json::from_value(value).map_err(serde::de::Error::custom)?);
                    }
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
