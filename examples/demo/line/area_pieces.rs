use echarts::{
    component::{axis, visual_map},
    datatype::value,
    element::{
        AreaStyle, AxisType, Label, LineStyle, MarkLine, MarkLineData, MarkLineVariant, Symbol,
    },
    series::{line, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false),
        )
        .y_axis(
            axis::Axis::new()
                .type_(AxisType::Value)
                .boundary_gap(("0", "20%")),
        )
        .visual_map(
            visual_map::VisualMap::new()
                .type_(visual_map::VisualMapType::Piecewise)
                .show(false)
                .dimension(0)
                .series_index(0)
                .pieces(vec![
                    visual_map::Piece::new()
                        .min(1)
                        .max(3)
                        .color("rgba(0, 0, 180, 0.4)"),
                    visual_map::Piece::new()
                        .min(5)
                        .max(7)
                        .color("rgba(0, 0, 180, 0.4)"),
                ]),
        )
        .series(Series::Line(
            line::Line::new()
                .smooth(0.6)
                .symbol(Symbol::None)
                .line_style(LineStyle::new().width(5).color("#5470C6"))
                .area_style(AreaStyle::new())
                .mark_line(
                    MarkLine::new()
                        .symbol(vec![Symbol::None, Symbol::None])
                        .label(Label::new().show(false))
                        .data(vec![
                            MarkLineVariant::Simple(MarkLineData::new().x_axis(1)),
                            MarkLineVariant::Simple(MarkLineData::new().x_axis(3)),
                            MarkLineVariant::Simple(MarkLineData::new().x_axis(6)),
                            MarkLineVariant::Simple(MarkLineData::new().x_axis(7)),
                        ]),
                )
                .data(vec![
                    vec![value("2019-10-10"), value(200)],
                    vec![value("2019-10-11"), value(560)],
                    vec![value("2019-10-12"), value(750)],
                    vec![value("2019-10-13"), value(580)],
                    vec![value("2019-10-14"), value(250)],
                    vec![value("2019-10-15"), value(300)],
                    vec![value("2019-10-16"), value(450)],
                    vec![value("2019-10-17"), value(300)],
                    vec![value("2019-10-18"), value(100)],
                ]),
        ))
}
