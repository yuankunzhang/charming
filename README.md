# Charming - A Rust Visualization Library

[![crates.io](https://img.shields.io/crates/v/charming.svg)](https://crates.io/crates/charming)
[![docs.rs](https://docs.rs/charming/badge.svg)](https://docs.rs/charming)

Charming is a powerful and versatile chart rendering library for Rust that leverages the power of [Apache ECharts](https://echarts.apache.org/en/index.html) to deliver high-quality data visualizations. Built with the Rust programming language, this library aims to provide the Rust ecosystem with an intuitive and effective way to generate and visualize charts, using a declarative and user-friendly API.

Highlights:

- Easy-to-use, declaritive API.
- Abundant chart types with rich and customizable chart themes and styles.
- Ready to use in WebAssembly environments.
- Rendering to multiple formats, including HTML, SVG, PNG, JPEG, GIF, WEBP, PNM, TIFF, TGA, DDS, BMP, ICO, HDR, OPENEXR, FARBFELD, AVIF, and QOI.

## Themes

<table>
    <tr>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/default.svg" alt="Default" /><p align="center">Default</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/dark.svg" alt="Dark" /><p align="center">Dark</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/vintage.svg" alt="Vintage" /><p align="center">Vintage</p></td>
    </tr>
    <tr>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/westeros.svg" alt="Westeros" /><p align="center">Westeros</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/essos.svg" alt="Essos" /><p align="center">Essos</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/wonderland.svg" alt="Wonderland" /><p align="center">Wonderland</p></td>
    </tr>
    <tr>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/walden.svg" alt="Walden" /><p align="center">Walden</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/chalk.svg" alt="Chalk" /><p align="center">Chalk</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/infographic.svg" alt="Infographic" /><p align="center">Infographic</p></td>
    </tr>
    <tr>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/macarons.svg" alt="Macarons" /><p align="center">Macarons</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/roma.svg" alt="Roma" /><p align="center">Roma</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/shine.svg" alt="Shine" /><p align="center">Shine</p></td>
    </tr>
    <tr>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/purple-passion.svg" alt="Purple Passion" /><p align="center">Purple Passion</p></td>
        <td><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme/halloween.svg" alt="Halloween" /><p align="center">Halloween</p></td>
    </tr>
</table>

Future versions of Charming will support custom themes.

## Basic Usage

Add charming as a dependency:

```sh
$ cargo add charming
```

Refer to the documentation of the [`Chart`](https://docs.rs/charming/latest/charming/struct.Chart.html) struct for how to create a chart with various components.

Once you create a chart, you can render it into various format. Charming provides three types of renderers:

- **HTML renderer**: `HtmlRenderer` renders a chart into an HTML fragments and offloads the actual rendering to user's web browser for an interactive, seamless experience. This renderer is useful when you want to render a chart on the client side, e.g., in a web application.
- **Image renderer**: `ImageRenderer` renders a chart into an image file. This renderer makes use of an embed [deno_core](https://github.com/denoland/deno_core) engine to execute the JavaScript code of Echarts and generate an image file. This renderer is disabled by default, and you need to enable the `ssr` (Server-Side Rendering) feature to use it.
- **WASM renderer**: `WasmRenderer` renders a chart in a WebAssembly runtime. This renderer is disabled by default, and you need to enable the `wasm` feature to use it. **Note that the `wasm` feature and `ssr` feature are mutually exclusive**.

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
                .radius(vec!["50", "150"])
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

This code creates the following SVG file:

![](img/pie/nightingale.svg)

As another example, the code file [gallery/src/dataset/encode_and_matrix.rs](./gallery/src/dataset/encode_and_matrix.rs) draws a complex chart with four sub-charts:

![](img/dataset/encode_and_matrix.svg)

### MSRV

We do not provide any minimal supported Rust version and it is usually the latest stable release as the deno dependency upgrades their version very frequently.

### Crate Feature Flags

The following feature flags are available, **note that `ssr` and `wasm` can't be used together**:

- `ssr` - Enables the `ImageRenderer`, which provides the capability to generate image files.
- `ssr-raster` Enables raster support to the `ImageRenderer` (png, jpg, etc.)
- `wasm` - Enables the `WasmRenderer`, which provides the capability to render charts in WebAssembly runtime.

### Renderers

```rs
// Use HtmlRenderer.
use charming::HtmlRenderer;

// Chart dimension 1000x800.
let renderer = HtmlRenderer::new("my charts", 1000, 800);
// Render the chart as HTML string.
let html_str = renderer.render(&chart).unwrap();
// Save the chart as HTML file.
renderer.save(&chart, "/tmp/chart.html").unwrap();


// Use ImageRenderer. The `ssr` feature needs to be enabled.
use charming::{ImageRenderer, ImageFormat};

// Chart dimension 1000x800.
let mut renderer = ImageRenderer::new(1000, 800);
// Render the chart as SVG string.
renderer.render(&chart).unwrap();
// Render the chart as PNG bytes.
renderer.render_format(ImageFormat::Png, &chart).unwrap();
// Save the chart as SVG file.
renderer.save(&chart, "/tmp/chart.svg").unwrap();
// Save the chart as PNG file.
renderer.save_format(ImageFormat::Png, &chart, "/tmp/chart.png");


// Use WasmRenderer. The `wasm` feature needs to be enabled.
use charming::WasmRenderer;

// Chart dimension 1000x800.
let renderer = WasmRenderer::new(1000, 800);
// Render the chart in the WebAssembly runtime
renderer.render("my-chart-id", &chart).unwrap();
```

### Themes

Charming supports a number of themes out of the box. You can use the `Theme` enum to specify a theme for your chart. For instance, the following code snippet shows how to use the `Westeros` theme:

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

## Gallery

Here are some selected chart examples. Click on any single chart to view its source code file.

You can also clone the repo and run `cargo run --bin gallery` to view the interactive charts on the rendered HTML page.

### Bar Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/bar/bar_with_background.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/bar/bar_with_background.svg" width="40%" alt="Bar with Background" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/bar/basic_bar.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/bar/basic_bar.svg" width="40%" alt="Basic Bar" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/bar/radial_polar_bar_label_position.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/bar/radial_polar_bar_label_position.svg" width="40%" alt="Radial Polar Bar Label Position" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/bar/set_style_of_single_bar.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/bar/set_style_of_single_bar.svg" width="40%" alt="Set Style of Single Bar" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/bar/stacked_column.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/bar/stacked_column.svg" width="40%" alt="Stacked Column" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/bar/tangential_polar_bar.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/bar/tangential_polar_bar.svg" width="40%" alt="Tangential Polar Bar" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/bar/waterfall.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/bar/waterfall.svg" width="40%" alt="Waterfall" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/bar/world_population.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/bar/world_population.svg" width="40%" alt="World Population" /></a>
</div>

### Boxplot Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/boxplot/boxplot_light_velocity.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/boxplot/boxplot_light_velocity.svg" width="40%" alt="Boxplot Light Velocity" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/boxplot/multiple_categories.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/boxplot/multiple_categories.svg" width="40%" alt="Multiple Categories" /></a>
</div>

### Candlestick Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/candlestick/basic_candlestick.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/candlestick/basic_candlestick.svg" width="40%" alt="Basic Candlestick" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/candlestick/shanghai_index.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/candlestick/shanghai_index.svg" width="40%" alt="Shanghai Index" /></a>
</div>

### Funnel Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/funnel/funnel_chart.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/funnel/funnel_chart.svg" width="40%" alt="Funnel Chart" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/funnel/multiple_funnels.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/funnel/multiple_funnels.svg" width="40%" alt="Multiple Funnels" /></a>
</div>

### Gauge Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/gauge/gauge_barometer.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/gauge/gauge_barometer.svg" width="40%" alt="Gauge Barometer" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/gauge/gauge_basic.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/gauge/gauge_basic.svg" width="40%" alt="Gauge Basic" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/gauge/gauge_simple.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/gauge/gauge_simple.svg" width="40%" alt="Gauge Simple" /></a>
</div>

### Graph Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/graph/hide_overlapped_label.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/graph/hide_overlapped_label.svg" width="40%" alt="Hide Overlapped Label" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/graph/les_miserables.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/graph/les_miserables.svg" width="40%" alt="Les Miserables" /></a>
</div>

### Heatmap Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/heatmap/heatmap_on_cartesian.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/heatmap/heatmap_on_cartesian.svg" width="40%" alt="Heatmap on Cartesian" /></a>
</div>

### Line Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/area_pieces.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/area_pieces.svg" width="40%" alt="Area Pieces" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/basic_area.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/basic_area.svg" width="40%" alt="Basic Area" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/basic_line.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/basic_line.svg" width="40%" alt="Basic Line" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/confidence_band.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/confidence_band.svg" width="40%" alt="Confidence Band" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/data_transform_filter.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/data_transform_filter.svg" width="40%" alt="Data Transform Filter" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/distribution_of_electricity.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/distribution_of_electricity.svg" width="40%" alt="Distribution of Electricity" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/gradient_stacked_area.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/gradient_stacked_area.svg" width="40%" alt="Gradient Stacked Area" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/large_scale_area.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/large_scale_area.svg" width="40%" alt="Large Scale Area" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/line_gradient.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/line_gradient.svg" width="40%" alt="Line Gradient" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/rainfall.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/rainfall.svg" width="40%" alt="Rainfall" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/rainfall_vs_evaporation.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/rainfall_vs_evaporation.svg" width="40%" alt="Rainfall Vs. Evaporation" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/smoothed_line.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/smoothed_line.svg" width="40%" alt="Smoothed Line" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/stacked_area.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/stacked_area.svg" width="40%" alt="Stacked Area" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/stacked_line.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/stacked_line.svg" width="40%" alt="Stacked Line" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/step_line.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/step_line.svg" width="40%" alt="Step Line" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/temperature_change.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/temperature_change.svg" width="40%" alt="Temperature Change" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/line/two_value_axes_in_polar.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/line/two_value_axes_in_polar.svg" width="40%" alt="Two Value-Axes in Polar" /></a>
</div>

### Parallel Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/parallel/basic_parallel.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/parallel/basic_parallel.svg" width="40%" alt="Basic Parallel" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/parallel/parallel_aqi.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/parallel/parallel_aqi.svg" width="40%" alt="Parallel AQI" /></a>
</div>

### Pie Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/pie/doughnut_chart_with_rounded_corner.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/pie/doughnut_chart_with_rounded_corner.svg" width="40%" alt="Nightingale" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/pie/nightingale.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/pie/nightingale.svg" width="40%" alt="Nightingale" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/pie/referer_of_a_website.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/pie/referer_of_a_website.svg" width="40%" alt="Referer of a Website" /></a>
</div>

### Radar Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/radar/basic_radar.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/radar/basic_radar.svg" width="40%" alt="Basic Radar" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/radar/multiple_radar.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/radar/multiple_radar.svg" width="40%" alt="Multiple Radar" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/radar/proportion_of_browsers.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/radar/proportion_of_browsers.svg" width="40%" alt="Proportion of Browsers" /></a>
</div>

### Sankey Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/sankey/basic_sankey.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/sankey/basic_sankey.svg" width="40%" alt="Basic Sankey" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/sankey/node_align_left_sankey.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/sankey/node_align_left_sankey.svg" width="40%" alt="Node Align Left Sankey" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/sankey/sankey_orient_vertical.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/sankey/sankey_orient_vertical.svg" width="40%" alt="Sankey Orient Vertical" /></a>
</div>

### Scatter Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/scatter/anscombe_quartet.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/scatter/anscombe_quartet.svg" width="40%" alt="Anscombe Quartet" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/scatter/basic_scatter.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/scatter/basic_scatter.svg" width="40%" alt="Basic Scatter" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/scatter/bubble_chart.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/scatter/bubble_chart.svg" width="40%" alt="Bubble Chart" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/scatter/effect_scatter.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/scatter/effect_scatter.svg" width="40%" alt="Effect Scatter" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/scatter/punch_card_of_github.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/scatter/punch_card_of_github.svg" width="40%" alt="Punch Card of Github" /></a>
</div>

### Calendar Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/calendar/simple_calendar.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/calendar/simple_calendar.svg" width="40%" alt="Simple Calendar" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/calendar/heatmap_calendar.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/calendar/heatmap_calendar.svg" width="40%" alt="Heatmap Calendar" /></a>
</div>

### Sunburst Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/sunburst/drink_flavors.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/sunburst/drink_flavors.svg" width="40%" alt="Drink Flavors" /></a>
</div>

### Theme River Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/theme_river/theme_river_lastfm.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/theme_river/theme_river_lastfm.svg" width="40%" alt="Theme River LastFM" /></a>
</div>

### Tree Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/tree/from_left_to_right_tree.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/tree/from_left_to_right_tree.svg" width="40%" alt="From Left to Right Tree" /></a>
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/tree/multiple_trees.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/tree/multiple_trees.svg" width="40%" alt="Multiple Trees" /></a>
</div>

### Custom Charts

<div align="center">
<a href="https://github.com/yuankunzhang/charming/tree/main/gallery/src/candlestick/ohlc.rs"><img src="https://github.com/yuankunzhang/charming/blob/main/img/candlestick/ohlc.svg" width="40%" alt="Custom Error Bar/Candle Stick" /></a>
</div>
