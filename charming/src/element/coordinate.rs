use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CoordinateSystem {
    Cartesian2d,
    Polar,
    Single,
    #[serde(rename = "singleAxis")]
    SingleAxis,
    Geo,
    Calendar,
    Matrix,
    Radar,
    None,
    View,
    Parallel,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum CoordinateSystemUsage {
    Data,
    Box,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Auto {
    Auto,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(untagged)]
pub enum BooleanOrAuto {
    Bool(bool),
    Auto(Auto),
}

impl From<bool> for BooleanOrAuto {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Auto> for BooleanOrAuto {
    fn from(value: Auto) -> Self {
        Self::Auto(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RoamMode {
    Pan,
    Move,
    Zoom,
    Scale,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(untagged)]
pub enum Roam {
    Bool(bool),
    Mode(RoamMode),
}

impl From<bool> for Roam {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<RoamMode> for Roam {
    fn from(value: RoamMode) -> Self {
        Self::Mode(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RoamTrigger {
    Global,
    SelfRect,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum PreserveAspectMode {
    Contain,
    Cover,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(untagged)]
pub enum PreserveAspect {
    Bool(bool),
    Mode(PreserveAspectMode),
}

impl From<bool> for PreserveAspect {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<PreserveAspectMode> for PreserveAspect {
    fn from(value: PreserveAspectMode) -> Self {
        Self::Mode(value)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum StackOrder {
    SeriesAsc,
    SeriesDesc,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MarkerRelativeTo {
    Container,
    Coordinate,
}
