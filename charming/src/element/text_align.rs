use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextVerticalAlign {
    Auto,
    Top,
    Bottom,
    Middle,
}
