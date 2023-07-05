pub mod component;
pub mod datatype;
pub mod element;
pub mod renderer;
pub mod series;
pub mod style;

pub use renderer::*;

use component::angle_axis::AngleAxis;
use component::data_zoom::DataZoom;
use component::dataset::Dataset;
use component::parallel_axis::ParallelAxis;
use component::parallel_coordinate::ParallelCoordinate;
use component::polar::Polar;
use component::radius_axis::RadiusAxis;
use component::visual_map::VisualMap;
use element::mark_line::MarkLine;
use element::single_axis::SingleAxis;
use serde::Serialize;

use component::axis::Axis;
use component::grid::Grid;
use component::legend::Legend;
use component::radar_coordinate::RadarCoordinate;
use component::title::Title;
use component::toolbox::Toolbox;
use component::tooltip::Tooltip;
use element::color::Color;
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

    #[serde(skip_serializing_if = "Vec::is_empty")]
    grid: Vec<Grid>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    x_axis: Vec<Axis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    y_axis: Vec<Axis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    polar: Vec<Polar>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    angle_axis: Vec<AngleAxis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    radius_axis: Vec<RadiusAxis>,

    #[serde(skip_serializing_if = "Option::is_none")]
    single_axis: Option<SingleAxis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    parallel_axis: Vec<ParallelAxis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    visual_map: Vec<VisualMap>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    data_zoom: Vec<DataZoom>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parallel: Option<ParallelCoordinate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<Dataset>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    radar: Vec<RadarCoordinate>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    color: Vec<Color>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    series: Vec<Series>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mark_line: Option<MarkLine>,
}

impl Chart {
    pub fn new() -> Self {
        Self {
            titles: vec![],
            toolbox: None,
            legend: None,
            tooltip: None,
            grid: vec![],
            x_axis: vec![],
            y_axis: vec![],
            polar: vec![],
            angle_axis: vec![],
            radius_axis: vec![],
            single_axis: None,
            parallel_axis: vec![],
            visual_map: vec![],
            data_zoom: vec![],
            parallel: None,
            dataset: None,
            radar: vec![],
            color: vec![],
            series: vec![],
            background_color: None,
            mark_line: None,
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
        self.grid.push(grid);
        self
    }

    pub fn x_axis(mut self, x_axis: Axis) -> Self {
        self.x_axis.push(x_axis);
        self
    }

    pub fn y_axis(mut self, y_axis: Axis) -> Self {
        self.y_axis.push(y_axis);
        self
    }

    pub fn polar(mut self, polar: Polar) -> Self {
        self.polar.push(polar);
        self
    }

    pub fn angle_axis(mut self, angle_axis: AngleAxis) -> Self {
        self.angle_axis.push(angle_axis);
        self
    }

    pub fn radius_axis(mut self, radius_axis: RadiusAxis) -> Self {
        self.radius_axis.push(radius_axis);
        self
    }

    pub fn single_axis(mut self, single_axis: SingleAxis) -> Self {
        self.single_axis = Some(single_axis);
        self
    }

    pub fn parallel_axis(mut self, parallel_axis: ParallelAxis) -> Self {
        self.parallel_axis.push(parallel_axis);
        self
    }

    pub fn visual_map(mut self, visual_map: VisualMap) -> Self {
        self.visual_map.push(visual_map);
        self
    }

    pub fn data_zoom(mut self, data_zoom: DataZoom) -> Self {
        self.data_zoom.push(data_zoom);
        self
    }

    pub fn parallel(mut self, parallel: ParallelCoordinate) -> Self {
        self.parallel = Some(parallel);
        self
    }

    pub fn dataset(mut self, dataset: Dataset) -> Self {
        self.dataset = Some(dataset);
        self
    }

    pub fn radar(mut self, radars: Vec<RadarCoordinate>) -> Self {
        self.radar = radars;
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

    pub fn background_color<C: Into<Color>>(mut self, color: C) -> Self {
        self.background_color = Some(color.into());
        self
    }

    pub fn mark_line(mut self, mark_line: MarkLine) -> Self {
        self.mark_line = Some(mark_line);
        self
    }
}

impl ToString for Chart {
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
