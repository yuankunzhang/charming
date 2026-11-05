use super::{item_style::ItemStyle, label::Label};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Blur {
    line_style: Option<super::LineStyle>,
    edge_label: Option<Label>,
    z2: Option<f64>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
}
