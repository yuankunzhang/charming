use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Sampling {
    #[default]
    Lttb,
    Average,
    Min,
    Max,
    Minmax,
    Sum,
}
