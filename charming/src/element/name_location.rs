use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NameLocation {
    Start,
    Middle,
    Center,
    End,
}
