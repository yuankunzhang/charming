# Charming - A Rust Visualization Library

Charming is a powerful and versatile chart rendering library for Rust that leverages the power of [Apache ECharts](https://echarts.apache.org/en/index.html) to deliver high-quality data visualizations. Built with the Rust programming language, this library aims to provide the Rust ecosystem with an intuitive and effective way to generate and visualize charts, using a declarative and user-friendly API.

Highlights:

- Easy-to-use, declaritive API.
- Rich and customizable chart themes and styles.
- Rendering to multiple formats, including HTML, SVG, PNG, and JPEG.
- Abundant chart types.

## Themes

<table>
    <tr>
        <td><img src="./img/theme/default.png" alt="Default" /><p align="center">Default</p></td>
        <td><img src="./img/theme/dark.png" alt="Dark" /><p align="center">Dark</p></td>
        <td><img src="./img/theme/vintage.png" alt="Vintage" /><p align="center">Vintage</p></td>
    </tr>
    <tr>
        <td><img src="./img/theme/westeros.png" alt="Westeros" /><p align="center">Vintage</p></td>
        <td><img src="./img/theme/essos.png" alt="Essos" /><p align="center">Essos</p></td>
        <td><img src="./img/theme/wonderland.png" alt="Wonderland" /><p align="center">Essos</p></td>
    </tr>
    <tr>
        <td><img src="./img/theme/walden.png" alt="Walden" /><p align="center">Walden</p></td>
        <td><img src="./img/theme/chalk.png" alt="Chalk" /><p align="center">Chalk</p></td>
        <td><img src="./img/theme/infographic.png" alt="Infographic" /><p align="center">Infographic</p></td>
    </tr>
    <tr>
        <td><img src="./img/theme/macarons.png" alt="Macarons" /><p align="center">Macarons</p></td>
        <td><img src="./img/theme/roma.png" alt="Roma" /><p align="center">Roma</p></td>
        <td><img src="./img/theme/shine.png" alt="Shine" /><p align="center">Shine</p></td>
    </tr>
    <tr>
        <td><img src="./img/theme/purple-passion.png" alt="Purple Passion" /><p align="center">Purple Passion</p></td>
        <td><img src="./img/theme/halloween.png" alt="Halloween" /><p align="center">Halloween</p></td>
    </tr>
</table>

## Basic Usage

Add charming to your `Cargo.toml`:

```toml
[dependencies]
charming = "0.1.0"
```

Below is an example of drawing a simple pie chart.

```rs
use echarts::{
    component::Legend,
    df,
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
                .radius(("50", "250"))
                .center(("50%", "50%"))
                .item_style(ItemStyle::new().border_radius(8))
                .data(df![
                    (40.0, "rose 1"),
                    (38.0, "rose 2"),
                    (32.0, "rose 3"),
                    (30.0, "rose 4"),
                    (28.0, "rose 5"),
                    (26.0, "rose 6"),
                    (22.0, "rose 7"),
                    (18.0, "rose 8"),
                ]),
        )
    let mut renderer = ImageRenderer::new(1000, 800);
    renderer.save(&chart, "/tmp/nightingale.svg");
}
```

This code creates the following SVG file:

![](img/nightingale.svg)

As another example, the code file [gallery/src/dataset/encode_and_matrix.rs](./gallery/src/dataset/encode_and_matrix.rs) draws a complex chart with four sub-charts:

![](img/encode-and-matrix.svg)

## Gallery

Here are some selected chart examples. Click on any single chart to view its source code file.

You can also clone the repo and run `cargo run --bin gallery` to view the interactive charts on the rendered HTML page.

### Bar Charts

<div align="center">
<a href="./gallery/src/bar/bar_with_background.rs"><img src="./img/bar/bar_with_background.svg" width="40%" alt="Bar with Background" /></a>
<a href="./gallery/src/bar/basic_bar.rs"><img src="./img/bar/basic_bar.svg" width="40%" alt="Basic Bar" /></a>
<a href="./gallery/src/bar/radial_polar_bar_label_position.rs"><img src="./img/bar/radial_polar_bar_label_position.svg" width="40%" alt="Radial Polar Bar Label Position" /></a>
<a href="./gallery/src/bar/set_style_of_single_bar.rs"><img src="./img/bar/set_style_of_single_bar.svg" width="40%" alt="Set Style of Single Bar" /></a>
<a href="./gallery/src/bar/stacked_column.rs"><img src="./img/bar/stacked_column.svg" width="40%" alt="Stacked Column" /></a>
<a href="./gallery/src/bar/tangential_polar_bar.rs"><img src="./img/bar/tangential_polar_bar.svg" width="40%" alt="Tangential Polar Bar" /></a>
<a href="./gallery/src/bar/waterfall.rs"><img src="./img/bar/waterfall.svg" width="40%" alt="Waterfall" /></a>
<a href="./gallery/src/bar/world_population.rs"><img src="./img/bar/world_population.svg" width="40%" alt="World Population" /></a>
</div>

### Boxplot Charts

<div align="center">
<a href="./gallery/src/boxplot/boxplot_light_velocity.rs"><img src="./img/boxplot/boxplot_light_velocity.svg" width="40%" alt="Boxplot Light Velocity" /></a>
<a href="./gallery/src/boxplot/multiple_categories.rs"><img src="./img/boxplot/multiple_categories.svg" width="40%" alt="Multiple Categories" /></a>
</div>

### Candlestick Charts

<div align="center">
<a href="./gallery/src/candlestick/basic_candlestick.rs"><img src="./img/candlestick/basic_candlestick.svg" width="40%" alt="Basic Candlestick" /></a>
<a href="./gallery/src/candlestick/shanghai_index.rs"><img src="./img/candlestick/shanghai_index.svg" width="40%" alt="Shanghai Index" /></a>
</div>

### Funnel Charts

<div align="center">
<a href="./gallery/src/funnel/multiple_funnels.rs"><img src="./img/funnel/multiple_funnels.svg" width="40%" alt="Multiple Funnels" /></a>
</div>

### Gauge Charts

<div align="center">
<a href="./gallery/src/gauge/gauge_barometer.rs"><img src="./img/gauge/gauge_barometer.svg" width="40%" alt="Gauge Barometer" /></a>
<a href="./gallery/src/gauge/gauge_basic.rs"><img src="./img/gauge/gauge_basic.svg" width="40%" alt="Gauge Basic" /></a>
<a href="./gallery/src/gauge/gauge_simple.rs"><img src="./img/gauge/gauge_simple.svg" width="40%" alt="Gauge Simple" /></a>
</div>

### Graph Charts

<div align="center">
<a href="./gallery/src/graph/hide_overlapped_label.rs"><img src="./img/graph/hide_overlapped_label.svg" width="40%" alt="Hide Overlapped Label" /></a>
<a href="./gallery/src/graph/les_miserables.rs"><img src="./img/graph/les_miserables.svg" width="40%" alt="Les Miserables" /></a>
</div>

### Line Charts

<div align="center">
<a href="./gallery/src/line/area_pieces.rs"><img src="./img/line/area_pieces.svg" width="40%" alt="Area Pieces" /></a>
<a href="./gallery/src/line/basic_area.rs"><img src="./img/line/basic_area.svg" width="40%" alt="Basic Area" /></a>
<a href="./gallery/src/line/basic_line.rs"><img src="./img/line/basic_line.svg" width="40%" alt="Basic Line" /></a>
<a href="./gallery/src/line/confidence_band.rs"><img src="./img/line/confidence_band.svg" width="40%" alt="Confidence Band" /></a>
<a href="./gallery/src/line/data_transform_filter.rs"><img src="./img/line/data_transform_filter.svg" width="40%" alt="Data Transform Filter" /></a>
<a href="./gallery/src/line/distribution_of_electricity.rs"><img src="./img/line/distribution_of_electricity.svg" width="40%" alt="Distribution of Electricity" /></a>
<a href="./gallery/src/line/gradient_stacked_area.rs"><img src="./img/line/gradient_stacked_area.svg" width="40%" alt="Gradient Stacked Area" /></a>
<a href="./gallery/src/line/large_scale_area.rs"><img src="./img/line/large_scale_area.svg" width="40%" alt="Large Scale Area" /></a>
<a href="./gallery/src/line/line_gradient.rs"><img src="./img/line/line_gradient.svg" width="40%" alt="Line Gradient" /></a>
<a href="./gallery/src/line/rainfall.rs"><img src="./img/line/rainfall.svg" width="40%" alt="Rainfall" /></a>
<a href="./gallery/src/line/rainfall_vs_evaporation.rs"><img src="./img/line/rainfall_vs_evaporation.svg" width="40%" alt="Rainfall Vs. Evaporation" /></a>
<a href="./gallery/src/line/smoothed_line.rs"><img src="./img/line/smoothed_line.svg" width="40%" alt="Smoothed Line" /></a>
<a href="./gallery/src/line/stacked_area.rs"><img src="./img/line/stacked_area.svg" width="40%" alt="Stacked Area" /></a>
<a href="./gallery/src/line/stacked_line.rs"><img src="./img/line/stacked_line.svg" width="40%" alt="Stacked Line" /></a>
<a href="./gallery/src/line/temperature_change.rs"><img src="./img/line/temperature_change.svg" width="40%" alt="Temperature Change" /></a>
</div>

### Parallel Charts

<div align="center">
<a href="./gallery/src/parallel/basic_parallel.rs"><img src="./img/parallel/basic_parallel.svg" width="40%" alt="Basic Parallel" /></a>
<a href="./gallery/src/parallel/parallel_aqi.rs"><img src="./img/parallel/parallel_aqi.svg" width="40%" alt="Parallel AQI" /></a>
</div>

### Pie Charts

<div align="center">
<a href="./gallery/src/pie/doughnut_chart_with_rounded_corner.rs"><img src="./img/pie/doughnut_chart_with_rounded_corner.svg" width="40%" alt="Nightingale" /></a>
<a href="./gallery/src/pie/nightingale.rs"><img src="./img/pie/nightingale.svg" width="40%" alt="Nightingale" /></a>
<a href="./gallery/src/pie/referer_of_a_website.rs"><img src="./img/pie/referer_of_a_website.svg" width="40%" alt="Referer of a Website" /></a>
</div>

### Radar Charts

<div align="center">
<a href="./gallery/src/radar/basic_radar.rs"><img src="./img/radar/basic_radar.svg" width="40%" alt="Basic Radar" /></a>
<a href="./gallery/src/radar/multiple_radar.rs"><img src="./img/radar/multiple_radar.svg" width="40%" alt="Multiple Radar" /></a>
<a href="./gallery/src/radar/proportion_of_browsers.rs"><img src="./img/radar/proportion_of_browsers.svg" width="40%" alt="Proportion of Browsers" /></a>
</div>

### Sankey Charts

<div align="center">
<a href="./gallery/src/sankey/basic_sankey.rs"><img src="./img/sankey/basic_sankey.svg" width="40%" alt="Basic Sankey" /></a>
<a href="./gallery/src/sankey/node_align_left_sankey.rs"><img src="./img/sankey/node_align_left_sankey.svg" width="40%" alt="Node Align Left Sankey" /></a>
<a href="./gallery/src/sankey/sankey_orient_vertical.rs"><img src="./img/sankey/sankey_orient_vertical.svg" width="40%" alt="Sankey Orient Vertical" /></a>
</div>

### Scatter Charts

<div align="center">
<a href="./gallery/src/scatter/anscombe_quartet.rs"><img src="./img/scatter/anscombe_quartet.svg" width="40%" alt="Anscombe Quartet" /></a>
<a href="./gallery/src/scatter/basic_scatter.rs"><img src="./img/scatter/basic_scatter.svg" width="40%" alt="Basic Scatter" /></a>
<a href="./gallery/src/scatter/effect_scatter.rs"><img src="./img/scatter/effect_scatter.svg" width="40%" alt="Effect Scatter" /></a>
<a href="./gallery/src/scatter/punch_cart_of_github.rs"><img src="./img/scatter/punch_card_of_github.svg" width="40%" alt="Punch Card of Github" /></a>
</div>

### Sunburst Charts

<div align="center">
<a href="./gallery/src/sunburst/drink_flavors.rs"><img src="./img/sunburst/drink_flavors.svg" width="40%" alt="Drink Flavors" /></a>
</div>

### Theme River Charts

<div align="center">
<a href="./gallery/src/theme_river/theme_river_lastfm.rs"><img src="./img/theme_river/theme_river_lastfm.svg" width="40%" alt="Theme River LastFM" /></a>
</div>

### Tree Charts

<div align="center">
<a href="./gallery/src/tree/from_left_to_right_tree.rs"><img src="./img/tree/from_left_to_right_tree.svg" width="40%" alt="From Left to Right Tree" /></a>
<a href="./gallery/src/tree/multiple_trees.rs"><img src="./img/tree/multiple_trees.svg" width="40%" alt="Multiple Trees" /></a>
</div>
