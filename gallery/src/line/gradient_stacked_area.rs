use charming::{
    component::{Axis, Feature, Grid, Legend, SaveAsImage, Title, Toolbox},
    element::{
        AreaStyle, AxisPointer, AxisPointerType, AxisType, Color, ColorStop, Label, LineStyle,
        Tooltip, Trigger,
    },
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .color(vec![
            "#80ffa5".into(),
            "#00ddff".into(),
            "#37a2ff".into(),
            "#ff0087".into(),
            "#ffbf00".into(),
        ])
        .title(Title::new().text("Gradient Stacked Area Chart"))
        .tooltip(
            Tooltip::new().trigger(Trigger::Axis).axis_pointer(
                AxisPointer::new()
                    .type_(AxisPointerType::Cross)
                    .label(Label::new().background_color("#6a7985")),
            ),
        )
        .legend(Legend::new().data(vec!["Line 1", "Line 2", "Line 3", "Line 4", "Line 5"]))
        .toolbox(Toolbox::new().feature(Feature::new().save_as_image(SaveAsImage::new())))
        .grid(
            Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Line::new()
                .name("Line 1")
                .show_symbol(false)
                .stack("Total")
                .line_style(LineStyle::new().width(0.0))
                .area_style(
                    AreaStyle::new()
                        .color(Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                ColorStop::new(0.0, "rgb(128, 255, 165)"),
                                ColorStop::new(1.0, "rgb(1, 191, 236)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![140, 232, 101, 264, 90, 340, 250]),
        )
        .series(
            Line::new()
                .name("Line 2")
                .show_symbol(false)
                .stack("Total")
                .line_style(LineStyle::new().width(0.0))
                .area_style(
                    AreaStyle::new()
                        .color(Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                ColorStop::new(0.0, "rgb(0, 221, 255)"),
                                ColorStop::new(1.0, "rgb(77, 119, 255)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![120, 282, 111, 234, 220, 340, 310]),
        )
        .series(
            Line::new()
                .name("Line 3")
                .show_symbol(false)
                .stack("Total")
                .line_style(LineStyle::new().width(0.0))
                .area_style(
                    AreaStyle::new()
                        .color(Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                ColorStop::new(0.0, "rgb(55, 162, 255)"),
                                ColorStop::new(1.0, "rgb(116, 21, 219)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![320, 132, 201, 334, 190, 130, 220]),
        )
        .series(
            Line::new()
                .name("Line 4")
                .show_symbol(false)
                .stack("Total")
                .line_style(LineStyle::new().width(0.0))
                .area_style(
                    AreaStyle::new()
                        .color(Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                ColorStop::new(0.0, "rgb(255, 0, 135)"),
                                ColorStop::new(1.0, "rgb(135, 0, 157)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![220, 402, 231, 134, 190, 230, 120]),
        )
        .series(
            Line::new()
                .name("Line 5")
                .show_symbol(false)
                .stack("Total")
                .line_style(LineStyle::new().width(0.0))
                .area_style(
                    AreaStyle::new()
                        .color(Color::LinearGradient {
                            x: 0.0,
                            y: 0.0,
                            x2: 0.0,
                            y2: 1.0,
                            color_stops: vec![
                                ColorStop::new(0.0, "rgb(255, 191, 0)"),
                                ColorStop::new(1.0, "rgb(224, 62, 76)"),
                            ],
                        })
                        .opacity(0.8),
                )
                .smooth(0.5)
                .data(vec![220, 302, 181, 234, 210, 290, 150]),
        )
}
