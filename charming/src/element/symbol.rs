use super::JsFunction;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Symbol {
    Circle,
    Rect,
    RoundRect,
    Triangle,
    Diamond,
    Pin,
    Arrow,
    None,
    Custom(String),
    Callback(JsFunction),
}

impl Serialize for Symbol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Symbol::Circle => serializer.serialize_str("circle"),
            Symbol::Rect => serializer.serialize_str("rect"),
            Symbol::RoundRect => serializer.serialize_str("roundRect"),
            Symbol::Triangle => serializer.serialize_str("triangle"),
            Symbol::Diamond => serializer.serialize_str("diamond"),
            Symbol::Pin => serializer.serialize_str("pin"),
            Symbol::Arrow => serializer.serialize_str("arrow"),
            Symbol::None => serializer.serialize_str("none"),
            Symbol::Custom(s) => serializer.serialize_str(s),
            Symbol::Callback(s) => s.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Symbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SymbolVisitor;

        impl<'de> Visitor<'de> for SymbolVisitor {
            type Value = Symbol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a structured RawString for callbacks")
            }

            fn visit_str<E>(self, value: &str) -> Result<Symbol, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "circle" => Ok(Symbol::Circle),
                    "rect" => Ok(Symbol::Rect),
                    "roundRect" => Ok(Symbol::RoundRect),
                    "triangle" => Ok(Symbol::Triangle),
                    "diamond" => Ok(Symbol::Diamond),
                    "pin" => Ok(Symbol::Pin),
                    "arrow" => Ok(Symbol::Arrow),
                    "none" => Ok(Symbol::None),
                    custom => Ok(Symbol::Custom(custom.to_string())),
                }
            }

            fn visit_map<M>(self, map: M) -> Result<Symbol, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let js_function =
                    JsFunction::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;
                Ok(Symbol::Callback(js_function))
            }
        }

        deserializer.deserialize_any(SymbolVisitor)
    }
}
