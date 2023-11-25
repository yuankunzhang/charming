use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BorderType {
    Solid,
    Dashed,
    Dotted,
}
