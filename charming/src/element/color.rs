use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ColorBy {
    Series,
    Data,
}

impl From<&str> for ColorBy {
    fn from(s: &str) -> Self {
        match s {
            "series" => Self::Series,
            "data" => Self::Data,
            _ => panic!("Invalid ColorBy"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct ColorStop {
    offset: f64,
    color: String,
}

impl ColorStop {
    pub fn new<F: Into<f64>, S: Into<String>>(offset: F, color: S) -> Self {
        Self {
            offset: offset.into(),
            color: color.into(),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum Color {
    Value(String),
    LinearGradient {
        x: f64,
        y: f64,
        x2: f64,
        y2: f64,
        color_stops: Vec<ColorStop>,
    },
    RadialGradient {
        x: f64,
        y: f64,
        r: f64,
        color_stops: Vec<ColorStop>,
    },
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Color::Value(rgb) => serializer.serialize_str(rgb),
            Color::LinearGradient {
                x,
                y,
                x2,
                y2,
                color_stops,
            } => {
                let mut s = serializer.serialize_struct("LinearGradient", 5)?;
                s.serialize_field("type", "linear")?;
                s.serialize_field("x", x)?;
                s.serialize_field("y", y)?;
                s.serialize_field("x2", x2)?;
                s.serialize_field("y2", y2)?;
                s.serialize_field("colorStops", color_stops)?;
                s.end()
            }
            Color::RadialGradient {
                x,
                y,
                r,
                color_stops,
            } => {
                let mut s = serializer.serialize_struct("RadialGradient", 4)?;
                s.serialize_field("type", "radial")?;
                s.serialize_field("x", x)?;
                s.serialize_field("y", y)?;
                s.serialize_field("r", r)?;
                s.serialize_field("colorStops", color_stops)?;
                s.end()
            }
        }
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        Color::Value(s.to_string())
    }
}

impl From<String> for Color {
    fn from(s: String) -> Self {
        Color::Value(s)
    }
}
