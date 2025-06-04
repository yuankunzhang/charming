use serde::de::{self, Deserializer, MapAccess, Unexpected, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Try to parse as a simple string first
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or a gradient object with type field")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Color::Value(value.to_string()))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                use serde_json::Value as JsonValue;

                // Deserialize the map into a generic serde_json::Map first
                let mut temp_map = serde_json::Map::new();
                let mut type_value = None;
                while let Some((key, value)) = map.next_entry::<String, JsonValue>()? {
                    if key == "type" {
                        type_value = Some(value.clone())
                    }
                    temp_map.insert(key, value);
                }

                let type_value = type_value.ok_or_else(|| de::Error::missing_field("type"))?;

                let type_value = type_value.as_str().ok_or_else(|| {
                    de::Error::invalid_type(Unexpected::Other("non-string"), &"a string")
                })?;

                let temp_value = JsonValue::Object(temp_map);

                match type_value {
                    "linear" => {
                        #[derive(Deserialize)]
                        struct LinearGradientDef {
                            x: f64,
                            y: f64,
                            x2: f64,
                            y2: f64,
                            #[serde(rename = "colorStops")]
                            color_stops: Vec<ColorStop>,
                        }

                        let gradient: LinearGradientDef =
                            serde_json::from_value(temp_value).map_err(de::Error::custom)?;

                        Ok(Color::LinearGradient {
                            x: gradient.x,
                            y: gradient.y,
                            x2: gradient.x2,
                            y2: gradient.y2,
                            color_stops: gradient.color_stops,
                        })
                    }
                    "radial" => {
                        #[derive(Deserialize)]
                        struct RadialGradientDef {
                            x: f64,
                            y: f64,
                            r: f64,
                            #[serde(rename = "colorStops")]
                            color_stops: Vec<ColorStop>,
                        }

                        let gradient: RadialGradientDef =
                            serde_json::from_value(temp_value).map_err(de::Error::custom)?;

                        Ok(Color::RadialGradient {
                            x: gradient.x,
                            y: gradient.y,
                            r: gradient.r,
                            color_stops: gradient.color_stops,
                        })
                    }
                    other => Err(de::Error::unknown_variant(other, &["linear", "radial"])),
                }
            }
        }

        deserializer.deserialize_any(ColorVisitor)
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
