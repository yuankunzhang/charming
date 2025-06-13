use super::{blur::Blur, emphasis::Emphasis, item_style::ItemStyle, label::Label};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkAreaData {
    name: Option<String>,
    x_axis: Option<String>,
    y_axis: Option<String>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkArea {
    silent: Option<bool>,
    label: Option<Label>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    blur: Option<Blur>,
    #[charming_skip_setter]
    data: Vec<(MarkAreaData, MarkAreaData)>,
}

impl MarkArea {
    pub fn data<D: Into<MarkAreaData>>(mut self, data: Vec<(D, D)>) -> Self {
        self.data = data
            .into_iter()
            .map(|(d1, d2)| (d1.into(), d2.into()))
            .collect();
        self
    }
}
