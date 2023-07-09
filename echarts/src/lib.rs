pub mod component;
pub mod datatype;
pub mod element;
pub mod renderer;
pub mod series;

pub use renderer::*;

use component::{
    AngleAxis, Axis, Axis3D, AxisPointer, DataZoom, GeoMap, Grid, Grid3D, Legend, ParallelAxis,
    ParallelCoordinate, Polar, RadarCoordinate, RadiusAxis, SingleAxis, Title, Toolbox, Tooltip,
    VisualMap,
};
use datatype::Dataset;
use element::{Color, MarkLine};
use serde::Serialize;
use series::Series;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    title: Vec<Title>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tooltip: Option<Tooltip>,

    #[serde(skip_serializing_if = "Option::is_none")]
    legend: Option<Legend>,

    #[serde(skip_serializing_if = "Option::is_none")]
    toolbox: Option<Toolbox>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    grid: Vec<Grid>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    grid3d: Vec<Grid3D>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    x_axis: Vec<Axis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    x_axis3d: Vec<Axis3D>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    y_axis: Vec<Axis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    y_axis3d: Vec<Axis3D>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    z_axis3d: Vec<Axis3D>,

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
    axis_pointer: Vec<AxisPointer>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mark_line: Option<MarkLine>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    series: Vec<Series>,

    #[serde(skip_serializing)]
    geo_maps: Vec<GeoMap>,
}

impl Chart {
    pub fn new() -> Self {
        Self {
            title: vec![],
            toolbox: None,
            legend: None,
            tooltip: None,
            grid: vec![],
            grid3d: vec![],
            x_axis: vec![],
            x_axis3d: vec![],
            y_axis: vec![],
            y_axis3d: vec![],
            z_axis3d: vec![],
            polar: vec![],
            angle_axis: vec![],
            radius_axis: vec![],
            single_axis: None,
            parallel_axis: vec![],
            axis_pointer: vec![],
            visual_map: vec![],
            data_zoom: vec![],
            parallel: None,
            dataset: None,
            radar: vec![],
            color: vec![],
            background_color: None,
            mark_line: None,
            series: vec![],
            geo_maps: vec![],
        }
    }

    pub fn title(mut self, title: Title) -> Self {
        self.title.push(title);
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

    pub fn grid3d(mut self, grid: Grid3D) -> Self {
        self.grid3d.push(grid);
        self
    }

    pub fn x_axis(mut self, x_axis: Axis) -> Self {
        self.x_axis.push(x_axis);
        self
    }

    pub fn x_axis3d(mut self, x_axis: Axis3D) -> Self {
        self.x_axis3d.push(x_axis);
        self
    }

    pub fn y_axis(mut self, y_axis: Axis) -> Self {
        self.y_axis.push(y_axis);
        self
    }

    pub fn y_axis3d(mut self, y_axis: Axis3D) -> Self {
        self.y_axis3d.push(y_axis);
        self
    }

    pub fn z_axis3d(mut self, z_axis: Axis3D) -> Self {
        self.z_axis3d.push(z_axis);
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

    pub fn axis_pointer(mut self, axis_pointer: AxisPointer) -> Self {
        self.axis_pointer.push(axis_pointer);
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

    pub fn background_color<C: Into<Color>>(mut self, color: C) -> Self {
        self.background_color = Some(color.into());
        self
    }

    pub fn mark_line(mut self, mark_line: MarkLine) -> Self {
        self.mark_line = Some(mark_line);
        self
    }

    pub fn series<S: Into<Series>>(mut self, series: S) -> Self {
        self.series.push(series.into());
        self
    }

    pub fn geo_map<M: Into<GeoMap>>(mut self, map: M) -> Self {
        self.geo_maps.push(map.into());
        self
    }
}

fn process_raw_strings(s: String) -> String {
    let s = str::replace(&s, &format!("\"{}", element::RAW_MARK), "");
    str::replace(&s, &format!("{}\"", element::RAW_MARK), "")
}

impl ToString for Chart {
    fn to_string(&self) -> String {
        process_raw_strings(serde_json::to_string_pretty(self).unwrap())
    }
}
