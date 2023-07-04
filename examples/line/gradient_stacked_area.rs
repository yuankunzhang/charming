use echarts::{
    component::{axis, grid, legend, title, toolbox, tooltip},
    element::{area_style, axis_type, color, line_style, tooltip_trigger},
    series::{line, Series},
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
                .trigger(tooltip_trigger::TooltipTrigger::Axis)
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
                .type_(axis_type::AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis_type::AxisType::Value))
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
                .data(vec![140, 232, 101, 264, 90, 340, 250]),
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
                .data(vec![120, 282, 111, 234, 220, 340, 310]),
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
                .data(vec![320, 132, 201, 334, 190, 130, 220]),
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
                .data(vec![220, 402, 231, 134, 190, 230, 120]),
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
                .data(vec![220, 302, 181, 234, 210, 290, 150]),
        ));
    println!("{}", chart.to_string());
}
