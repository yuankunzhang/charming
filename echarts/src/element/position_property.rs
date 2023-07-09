use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum PositionProperty {
    Absolute(f64),
    Relative(String),
}

impl From<f64> for PositionProperty {
    fn from(f: f64) -> Self {
        PositionProperty::Absolute(f)
    }
}

impl From<i64> for PositionProperty {
    fn from(i: i64) -> Self {
        PositionProperty::Absolute(i as f64)
    }
}

impl From<&str> for PositionProperty {
    fn from(s: &str) -> Self {
        PositionProperty::Relative(s.to_string())
    }
}

impl From<String> for PositionProperty {
    fn from(s: String) -> Self {
        PositionProperty::Relative(s)
    }
}
