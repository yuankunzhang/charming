#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
Charming is a powerful and versatile chart rendering library for Rust that
leverages the power of [Apache Echarts](https://echarts.apache.org/en/index.html)
to deliver high-quality data visualization. Built with the Rust programming
language, this library aims to provide the Rust ecosystem with an intuitive
and effective way to generate and visualize charts, using a declarative and
user-friendly API.

## Basic Usage

Refer to the documentation of the [`Chart`] struct for how to create a chart
with various components.

Once you create a chart, you can render it into various format. Charming
provides three types of renderers:

- **HTML renderer**: [`HtmlRenderer`] renders a chart into an HTML fragments and
  offloads the actual rendering to user's web browser for an interactive,
  seamless experience. This renderer is useful when you want to render a chart
  on the client side, e.g., in a web application.
- **Image renderer**: [`ImageRenderer`] renders a chart into an image file. This
  renderer makes use of an embed [deno_core](https://github.com/denoland/deno_core)
  engine to execute the JavaScript code of Echarts and generate an image file.
  This renderer is disabled by default, and you need to enable the `ssr`
  (Server-Side Rendering) feature to use it.
- **WASM renderer**: [`WasmRenderer`] renders a chart in a WebAssembly runtime.
  This renderer is disabled by default, and you need to enable the `wasm`
  feature to use it. Note that the `wasm` feature and `ssr` feature are
  mutually exclusive.

Here is an example of drawing a simple pie chart into an SVG file:

```rust
use charming::{
    component::Legend,
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart, ImageRenderer
};

fn main() {
    let chart = Chart::new()
        .legend(Legend::new().top("bottom"))
        .series(
            Pie::new()
                .name("Nightingale Chart")
                .rose_type(PieRoseType::Radius)
                .radius(vec!["50", "250"])
                .center(vec!["50%", "50%"])
                .item_style(ItemStyle::new().border_radius(8))
                .data(vec![
                    (40.0, "rose 1"),
                    (38.0, "rose 2"),
                    (32.0, "rose 3"),
                    (30.0, "rose 4"),
                    (28.0, "rose 5"),
                    (26.0, "rose 6"),
                    (22.0, "rose 7"),
                    (18.0, "rose 8"),
                ]),
        );

    let mut renderer = ImageRenderer::new(1000, 800);
    renderer.save(&chart, "/tmp/nightingale.svg");
}
```

## Themes

Charming supports a number of themes out of the box. You can use the
[`theme::Theme`] enum to specify a theme for your chart. For instance, the
following code snippet shows how to use the `Westeros` theme:

```rust
use charming::{Chart, ImageRenderer};
use charming::theme::Theme;
use charming::component::Title;

ImageRenderer::new(1000, 800).theme(Theme::Westeros).save(
    &Chart::new().title(Title::new().text("Westeros")),
    "/tmp/westeros.svg",
);
```

Future versions of Charming will support custom themes.
 */
pub mod component;
pub mod datatype;
pub mod element;
pub mod renderer;
pub mod series;
pub mod theme;

pub use renderer::*;

use component::{
    AngleAxis, Aria, Axis, Axis3D, DataZoom, GeoMap, Grid, Grid3D, Legend, ParallelAxis,
    ParallelCoordinate, PolarCoordinate, RadarCoordinate, RadiusAxis, SaveAsImageType, SingleAxis,
    Title, Toolbox, VisualMap,
};
use datatype::Dataset;
use element::{process_raw_strings, AxisPointer, Color, MarkLine, Tooltip};
use serde::Serialize;
use series::Series;

/**
The chart representation.

## Anatomy of a Chart

A chart is a collection of different components, each of which is responsible
for rendering a specific part of the chart. Below is a sample chart with a
few components:

```txt
                   Sales Report
  |                                                        # coffee
30|                  x                                     x juice
  |      @           x             @                       @ milk
20|    # @           x@           x@          #
  |    #x@          #x@          #x@          #x
10|    #x@          #x@          #x@          #x@
  |    #x@          #x@          #x@          #x@
 0+-----------------------------------------------------
       Jan          Feb          Mar          Apr
```

The chart above has the following components: **an x axis**, **an y axis**,
**a title** on the top center, and **a legend** on the top right.

The creation of charts in Charming is done in a builder-like fashion. Once you
get a hang of this pattern, you will find that it is very easy to compose a
chart. For instance, the following code snippet shows how to create the chart
above:

```rust
use charming::Chart;
use charming::component::{Axis, Legend, Title};

let chart = Chart::new()
    .title(Title::new().text("Sales Report"))
    .x_axis(Axis::new().data(vec!["Jan", "Feb", "Mar", "Apr"]))
    .y_axis(Axis::new())
    .legend(Legend::new().data(vec!["coffee", "juice", "milk"]));
```

## Components of a Chart

The following sections describe the components of a chart in detail.

### Title

[`Title`] is the title of a chart, including main title and subtitle. A chart
can have multiple titles, which is useful when you want to show multiple sub-
charts in a single chart.

```rust
use charming::Chart;
use charming::component::Title;

let chart = Chart::new()
    .title(Title::new().text("Sales Report"));
```

### Legend

[`Legend`] is the legend of a chart, which is used to show the meaning of the
symbols and colors in the chart. A chart can have multiple legends.

```rust
use charming::Chart;
use charming::component::Legend;

let chart = Chart::new()
    .legend(Legend::new().data(vec!["coffee", "juice", "milk"]));
```

### Grid

[`Grid`] is the background grid in a cartesian coordinate system. A chart can
have multiple grids.

```rust
use charming::Chart;
use charming::component::Grid;

let chart = Chart::new()
    .grid(Grid::new());
```

### X Axis and Y Axis

[`Axis`] is the axis in a cartesian coordinate system.

```rust
use charming::Chart;
use charming::component::Axis;

let chart = Chart::new()
    .x_axis(Axis::new().data(vec!["Jan", "Feb", "Mar", "Apr"]))
    .y_axis(Axis::new());
```

### Polar Coordinate

[`Polar`] is the polar coordinate system. Polar coordinate can be used in
scatter and line charts. Every polar coordinate has an [`AngleAxis`] and a
[`RadiusAxis`].

### Radar Coordinate

[`RadarCoordinate`] is the radar coordinate system. Radar coordinate can be in
radar charts.

### Data Zoom

[`DataZoom`] is used for zooming a specific area, which enables user to view
data in different scales. A chart can have multiple data zooms.

### Visual Map

[`VisualMap`] is a visual encoding component. It maps data to visual channels,
such as color, symbol size or symbol shape. A chart can have multiple visual
maps.

### Tooltip

[`Tooltip`] is a floating box that appears when user hovers over a data item.

### AxisPointer

[`AxisPointer`] is a tool for displaying reference line and axis value under
mouse pointer.

### Toolbox

[`Toolbox`] is a feature toolbox that includes data view, save as image, data
zoom, restore, and reset.
 */
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
    #[serde(rename = "grid3D")]
    grid3d: Vec<Grid3D>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    x_axis: Vec<Axis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "xAxis3D")]
    x_axis3d: Vec<Axis3D>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    y_axis: Vec<Axis>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "yAxis3D")]
    y_axis3d: Vec<Axis3D>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "zAxis3D")]
    z_axis3d: Vec<Axis3D>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    polar: Vec<PolarCoordinate>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    aria: Option<Aria>,

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
            aria: None,
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

    pub fn polar(mut self, polar: PolarCoordinate) -> Self {
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

    pub fn radar(mut self, radar: RadarCoordinate) -> Self {
        self.radar.push(radar);
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

    pub fn aria(mut self, aria: Aria) -> Self {
        self.aria = Some(aria);
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

    pub fn save_as_image_type(&self) -> Option<&SaveAsImageType> {
        self.toolbox
            .as_ref()
            .and_then(|toolbox| toolbox.save_as_image_type())
    }
}

impl ToString for Chart {
    fn to_string(&self) -> String {
        process_raw_strings(serde_json::to_string_pretty(self).unwrap().as_str())
    }
}

#[derive(Debug)]
pub enum EchartsError {
    HtmlRenderingError(String),
    ImageRenderingError(String),
    JsRuntimeError(String),
    WasmError(String),
}

impl std::error::Error for EchartsError {}
impl std::fmt::Display for EchartsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
