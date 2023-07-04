use echarts::{
    component::{axis, grid, legend, title, toolbox, tooltip},
    element::axis_type,
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
                .type_(axis_type::AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis_type::AxisType::Value))
        .series(Series::Line(
            line::Line::new()
                .name("Email")
                .stack("Total")
                .data(vec![120, 132, 101, 134, 90, 230, 210]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Union Ads")
                .stack("Total")
                .data(vec![220, 182, 191, 234, 290, 330, 310]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Video Ads")
                .stack("Total")
                .data(vec![150, 232, 201, 154, 190, 330, 410]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Direct")
                .stack("Total")
                .data(vec![320, 332, 301, 334, 390, 330, 320]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Search Engine")
                .stack("Total")
                .data(vec![820, 932, 901, 934, 1290, 1330, 1320]),
        ));
    println!("{}", chart.to_string());
}
