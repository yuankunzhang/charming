use charming::{
    component::{Axis, Grid, Legend},
    element::{
        AxisPointer, AxisPointerType, AxisType, Emphasis, EmphasisFocus, LineStyle, LineStyleType,
        MarkLine, MarkLineData, MarkLineVariant, Tooltip, Trigger,
    },
    series::{bar, Bar, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        )
        .legend(Legend::new())
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
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Direct")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![320, 332, 301, 334, 390, 330, 320]),
        ))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Email")
                .stack("Ad")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![120, 132, 101, 134, 90, 230, 210]),
        ))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Union Ads")
                .stack("Ad")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![220, 182, 191, 234, 290, 330, 310]),
        ))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Video Ads")
                .stack("Ad")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![150, 232, 201, 154, 190, 330, 410]),
        ))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Search Engine")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .mark_line(
                    MarkLine::new()
                        .line_style(LineStyle::new().type_(LineStyleType::Dashed))
                        .data(vec![MarkLineVariant::StartToEnd(
                            MarkLineData::new().type_("min"),
                            MarkLineData::new().type_("max"),
                        )]),
                )
                .data(vec![862, 1018, 964, 1026, 1679, 1600, 1570]),
        ))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Baidu")
                .bar_width(5)
                .stack("Search Engine")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![620, 732, 701, 734, 1090, 1130, 1120]),
        ))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Google")
                .stack("Search Engine")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![120, 132, 101, 134, 290, 230, 220]),
        ))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Bing")
                .stack("Search Engine")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![60, 72, 71, 74, 190, 130, 110]),
        ))
        .series(
            Bar::new()
                .name("Others")
                .stack("Search Engine")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Series))
                .data(vec![62, 82, 91, 84, 109, 110, 120]),
        )
}
