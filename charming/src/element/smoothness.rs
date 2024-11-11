use serde::Serialize;

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
