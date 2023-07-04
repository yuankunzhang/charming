use echarts::{
    component::{axis, visual_map},
    datatype::value,
    element::{area_style, axis_type, label, line_style, mark_line, symbol},
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(axis_type::AxisType::Category)
                .boundary_gap(false),
        )
        .y_axis(
            axis::Axis::new()
                .type_(axis_type::AxisType::Value)
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
                .symbol(symbol::Symbol::None)
                .line_style(line_style::LineStyle::new().width(5).color("#5470C6"))
                .area_style(area_style::AreaStyle::new())
                .mark_line(
                    mark_line::MarkLine::new()
                        .symbol(vec![symbol::Symbol::None, symbol::Symbol::None])
                        .label(label::Label::new().show(false))
                        .data(vec![
                            mark_line::MarkLineVariant::Simple(
                                mark_line::MarkLineData::new().x_axis(1),
                            ),
                            mark_line::MarkLineVariant::Simple(
                                mark_line::MarkLineData::new().x_axis(3),
                            ),
                            mark_line::MarkLineVariant::Simple(
                                mark_line::MarkLineData::new().x_axis(5),
                            ),
                            mark_line::MarkLineVariant::Simple(
                                mark_line::MarkLineData::new().x_axis(7),
                            ),
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
        ));

    println!("{}", chart.to_string());
}
