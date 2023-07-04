use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parallel {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zlevel: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,
}
