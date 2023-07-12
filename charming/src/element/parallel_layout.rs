use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParallelLayout {
    Horizontal,
    Vertical,
}
