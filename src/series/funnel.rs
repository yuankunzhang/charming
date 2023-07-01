use serde::{Deserialize, Serialize};

use crate::utility::{color::ColorBy, label::Label, orient::Orient, sort::Sort};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    Left,
    Right,
    Center,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datum {
    value: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Datum {
    fn new<F: Into<f64>, S: Into<Option<String>>>(value: F, name: S) -> Self {
        Self {
            value: value.into(),
            name: name.into(),
        }
    }
}

impl From<f64> for Datum {
    fn from(value: f64) -> Self {
        Self::new(value, None)
    }
}

impl From<i32> for Datum {
    fn from(value: i32) -> Self {
        Self::new(value, None)
    }
}

impl From<(f64, &str)> for Datum {
    fn from((value, name): (f64, &str)) -> Self {
        Self::new(value, name.to_string())
    }
}

impl From<(i32, &str)> for Datum {
    fn from((value, name): (i32, &str)) -> Self {
        Self::new(value, name.to_string())
    }
}

pub type Data = Vec<Datum>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Funnel {
    #[serde(rename = "type")]
    type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color_by: Option<ColorBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    orient: Option<Orient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Sort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend_hover_link: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    funnel_align: Option<Align>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    data: Data,
}

impl Funnel {
    pub fn new() -> Self {
        Self {
            type_: "funnel".to_string(),
            name: None,
            color_by: None,
            min: None,
            max: None,
            min_size: None,
            max_size: None,
            width: None,
            height: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            orient: None,
            sort: None,
            gap: None,
            legend_hover_link: None,
            funnel_align: None,
            label: None,
            data: vec![],
        }
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn color_by(mut self, color_by: ColorBy) -> Self {
        self.color_by = Some(color_by);
        self
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn min_size<S: Into<String>>(mut self, min_size: S) -> Self {
        self.min_size = Some(min_size.into());
        self
    }

    pub fn max_size<S: Into<String>>(mut self, max_size: S) -> Self {
        self.max_size = Some(max_size.into());
        self
    }

    pub fn width<S: Into<String>>(mut self, width: S) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height<S: Into<String>>(mut self, height: S) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn left<S: Into<String>>(mut self, left: S) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<S: Into<String>>(mut self, top: S) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<S: Into<String>>(mut self, right: S) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<S: Into<String>>(mut self, bottom: S) -> Self {
        self.bottom = Some(bottom.into());
        self
    }

    pub fn orient(mut self, orient: Orient) -> Self {
        self.orient = Some(orient);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn gap<F: Into<f64>>(mut self, gap: F) -> Self {
        self.gap = Some(gap.into());
        self
    }

    pub fn legend_hover_link(mut self, legend_hover_link: bool) -> Self {
        self.legend_hover_link = Some(legend_hover_link);
        self
    }

    pub fn funnel_align(mut self, funnel_align: Align) -> Self {
        self.funnel_align = Some(funnel_align);
        self
    }

    pub fn label(mut self, label: Label) -> Self {
        self.label = Some(label);
        self
    }

    pub fn data<D: Into<Datum>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}
