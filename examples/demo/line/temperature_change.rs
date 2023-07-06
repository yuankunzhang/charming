use echarts::{
    component::{
        Axis, Legend, Title, Toolbox, ToolboxDataView, ToolboxDataZoom, ToolboxFeature,
        ToolboxMagicType, ToolboxMagicTypeType, ToolboxRestore, ToolboxSaveAsImage, Tooltip,
    },
    element::{
        AxisLabel, AxisType, Label, MarkLine, MarkLineData, MarkLineDataType, MarkLineVariant,
        MarkPoint, MarkPointData, Position, Symbol, TooltipTrigger,
    },
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Temperature change in the coming week"))
        .tooltip(Tooltip::new().trigger(TooltipTrigger::Axis))
        .legend(Legend::new())
        .toolbox(
            Toolbox::new().show(true).feature(
                ToolboxFeature::new()
                    .save_as_image(ToolboxSaveAsImage::new())
                    .restore(ToolboxRestore::new())
                    .magic_type(
                        ToolboxMagicType::new()
                            .type_(vec![ToolboxMagicTypeType::Line, ToolboxMagicTypeType::Bar]),
                    )
                    .data_zoom(ToolboxDataZoom::new().y_axis_index("none"))
                    .data_view(ToolboxDataView::new().read_only(false)),
            ),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .axis_label(AxisLabel::new().formatter("{value} °C")),
        )
        .series(
            Line::new()
                .name("Highest")
                .data(vec![10, 11, 13, 11, 12, 12, 9])
                .mark_point(MarkPoint::new().data(vec![("max", "Max"), ("min", "Min")]))
                .mark_line(MarkLine::new().data(vec![MarkLineVariant::Simple(
                        MarkLineData::new()
                            .type_(MarkLineDataType::Average)
                            .name("Avg"),
                    )])),
        )
        .series(
            Line::new()
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
        )
}
