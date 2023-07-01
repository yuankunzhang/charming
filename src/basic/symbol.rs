use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Symbol {
    Circle,
    Rect,
    RoundRect,
    Triangle,
    Diamond,
    Pin,
    Arrow,
    None,
}
