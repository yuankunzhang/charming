use serde::{de::Visitor, Deserialize, Deserializer, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    Number(i32),
    Custom(String),
}
impl<S> From<S> for FontWeight
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        let s = s.into();
        match s.as_str() {
            "normal" => FontWeight::Normal,
            "bold" => FontWeight::Bold,
            "bolder" => FontWeight::Bolder,
            "lighter" => FontWeight::Lighter,
            _ => FontWeight::Custom(s),
        }
    }
}

impl Serialize for FontWeight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FontWeight::Normal => serializer.serialize_str("normal"),
            FontWeight::Bold => serializer.serialize_str("bold"),
            FontWeight::Bolder => serializer.serialize_str("bolder"),
            FontWeight::Lighter => serializer.serialize_str("lighter"),
            FontWeight::Number(num) => serializer.serialize_i32(*num),
            FontWeight::Custom(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FontWeight {
    fn deserialize<D>(deserializer: D) -> Result<FontWeight, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FontWeightVisitor;

        impl serde::de::Visitor<'_> for FontWeightVisitor {
            type Value = FontWeight;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string like 'normal' or an integer font weight")
            }

            fn visit_str<E>(self, value: &str) -> Result<FontWeight, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "normal" => Ok(FontWeight::Normal),
                    "bold" => Ok(FontWeight::Bold),
                    "bolder" => Ok(FontWeight::Bolder),
                    "lighter" => Ok(FontWeight::Lighter),
                    other => Ok(FontWeight::Custom(other.to_string())),
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<FontWeight, E>
            where
                E: serde::de::Error,
            {
                Ok(FontWeight::Number(value as i32))
            }

            fn visit_u64<E>(self, value: u64) -> Result<FontWeight, E>
            where
                E: serde::de::Error,
            {
                Ok(FontWeight::Number(value as i32))
            }
        }

        deserializer.deserialize_any(FontWeightVisitor)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum FontFamily {
    Serif,
    SansSerif,
    MonoSpace,
    Cursive,
    Fantasy,
    Custom(String),
}

impl<S> From<S> for FontFamily
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        let s = s.into();
        match s.as_str() {
            "serif" => FontFamily::Serif,
            "sans-serif" => FontFamily::SansSerif,
            "monospace" => FontFamily::MonoSpace,
            "cursive" => FontFamily::Cursive,
            "fantasy" => FontFamily::Fantasy,
            _ => FontFamily::Custom(s),
        }
    }
}

impl Serialize for FontFamily {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FontFamily::Serif => serializer.serialize_str("serif"),
            FontFamily::SansSerif => serializer.serialize_str("sans-serif"),
            FontFamily::MonoSpace => serializer.serialize_str("monospace"),
            FontFamily::Cursive => serializer.serialize_str("cursive"),
            FontFamily::Fantasy => serializer.serialize_str("fantasy"),
            FontFamily::Custom(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FontFamily {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FontFamilyVisitor;

        impl Visitor<'_> for FontFamilyVisitor {
            type Value = FontFamily;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a font family")
            }

            fn visit_str<E>(self, value: &str) -> Result<FontFamily, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "serif" => Ok(FontFamily::Serif),
                    "sans-serif" => Ok(FontFamily::SansSerif),
                    "monospace" => Ok(FontFamily::MonoSpace),
                    "cursive" => Ok(FontFamily::Cursive),
                    "fantasy" => Ok(FontFamily::Fantasy),
                    custom => Ok(FontFamily::Custom(custom.to_string())),
                }
            }
        }

        deserializer.deserialize_str(FontFamilyVisitor)
    }
}

#[cfg(test)]
#[test]
fn font_style() {
    let normal = serde_json::to_string(&FontStyle::Normal).unwrap();
    let italic = serde_json::to_string(&FontStyle::Italic).unwrap();
    let oblique = serde_json::to_string(&FontStyle::Oblique).unwrap();

    assert_eq!("\"normal\"", normal);
    assert_eq!("\"italic\"", italic);
    assert_eq!("\"oblique\"", oblique);
}

#[test]
fn font_weight() {
    let normal = serde_json::to_string(&FontWeight::Normal).unwrap();
    let bold = serde_json::to_string(&FontWeight::Bold).unwrap();
    let bolder = serde_json::to_string(&FontWeight::Bolder).unwrap();
    let lighter = serde_json::to_string(&FontWeight::Lighter).unwrap();
    let number = serde_json::to_string(&FontWeight::Number(100)).unwrap();
    let custom = serde_json::to_string(&FontWeight::Custom("test".to_string())).unwrap();

    assert_eq!("\"normal\"", normal);
    assert_eq!("\"bold\"", bold);
    assert_eq!("\"bolder\"", bolder);
    assert_eq!("\"lighter\"", lighter);
    assert_eq!("100", number);
    assert_eq!("\"test\"", custom);
}

#[test]
fn font_family() {
    let serif = serde_json::to_string(&FontFamily::Serif).unwrap();
    let sans_serif = serde_json::to_string(&FontFamily::SansSerif).unwrap();
    let monospace = serde_json::to_string(&FontFamily::MonoSpace).unwrap();
    let cursive = serde_json::to_string(&FontFamily::Cursive).unwrap();
    let fantasy = serde_json::to_string(&FontFamily::Fantasy).unwrap();
    let custom = serde_json::to_string(&FontFamily::Custom("test".to_string())).unwrap();

    assert_eq!("\"serif\"", serif);
    assert_eq!("\"sans-serif\"", sans_serif);
    assert_eq!("\"monospace\"", monospace);
    assert_eq!("\"cursive\"", cursive);
    assert_eq!("\"fantasy\"", fantasy);
    assert_eq!("\"test\"", custom);
}
