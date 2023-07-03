use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinateSystem {
    Cartesian2d,
    Polar,
    Single,
    Geo,
    Calendar,
    Parallel,
}
