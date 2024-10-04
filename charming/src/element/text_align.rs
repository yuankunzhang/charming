use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TextVerticalAlign {
    Auto,
    Top,
    Bottom,
    Middle,
}
