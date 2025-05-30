use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, Copy, PartialEq, PartialOrd)]
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
