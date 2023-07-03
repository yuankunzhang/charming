use serde::ser::SerializeSeq;
use serde::Serialize;

use crate::datatype::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InputSelector {
    from_dataset_index: u64,
    from_transform_result: u64,
}

pub struct Dataset {
    source: Vec<Vec<Value>>,
    transforms: Vec<serde_json::Value>,
    input_selector: InputSelector,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            source: vec![],
            transforms: vec![],
            input_selector: InputSelector {
                from_dataset_index: 0,
                from_transform_result: 0,
            },
        }
    }

    pub fn source<V: Into<Value>>(mut self, source: Vec<Vec<V>>) -> Self {
        let source = source
            .into_iter()
            .map(|row| row.into_iter().map(|d| d.into()).collect())
            .collect();
        self.source = source;
        self
    }

    pub fn transform(mut self, transform: &str) -> Self {
        self.transforms
            .push(serde_json::from_str(transform).unwrap());
        self
    }

    pub fn from_dataset_index(mut self, from_dataset_index: u64) -> Self {
        self.input_selector.from_dataset_index = from_dataset_index;
        self
    }

    pub fn from_transform_result(mut self, from_transform_result: u64) -> Self {
        self.input_selector.from_transform_result = from_transform_result;
        self
    }
}

impl Serialize for Dataset {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct SourceHelper<'a> {
            source: &'a Vec<Vec<Value>>,
        }

        #[derive(Serialize)]
        struct TransformHelper<'a> {
            transform: &'a serde_json::Value,
        }

        let mut s = serializer.serialize_seq(Some(2 + self.transforms.len()))?;
        s.serialize_element(&SourceHelper {
            source: &self.source,
        })?;
        for transform in &self.transforms {
            s.serialize_element(&TransformHelper { transform })?;
        }
        s.serialize_element(&self.input_selector)?;
        s.end()
    }
}
