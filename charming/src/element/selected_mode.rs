use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum SelectedMode {
    Boolean(bool),
    String(String),
}

impl From<bool> for SelectedMode {
    fn from(f: bool) -> Self {
        SelectedMode::Boolean(f)
    }
}

impl From<String> for SelectedMode {
    fn from(f: String) -> Self {
        SelectedMode::String(f)
    }
}
