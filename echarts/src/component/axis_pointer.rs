use serde::Serialize;

use crate::element::{Label, LineStyle};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisPointerType {
    Line,
    Shadow,
    Cross,
    None,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisPointerAxis {
    X,
    Y,
    Radius,
    Angle,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointerLink {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    x_axis_index: Vec<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis_name: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    y_axis_index: Vec<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_name: Option<String>,
}

impl AxisPointerLink {
    pub fn new() -> Self {
        Self {
            x_axis_index: vec![],
            x_axis_name: None,
            y_axis_index: vec![],
            y_axis_name: None,
        }
    }

    pub fn x_axis_index<F: Into<f64>>(mut self, x_axis_index: Vec<F>) -> Self {
        self.x_axis_index = x_axis_index.into_iter().map(|f| f.into()).collect();
        self
    }

    pub fn x_axis_name<S: Into<String>>(mut self, x_axis_name: S) -> Self {
        self.x_axis_name = Some(x_axis_name.into());
        self
    }

    pub fn y_axis_index<F: Into<f64>>(mut self, y_axis_index: Vec<F>) -> Self {
        self.y_axis_index = y_axis_index.into_iter().map(|f| f.into()).collect();
        self
    }

    pub fn y_axis_name<S: Into<String>>(mut self, y_axis_name: S) -> Self {
        self.y_axis_name = Some(y_axis_name.into());
        self
    }
}

/// Axis Pointer is a tool for displaying reference line and axis value under
/// mouse pointer.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointer {
    /// Component ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    /// Whether to show the axis pointer.
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    /// Indicator type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<AxisPointerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    axis: Option<AxisPointerAxis>,

    /// Label of axis pointer.
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Label>,

    /// Line style of axis pointer.
    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    snap: Option<bool>,

    /// Axis pointer can be linked to each other.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    link: Vec<AxisPointerLink>,
}

impl AxisPointer {
    pub fn new() -> Self {
        Self {
            id: None,
            show: None,
            type_: None,
            axis: None,
            snap: None,
            label: None,
            line_style: None,
            link: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn type_<A: Into<AxisPointerType>>(mut self, type_: A) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn axis<A: Into<AxisPointerAxis>>(mut self, axis: A) -> Self {
        self.axis = Some(axis.into());
        self
    }

    pub fn snap(mut self, snap: bool) -> Self {
        self.snap = Some(snap);
        self
    }

    pub fn label<A: Into<Label>>(mut self, label: A) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn line_style<A: Into<LineStyle>>(mut self, line_style: A) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn link<A: Into<AxisPointerLink>>(mut self, link: Vec<A>) -> Self {
        self.link = link.into_iter().map(|a| a.into()).collect();
        self
    }
}
