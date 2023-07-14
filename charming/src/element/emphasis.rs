use serde::Serialize;

use super::{item_style::ItemStyle, AreaStyle, Label};

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

    #[serde(skip_serializing_if = "Option::is_none")]
    area_style: Option<AreaStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,
}

impl Emphasis {
    pub fn new() -> Self {
        Self {
            focus: None,
            item_style: None,
            area_style: None,
            label: None,
        }
    }

    pub fn focus<E: Into<EmphasisFocus>>(mut self, emphasis: E) -> Self {
        self.focus = Some(emphasis.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn area_style<A: Into<AreaStyle>>(mut self, area_style: A) -> Self {
        self.area_style = Some(area_style.into());
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }
}
