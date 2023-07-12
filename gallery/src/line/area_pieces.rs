use charming::{
    component::{Axis, VisualMap, VisualMapPiece, VisualMapType},
    df,
    element::{
        AreaStyle, AxisType, Label, LineStyle, MarkLine, MarkLineData, MarkLineVariant, Symbol,
    },
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(Axis::new().type_(AxisType::Category).boundary_gap(false))
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .boundary_gap(("0", "20%")),
        )
        .visual_map(
            VisualMap::new()
                .type_(VisualMapType::Piecewise)
                .show(false)
                .dimension(0)
                .series_index(0)
                .pieces(vec![
                    VisualMapPiece::new()
                        .min(1)
                        .max(3)
                        .color("rgba(0, 0, 180, 0.4)"),
                    VisualMapPiece::new()
                        .min(5)
                        .max(7)
                        .color("rgba(0, 0, 180, 0.4)"),
                ]),
        )
        .series(
            Line::new()
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
                .data(df![
                    ["2019-10-10", 200],
                    ["2019-10-11", 560],
                    ["2019-10-12", 750],
                    ["2019-10-13", 580],
                    ["2019-10-14", 250],
                    ["2019-10-15", 300],
                    ["2019-10-16", 450],
                    ["2019-10-17", 300],
                    ["2019-10-18", 100],
                ]),
        )
}
