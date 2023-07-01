// main.rs
mod component;
mod dataset;
mod renderer;
mod series;
mod style;
mod utility;

use echarts::component::axis;
use echarts::component::grid;
use echarts::component::legend;
use echarts::component::title;
use echarts::component::toolbox;
use echarts::component::tooltip;
use echarts::series::line;
use echarts::series::Series;
use echarts::utility::area_style;
use echarts::utility::color;
use echarts::utility::line_style;
use echarts::Chart;

fn main() {
    println!("Hello, world!");
}
