use echarts::{
    component::{axis, legend, title, toolbox, tooltip},
    element::{
        axis_label, axis_type, label, mark_line, mark_point, symbol::Symbol, tooltip_trigger,
    },
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .title(title::Title::new().text("Temperature change in the coming week"))
        .tooltip(tooltip::Tooltip::new().trigger(tooltip_trigger::TooltipTrigger::Axis))
        .legend(legend::Legend::new())
        .toolbox(
            toolbox::Toolbox::new().show(true).feature(
                toolbox::Feature::new()
                    .save_as_image(toolbox::SaveAsImage::new())
                    .restore(toolbox::Restore::new())
                    .magic_type(toolbox::MagicType::new().type_(vec![
                        toolbox::MagicTypeType::Line,
                        toolbox::MagicTypeType::Bar,
                    ]))
                    .data_zoom(toolbox::DataZoom::new().y_axis_index("none"))
                    .data_view(toolbox::DataView::new().read_only(false)),
            ),
        )
        .x_axis(
            axis::Axis::new()
                .type_(axis_type::AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(
            axis::Axis::new()
                .type_(axis_type::AxisType::Value)
                .axis_label(axis_label::AxisLabel::new().formatter("{value} °C")),
        )
        .series(Series::Line(
            line::Line::new()
                .name("Highest")
                .data(vec![10, 11, 13, 11, 12, 12, 9])
                .mark_point(mark_point::MarkPoint::new().data(vec![("max", "Max"), ("min", "Min")]))
                .mark_line(mark_line::MarkLine::new().data(
                    vec![mark_line::MarkLineVariant::Simple(
                        mark_line::MarkLineData::new()
                            .type_(mark_line::MarkLineDataType::Average)
                            .name("Avg"),
                    )],
                )),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Lowest")
                .data(vec![1, -2, 2, 5, 3, 2, 0])
                .mark_point(mark_point::MarkPoint::new().data(
                    vec![mark_point::MarkPointData::new()
                        .name("周最低")
                        .value(-2)
                        .x_axis(1)
                        .y_axis(-1.5)],
                ))
                .mark_line(mark_line::MarkLine::new().data(vec![
                        mark_line::MarkLineVariant::Simple(
                            mark_line::MarkLineData::new()
                                .type_(mark_line::MarkLineDataType::Average)
                                .name("Avg"),
                        ),
                        mark_line::MarkLineVariant::StartToEnd(
                            mark_line::MarkLineData::new()
                                .symbol(Symbol::None)
                                .x("90%")
                                .y_axis("max"),
                            mark_line::MarkLineData::new()
                                .type_(mark_line::MarkLineDataType::Max)
                                .name("最高点")
                                .symbol(Symbol::Circle)
                                .label(
                                    label::Label::new()
                                        .position(label::Position::Start)
                                        .formatter("Max"),
                                )
                        ),
                    ])),
        ));

    println!("{}", chart.to_string());
}
