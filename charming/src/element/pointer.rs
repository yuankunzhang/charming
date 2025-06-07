use super::{icon::Icon, item_style::ItemStyle};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pointer {
    show: Option<bool>,
    show_above: Option<bool>,
    icon: Option<Icon>,
    #[charming_skip_setter]
    offset_center: Option<(String, String)>,
    length: Option<String>,
    width: Option<f64>,
    keep_aspect: Option<bool>,
    item_style: Option<ItemStyle>,
}

impl Pointer {
    pub fn offset_center<S: Into<String>>(mut self, offset_center: (S, S)) -> Self {
        self.offset_center = Some((offset_center.0.into(), offset_center.1.into()));
        self
    }
}
