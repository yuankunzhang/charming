use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

use crate::{
    datatype::CompositeValue,
    element::{CellSize, Color, Formatter, ItemStyle, LabelPosition, Orient, Range, SplitLine},
};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    day_label: Option<CalendarLabel>,
    month_label: Option<CalendarLabel>,
    year_label: Option<CalendarLabel>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum CalendarNameMap {
    Locale(String),
    Names(Vec<String>),
}

impl From<&str> for CalendarNameMap {
    fn from(value: &str) -> Self {
        Self::Locale(value.to_owned())
    }
}

impl<S: Into<String>> From<Vec<S>> for CalendarNameMap {
    fn from(value: Vec<S>) -> Self {
        Self::Names(value.into_iter().map(Into::into).collect())
    }
}

#[serde_with::apply(Option => #[serde(skip_serializing_if = "Option::is_none")])]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CalendarLabel {
    show: Option<bool>,
    silent: Option<bool>,
    position: Option<LabelPosition>,
    margin: Option<crate::datatype::StringOrNumber>,
    first_day: Option<usize>,
    name_map: Option<CalendarNameMap>,
    formatter: Option<Formatter>,
    color: Option<Color>,
    font_size: Option<crate::datatype::StringOrNumber>,
    font_style: Option<crate::element::font_settings::FontStyle>,
    font_weight: Option<crate::element::font_settings::FontWeight>,
    font_family: Option<crate::element::font_settings::FontFamily>,
    align: Option<crate::element::LabelAlign>,
    vertical_align: Option<crate::element::LabelVerticalAlign>,
    rotate: Option<f64>,
    rich: Option<std::collections::HashMap<String, crate::element::TextStyle>>,
    rich_inherit_plain_label: Option<bool>,
    min_margin: Option<f64>,
    text_margin: Option<crate::datatype::NumberOrArray>,
    line_overflow: Option<crate::element::LineOverflow>,
}
