use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CoordinateSystem {
    Cartesian2d,
    Polar,
    Single,
    Geo,
    Calendar,
    Parallel,
}
