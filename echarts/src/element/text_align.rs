use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextVerticalAlign {
    Auto,
    Top,
    Bottom,
    Middle,
}
