use serde::Serialize;

use super::item_style::ItemStyle;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EmphasisFocus {
    None,
    #[serde(rename = "self")]
    Self_,
    Series,
    Ancestor,
    Descendant,
    Relative,
    Adjacency,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Emphasis {
    #[serde(skip_serializing_if = "Option::is_none")]
    focus: Option<EmphasisFocus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_style: Option<ItemStyle>,
}

impl Emphasis {
    pub fn new() -> Self {
        Self {
            focus: None,
            item_style: None,
        }
    }

    pub fn focus(mut self, emphasis: EmphasisFocus) -> Self {
        self.focus = Some(emphasis);
        self
    }

    pub fn item_style(mut self, item_style: ItemStyle) -> Self {
        self.item_style = Some(item_style);
        self
    }
}
