use echarts::{
    component::{axis, visual_map},
    element::{self, area_style, label, line_style, symbol},
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
                    vec![element::value("2019-10-10"), element::value(200)],
                    vec![element::value("2019-10-11"), element::value(560)],
                    vec![element::value("2019-10-12"), element::value(750)],
                    vec![element::value("2019-10-13"), element::value(580)],
                    vec![element::value("2019-10-14"), element::value(250)],
                    vec![element::value("2019-10-15"), element::value(300)],
                    vec![element::value("2019-10-16"), element::value(450)],
                    vec![element::value("2019-10-17"), element::value(300)],
                    vec![element::value("2019-10-18"), element::value(100)],
                ]),
        ));

    println!("{}", chart.to_string());
}
