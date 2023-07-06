use serde::Serialize;

use crate::element::Orient;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Value,
    Category,
    Time,
    Log,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleAxis {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<Type>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    orient: Option<Orient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inverse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<String>,
}

impl SingleAxis {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            width: None,
            height: None,
            orient: None,
            inverse: None,
            min: None,
            max: None,
        }
    }

    pub fn type_(mut self, type_: Type) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn left<S: Into<String>>(mut self, left: S) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<S: Into<String>>(mut self, top: S) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<S: Into<String>>(mut self, right: S) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<S: Into<String>>(mut self, bottom: S) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn width<S: Into<String>>(mut self, width: S) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<S: Into<String>>(mut self, height: S) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn orient(mut self, orient: Orient) -> Self {
        self.orient = Some(orient);
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }

    pub fn min<S: Into<String>>(mut self, min: S) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<S: Into<String>>(mut self, max: S) -> Self {
        self.max = Some(max.into());
        self
    }
}
