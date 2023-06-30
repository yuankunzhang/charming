// main.rs
mod chart;
mod component;
mod dataset;
mod renderer;
mod series;
mod style;

use chart::*;
use component::color::*;
use series::*;

fn main() {
    // let s = renderer::render_string().unwrap();
    // println!("{}", s);
    gradient_stacked_area_chart();
}

fn gradient_stacked_area_chart() {
    let opt = Chart::new()
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
            series::line::Line::new()
                .name("Line 1")
                .show_symbol(false)
                .stack("Total")
                .line_style(series::line::LineStyle::new().width(0.0))
                .area_style(
                    series::line::AreaStyle::new()
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
                .data(vec![
                    140.0.into(),
                    232.0.into(),
                    101.0.into(),
                    264.0.into(),
                    90.0.into(),
                    340.0.into(),
                    250.0.into(),
                ]),
        ))
        .series(Series::Line(
            series::line::Line::new()
                .name("Line 2")
                .show_symbol(false)
                .stack("Total")
                .line_style(series::line::LineStyle::new().width(0.0))
                .area_style(
                    series::line::AreaStyle::new()
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
                .data(vec![
                    120.0.into(),
                    282.0.into(),
                    111.0.into(),
                    234.0.into(),
                    220.0.into(),
                    340.0.into(),
                    310.0.into(),
                ]),
        ))
        .series(Series::Line(
            series::line::Line::new()
                .name("Line 3")
                .show_symbol(false)
                .stack("Total")
                .line_style(series::line::LineStyle::new().width(0.0))
                .area_style(
                    series::line::AreaStyle::new()
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
                .data(vec![
                    320.0.into(),
                    132.0.into(),
                    201.0.into(),
                    334.0.into(),
                    190.0.into(),
                    130.0.into(),
                    220.0.into(),
                ]),
        ))
        .series(Series::Line(
            series::line::Line::new()
                .name("Line 4")
                .show_symbol(false)
                .stack("Total")
                .line_style(series::line::LineStyle::new().width(0.0))
                .area_style(
                    series::line::AreaStyle::new()
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
                .data(vec![
                    220.0.into(),
                    402.0.into(),
                    231.0.into(),
                    134.0.into(),
                    190.0.into(),
                    230.0.into(),
                    120.0.into(),
                ]),
        ))
        .series(Series::Line(
            series::line::Line::new()
                .name("Line 5")
                .show_symbol(false)
                .stack("Total")
                .line_style(series::line::LineStyle::new().width(0.0))
                .area_style(
                    series::line::AreaStyle::new()
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
                .data(vec![
                    220.0.into(),
                    302.0.into(),
                    181.0.into(),
                    234.0.into(),
                    210.0.into(),
                    290.0.into(),
                    150.0.into(),
                ]),
        ));
    println!("{}", serde_json::to_string_pretty(&opt).unwrap());
}
