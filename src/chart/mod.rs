use serde::Serialize;

pub mod axis;
pub mod grid;
pub mod legend;
pub mod title;
pub mod toolbox;
pub mod tooltip;

use crate::component::color::*;
use crate::series::*;
use axis::*;
use grid::*;
use legend::*;
use title::*;
use toolbox::*;
use tooltip::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip: Option<Tooltip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legend: Option<Legend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toolbox: Option<Toolbox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid: Option<Grid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis: Option<Axis>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    color: Vec<Color>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    series: Vec<Series>,
}

impl Chart {
    pub fn new() -> Self {
        Self {
            title: None,
            toolbox: None,
            legend: None,
            tooltip: None,
            grid: None,
            x_axis: None,
            y_axis: None,
            color: vec![],
            series: vec![],
        }
    }

    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    pub fn tooltip(mut self, tooltip: Tooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = Some(legend);
        self
    }

    pub fn toolbox(mut self, toolbox: Toolbox) -> Self {
        self.toolbox = Some(toolbox);
        self
    }

    pub fn grid(mut self, grid: Grid) -> Self {
        self.grid = Some(grid);
        self
    }

    pub fn x_axis(mut self, x_axis: Axis) -> Self {
        self.x_axis = Some(x_axis);
        self
    }

    pub fn y_axis(mut self, y_axis: Axis) -> Self {
        self.y_axis = Some(y_axis);
        self
    }

    pub fn color(mut self, color: Vec<Color>) -> Self {
        self.color = color;
        self
    }

    pub fn series(mut self, series: Series) -> Self {
        self.series.push(series);
        self
    }
}

impl ToString for Chart {
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
