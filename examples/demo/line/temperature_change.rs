use echarts::{
    component::{axis, legend, title, toolbox, tooltip},
    element::{
        AxisLabel, AxisType, Label, MarkLine, MarkLineData, MarkLineDataType, MarkLineVariant,
        MarkPoint, MarkPointData, Position, Symbol, TooltipTrigger,
    },
    series::{line, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(title::Title::new().text("Temperature change in the coming week"))
        .tooltip(tooltip::Tooltip::new().trigger(TooltipTrigger::Axis))
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
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(
            axis::Axis::new()
                .type_(AxisType::Value)
                .axis_label(AxisLabel::new().formatter("{value} °C")),
        )
        .series(Series::Line(
            line::Line::new()
                .name("Highest")
                .data(vec![10, 11, 13, 11, 12, 12, 9])
                .mark_point(MarkPoint::new().data(vec![("max", "Max"), ("min", "Min")]))
                .mark_line(MarkLine::new().data(vec![MarkLineVariant::Simple(
                        MarkLineData::new()
                            .type_(MarkLineDataType::Average)
                            .name("Avg"),
                    )])),
        ))
        .series(Series::Line(
            line::Line::new()
                .name("Lowest")
                .data(vec![1, -2, 2, 5, 3, 2, 0])
                .mark_point(MarkPoint::new().data(vec![MarkPointData::new()
                        .name("周最低")
                        .value(-2)
                        .x_axis(1)
                        .y_axis(-1.5)]))
                .mark_line(MarkLine::new().data(vec![
                        MarkLineVariant::Simple(
                            MarkLineData::new()
                                .type_(MarkLineDataType::Average)
                                .name("Avg"),
                        ),
                        MarkLineVariant::StartToEnd(
                            MarkLineData::new()
                                .symbol(Symbol::None)
                                .x("90%")
                                .y_axis("max"),
                            MarkLineData::new()
                                .type_(MarkLineDataType::Max)
                                .name("最高点")
                                .symbol(Symbol::Circle)
                                .label(
                                    Label::new()
                                        .position(Position::Start)
                                        .formatter("Max"),
                                )
                        ),
                    ])),
        ))
}
