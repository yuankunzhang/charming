use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Smoothness {
    Single(f64),
    Boolean(bool),
}

impl Serialize for Smoothness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Smoothness::Single(smoothness) => serializer.serialize_f64(*smoothness),
            Smoothness::Boolean(smoothness) => serializer.serialize_bool(*smoothness),
        }
    }
}

impl<'de> Deserialize<'de> for Smoothness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SmoothnessVisitor;

        impl Visitor<'_> for SmoothnessVisitor {
            type Value = Smoothness;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a float or a boolean")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Smoothness, E>
            where
                E: de::Error,
            {
                Ok(Smoothness::Single(value))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Smoothness, E>
            where
                E: de::Error,
            {
                Ok(Smoothness::Boolean(value))
            }
        }

        deserializer.deserialize_any(SmoothnessVisitor)
    }
}

impl From<f64> for Smoothness {
    fn from(value: f64) -> Self {
        Smoothness::Single(value)
    }
}

impl From<f32> for Smoothness {
    fn from(value: f32) -> Self {
        Smoothness::Single(value as f64)
    }
}

impl From<i32> for Smoothness {
    fn from(value: i32) -> Self {
        Smoothness::Single(value as f64)
    }
}

impl From<u32> for Smoothness {
    fn from(value: u32) -> Self {
        Smoothness::Single(value as f64)
    }
}

impl From<bool> for Smoothness {
    fn from(value: bool) -> Self {
        Smoothness::Boolean(value)
    }
}
