use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BorderType {
    Solid,
    Dashed,
    Dotted,
}
