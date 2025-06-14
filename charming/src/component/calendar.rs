use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::CompositeValue,
    element::{CellSize, ItemStyle, Orient, Range, SplitLine},
};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    id: Option<String>,
    z_level: Option<f64>,
    z: Option<f64>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    range: Option<Range>,
    cell_size: Option<CellSize>,
    orient: Option<Orient>,
    split_line: Option<SplitLine>,
    item_style: Option<ItemStyle>,

    silent: Option<bool>,
}
