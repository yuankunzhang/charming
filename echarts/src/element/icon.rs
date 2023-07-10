use serde::Serialize;

pub enum Icon {
    Circle,
    Rect,
    RoundRect,
    Triangle,
    Diamond,
    Pin,
    Arrow,
    None,
    Custom(String),
}

impl Serialize for Icon {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Icon::Circle => serializer.serialize_str("circle"),
            Icon::Rect => serializer.serialize_str("rect"),
            Icon::RoundRect => serializer.serialize_str("roundRect"),
            Icon::Triangle => serializer.serialize_str("triangle"),
            Icon::Diamond => serializer.serialize_str("diamond"),
            Icon::Pin => serializer.serialize_str("pin"),
            Icon::Arrow => serializer.serialize_str("arrow"),
            Icon::None => serializer.serialize_str("none"),
            Icon::Custom(s) => serializer.serialize_str(s),
        }
    }
}

impl<S> From<S> for Icon
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        let s = s.into();
        match s.as_str() {
            "circle" => Icon::Circle,
            "rect" => Icon::Rect,
            "roundRect" => Icon::RoundRect,
            "triangle" => Icon::Triangle,
            "diamond" => Icon::Diamond,
            "pin" => Icon::Pin,
            "arrow" => Icon::Arrow,
            "none" => Icon::None,
            _ => Icon::Custom(s),
        }
    }
}
