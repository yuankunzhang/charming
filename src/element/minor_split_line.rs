use serde::Serialize;

use super::line_style::LineStyle;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MinorSplitLine {
    show: Option<bool>,
    line_style: Option<LineStyle>,
}

impl MinorSplitLine {
    pub fn new() -> MinorSplitLine {
        MinorSplitLine {
            show: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> MinorSplitLine {
        self.show = Some(show);
        self
    }

    pub fn line_style<F: Into<LineStyle>>(mut self, line_style: F) -> MinorSplitLine {
        self.line_style = Some(line_style.into());
        self
    }
}
