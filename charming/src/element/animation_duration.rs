use crate::element::js_function::JsFunction;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum AnimationDuration {
    Number(f64),
    Function(JsFunction),
}

impl From<f64> for AnimationDuration {
    fn from(f: f64) -> Self {
        AnimationDuration::Number(f)
    }
}

impl From<JsFunction> for AnimationDuration {
    fn from(f: JsFunction) -> Self {
        AnimationDuration::Function(f)
    }
}
