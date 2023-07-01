use echarts::{
    component::{axis, grid, legend, title, toolbox, tooltip},
    series::{line, Series},
    utility::{area_style, color, line_style},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .color(vec![
            "#80ffa5".into(),
            "#00ddff".into(),
            "#37a2ff".into(),
            "#ff0087".into(),
            "#ffbf00".into(),
        ])
        .title(title::Title::new().text("Gradient Stacked Area Chart"))
        .tooltip(
            tooltip::Tooltip::new()
                .trigger(tooltip::Trigger::Axis)
                .axis_pointer(
                    tooltip::AxisPointer::new()
                        .type_(tooltip::AxisPointerType::Cross)
                        .label(tooltip::AxisPointerLabel::new().background_color("#6a7985".into())),
                ),
        )
        .legend(legend::Legend::new().data(vec!["Line 1", "Line 2", "Line 3", "Line 4", "Line 5"]))
        .toolbox(
            toolbox::Toolbox::new()
                .feature(toolbox::Feature::new().save_as_image(toolbox::SaveAsImage::new())),
        )
        .grid(
            grid::Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
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
                .name("Line 1")
                .show_symbol(false)
                .stack("Total")
                .line_style(line_style::LineStyle::new().width(0.0))
                .area_style(
                    area_style::AreaStyle::new()
                        .color(color::Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                color::ColorStop::new(0.0, "rgb(128, 255, 165)"),
                                color::ColorStop::new(1.0, "rgb(1, 191, 236)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![140.0, 232.0, 101.0, 264.0, 90.0, 340.0, 250.0]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Line 2")
                .show_symbol(false)
                .stack("Total")
                .line_style(line_style::LineStyle::new().width(0.0))
                .area_style(
                    area_style::AreaStyle::new()
                        .color(color::Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                color::ColorStop::new(0.0, "rgb(0, 221, 255)"),
                                color::ColorStop::new(1.0, "rgb(77, 119, 255)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![120.0, 282.0, 111.0, 234.0, 220.0, 340.0, 310.0]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Line 3")
                .show_symbol(false)
                .stack("Total")
                .line_style(line_style::LineStyle::new().width(0.0))
                .area_style(
                    area_style::AreaStyle::new()
                        .color(color::Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                color::ColorStop::new(0.0, "rgb(55, 162, 255)"),
                                color::ColorStop::new(1.0, "rgb(116, 21, 219)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![320.0, 132.0, 201.0, 334.0, 190.0, 130.0, 220.0]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Line 4")
                .show_symbol(false)
                .stack("Total")
                .line_style(line_style::LineStyle::new().width(0.0))
                .area_style(
                    area_style::AreaStyle::new()
                        .color(color::Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                color::ColorStop::new(0.0, "rgb(255, 0, 135)"),
                                color::ColorStop::new(1.0, "rgb(135, 0, 157)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![220.0, 402.0, 231.0, 134.0, 190.0, 230.0, 120.0]),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Line 5")
                .show_symbol(false)
                .stack("Total")
                .line_style(line_style::LineStyle::new().width(0.0))
                .area_style(
                    area_style::AreaStyle::new()
                        .color(color::Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                color::ColorStop::new(0.0, "rgb(255, 191, 0)"),
                                color::ColorStop::new(1.0, "rgb(224, 62, 76)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![220.0, 302.0, 181.0, 234.0, 210.0, 290.0, 150.0]),
        ));
    println!("{}", chart.to_string());
}
