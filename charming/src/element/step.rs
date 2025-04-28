use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Step {
    True,
    False,
    Start,
    Middle,
    End,
}

impl From<bool> for Step {
    fn from(value: bool) -> Self {
        if value {
            Step::True
        } else {
            Step::False
        }
    }
}

impl Serialize for Step {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Step::True => serializer.serialize_bool(true),
            Step::False => serializer.serialize_bool(false),
            Step::Start => serializer.serialize_str("start"),
            Step::Middle => serializer.serialize_str("middle"),
            Step::End => serializer.serialize_str("end"),
        }
    }
}

impl<'de> Deserialize<'de> for Step {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StepVisitor;

        impl Visitor<'_> for StepVisitor {
            type Value = Step;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "a boolean or a string containing either \"start\", \"middle\" or \"end\"",
                )
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    true => Ok(Step::True),
                    false => Ok(Step::False),
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "start" => Ok(Step::Start),
                    "middle" => Ok(Step::Middle),
                    "end" => Ok(Step::End),
                    _ => Err(E::custom("unable to parse step type, invalid string")),
                }
            }
        }
        deserializer.deserialize_any(StepVisitor)
    }
}
