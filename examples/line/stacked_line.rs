use echarts::{
    component::{axis, grid, legend, title, toolbox, tooltip},
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .title(title::Title::new().text("Stacked Line"))
        .tooltip(tooltip::Tooltip::new().trigger(tooltip::Trigger::Axis))
        .legend(legend::Legend::new().data(vec![
            "Email",
            "Union Ads",
            "Video Ads",
            "Direct",
            "Search Engine",
        ]))
        .grid(
            grid::Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .toolbox(
            toolbox::Toolbox::new()
                .feature(toolbox::Feature::new().save_as_image(toolbox::SaveAsImage::new())),
        )
        .x_axis(
            axis::Axis::new()
                .type_(axis::AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis::AxisType::Value))
        .series(Series::Line(
            line::Line::new()
                .name("Email")
                .stack("Total")
                .data(vec![120.0, 132.0, 101.0, 134.0, 90.0, 230.0, 210.0]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Union Ads")
                .stack("Total")
                .data(vec![220.0, 182.0, 191.0, 234.0, 290.0, 330.0, 310.0]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Video Ads")
                .stack("Total")
                .data(vec![150.0, 232.0, 201.0, 154.0, 190.0, 330.0, 410.0]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Direct")
                .stack("Total")
                .data(vec![320.0, 332.0, 301.0, 334.0, 390.0, 330.0, 320.0]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Search Engine")
                .stack("Total")
                .data(vec![820.0, 932.0, 901.0, 934.0, 1290.0, 1330.0, 1320.0]),
        ));
    println!("{}", chart.to_string());
}
