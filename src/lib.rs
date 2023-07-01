use component::dataset::Dataset;
use serde::Serialize;

pub mod basic;
pub mod component;
pub mod renderer;
pub mod series;
pub mod style;

use basic::color::Color;
use component::axis::Axis;
use component::grid::Grid;
use component::legend::Legend;
use component::radar_coordinate::RadarCoordinate;
use component::title::Title;
use component::toolbox::Toolbox;
use component::tooltip::Tooltip;
use series::Series;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "title")]
    titles: Vec<Title>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<Dataset>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "radar")]
    radars: Vec<RadarCoordinate>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    color: Vec<Color>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    series: Vec<Series>,
}

impl Chart {
    pub fn new() -> Self {
        Self {
            titles: vec![],
            toolbox: None,
            legend: None,
            tooltip: None,
            grid: None,
            x_axis: None,
            y_axis: None,
            dataset: None,
            radars: vec![],
            color: vec![],
            series: vec![],
        }
    }

    pub fn title(mut self, title: Title) -> Self {
        self.titles.push(title);
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

    pub fn dataset(mut self, dataset: Dataset) -> Self {
        self.dataset = Some(dataset);
        self
    }

    pub fn radars(mut self, radars: Vec<RadarCoordinate>) -> Self {
        self.radars = radars;
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
