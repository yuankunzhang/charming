use crate::element::js_function::JsFunction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
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

impl From<f32> for AnimationTime {
    fn from(f: f32) -> Self {
        AnimationTime::Number(f as f64)
    }
}

impl From<i32> for AnimationTime {
    fn from(f: i32) -> Self {
        AnimationTime::Number(f as f64)
    }
}

impl From<i64> for AnimationTime {
    fn from(f: i64) -> Self {
        AnimationTime::Number(f as f64)
    }
}

impl From<JsFunction> for AnimationTime {
    fn from(f: JsFunction) -> Self {
        AnimationTime::Function(f)
    }
}
