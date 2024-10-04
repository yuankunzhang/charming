use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NameLocation {
    Start,
    Middle,
    Center,
    End,
}
