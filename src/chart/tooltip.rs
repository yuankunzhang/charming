use serde::Serialize;

use crate::component::color::Color;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOn {
    Mousemove,
    Click,
    #[serde(rename = "mousemove|click")]
    MousemoveAndClick,
    None,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    Item,
    Axis,
    None,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisPointerType {
    Line,
    Shadow,
    Cross,
    None,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisPointerAxis {
    X,
    Y,
    Radius,
    Angle,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointerLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,
}

impl AxisPointerLabel {
    pub fn new() -> Self {
        Self {
            background_color: None,
        }
    }

    pub fn background_color(mut self, background_color: Color) -> Self {
        self.background_color = Some(background_color);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointer {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<AxisPointerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis: Option<AxisPointerAxis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<AxisPointerLabel>,
}

impl AxisPointer {
    pub fn new() -> Self {
        Self {
            type_: None,
            axis: None,
            label: None,
        }
    }

    pub fn type_(mut self, type_: AxisPointerType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn axis(mut self, axis: AxisPointerAxis) -> Self {
        self.axis = Some(axis);
        self
    }

    pub fn label(mut self, label: AxisPointerLabel) -> Self {
        self.label = Some(label);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<Trigger>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_on: Option<TriggerOn>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis_pointer: Option<AxisPointer>,
}

impl Tooltip {
    pub fn new() -> Self {
        Self {
            trigger: None,
            trigger_on: None,
            axis_pointer: None,
        }
    }

    pub fn trigger(mut self, trigger: Trigger) -> Self {
        self.trigger = Some(trigger);
        self
    }

    pub fn trigger_on(mut self, trigger_on: TriggerOn) -> Self {
        self.trigger_on = Some(trigger_on);
        self
    }

    pub fn axis_pointer(mut self, axis_pointer: AxisPointer) -> Self {
        self.axis_pointer = Some(axis_pointer);
        self
    }
}
