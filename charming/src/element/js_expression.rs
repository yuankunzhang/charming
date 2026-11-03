use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct};

pub(crate) const EXPRESSION_SERDE_NAME: &str = "__charming_js_expression";

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub(crate) struct JsExpression(String);

impl<S: Into<String>> From<S> for JsExpression {
    fn from(source: S) -> Self {
        Self(source.into())
    }
}

impl Serialize for JsExpression {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut value = serializer.serialize_struct(EXPRESSION_SERDE_NAME, 2)?;
        value.serialize_field("type", "expression")?;
        value.serialize_field("source", &self.0)?;
        value.end()
    }
}

impl<'de> Deserialize<'de> for JsExpression {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "type")]
        enum Source {
            #[serde(rename = "expression")]
            Expression { source: String },
        }
        let Source::Expression { source } = Source::deserialize(deserializer)?;
        Ok(Self(source))
    }
}
