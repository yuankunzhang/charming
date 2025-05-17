use crate::element::js_function::JsFunction;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum AnimationTime {
    Number(f64),
    Function(JsFunction),
}

impl From<f64> for AnimationTime {
    fn from(f: f64) -> Self {
        AnimationTime::Number(f)
    }
}

impl From<JsFunction> for AnimationTime {
    fn from(f: JsFunction) -> Self {
        AnimationTime::Function(f)
    }
}
