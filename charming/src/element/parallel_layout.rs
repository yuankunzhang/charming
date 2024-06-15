use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParallelLayout {
    Horizontal,
    Vertical,
}
