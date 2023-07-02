use echarts::{
    basic::{self, area_style, label, line_style, symbol},
    component::{axis, visual_map},
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(axis::Type::Category)
                .boundary_gap(false),
        )
        .y_axis(
            axis::Axis::new()
                .type_(axis::Type::Value)
                .boundary_gap(("0", "20%")),
        )
        .visual_map(
            visual_map::VisualMap::new().map(
                visual_map::VisualMapItem::new()
                    .type_("piecewise")
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
            ),
        )
        .series(Series::Line(
            line::Line::new()
                .smooth(0.6)
                .symbol(symbol::Symbol::None)
                .line_style(line_style::LineStyle::new().width(5).color("#5470C6"))
                .area_style(area_style::AreaStyle::new())
                .mark_line(
                    line::MarkLine::new()
                        .symbol(vec![symbol::Symbol::None, symbol::Symbol::None])
                        .label(label::Label::new().show(false))
                        .data(vec![
                            line::MarkLineVariant::Simple(line::MarkLineData::new().x_axis(1)),
                            line::MarkLineVariant::Simple(line::MarkLineData::new().x_axis(3)),
                            line::MarkLineVariant::Simple(line::MarkLineData::new().x_axis(5)),
                            line::MarkLineVariant::Simple(line::MarkLineData::new().x_axis(7)),
                        ]),
                )
                .data(vec![
                    vec![basic::value("2019-10-10"), basic::value(200)],
                    vec![basic::value("2019-10-11"), basic::value(560)],
                    vec![basic::value("2019-10-12"), basic::value(750)],
                    vec![basic::value("2019-10-13"), basic::value(580)],
                    vec![basic::value("2019-10-14"), basic::value(250)],
                    vec![basic::value("2019-10-15"), basic::value(300)],
                    vec![basic::value("2019-10-16"), basic::value(450)],
                    vec![basic::value("2019-10-17"), basic::value(300)],
                    vec![basic::value("2019-10-18"), basic::value(100)],
                ]),
        ));

    println!("{}", chart.to_string());
}
