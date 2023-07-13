use charming::{
    component::{Axis, Grid, Title},
    df,
    element::{
        Label, LabelAlign, LineStyle, LineStyleType, MarkLine, MarkLineData, MarkLineVariant,
        Symbol, Tooltip,
    },
    series::Scatter,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(
            Title::new()
                .text("Anscombe's quartet")
                .left("center")
                .top(0),
        )
        .grid(Grid::new().left("7%").top("7%").width("38%").height("38%"))
        .grid(Grid::new().right("7%").top("7%").width("38%").height("38%"))
        .grid(
            Grid::new()
                .left("7%")
                .bottom("7%")
                .width("38%")
                .height("38%"),
        )
        .grid(
            Grid::new()
                .right("7%")
                .bottom("7%")
                .width("38%")
                .height("38%"),
        )
        .tooltip(Tooltip::new().formatter("Group {a}: ({c})"))
        .x_axis(Axis::new().grid_index(0).min(0).max(20))
        .x_axis(Axis::new().grid_index(1).min(0).max(20))
        .x_axis(Axis::new().grid_index(2).min(0).max(20))
        .x_axis(Axis::new().grid_index(3).min(0).max(20))
        .y_axis(Axis::new().grid_index(0).min(0).max(15))
        .y_axis(Axis::new().grid_index(1).min(0).max(15))
        .y_axis(Axis::new().grid_index(2).min(0).max(15))
        .y_axis(Axis::new().grid_index(3).min(0).max(15))
        .series(
            Scatter::new()
                .name("I")
                .x_axis_index(0)
                .y_axis_index(0)
                .mark_line(
                    MarkLine::new()
                        .label(
                            Label::new()
                                .formatter("y = 0.5 * x + 3")
                                .align(LabelAlign::Right),
                        )
                        .line_style(LineStyle::new().type_(LineStyleType::Solid))
                        .data(vec![MarkLineVariant::StartToEnd(
                            MarkLineData::new().symbol(Symbol::None).coord(vec![0, 3]),
                            MarkLineData::new().symbol(Symbol::None).coord(vec![20, 13]),
                        )]),
                )
                .data(df![
                    [10.0, 8.04],
                    [8.0, 6.95],
                    [13.0, 7.58],
                    [9.0, 8.81],
                    [11.0, 8.33],
                    [14.0, 9.96],
                    [6.0, 7.24],
                    [4.0, 4.26],
                    [12.0, 10.84],
                    [7.0, 4.82],
                    [5.0, 5.68]
                ]),
        )
        .series(
            Scatter::new()
                .name("II")
                .x_axis_index(1)
                .y_axis_index(1)
                .mark_line(
                    MarkLine::new()
                        .label(
                            Label::new()
                                .formatter("y = 0.5 * x + 3")
                                .align(LabelAlign::Right),
                        )
                        .line_style(LineStyle::new().type_(LineStyleType::Solid))
                        .data(vec![MarkLineVariant::StartToEnd(
                            MarkLineData::new().symbol(Symbol::None).coord(vec![0, 3]),
                            MarkLineData::new().symbol(Symbol::None).coord(vec![20, 13]),
                        )]),
                )
                .data(df![
                    [10.0, 9.14],
                    [8.0, 8.14],
                    [13.0, 8.74],
                    [9.0, 8.77],
                    [11.0, 9.26],
                    [14.0, 8.1],
                    [6.0, 6.13],
                    [4.0, 3.1],
                    [12.0, 9.13],
                    [7.0, 7.26],
                    [5.0, 4.74]
                ]),
        )
        .series(
            Scatter::new()
                .name("III")
                .x_axis_index(2)
                .y_axis_index(2)
                .mark_line(
                    MarkLine::new()
                        .label(
                            Label::new()
                                .formatter("y = 0.5 * x + 3")
                                .align(LabelAlign::Right),
                        )
                        .line_style(LineStyle::new().type_(LineStyleType::Solid))
                        .data(vec![MarkLineVariant::StartToEnd(
                            MarkLineData::new().symbol(Symbol::None).coord(vec![0, 3]),
                            MarkLineData::new().symbol(Symbol::None).coord(vec![20, 13]),
                        )]),
                )
                .data(df![
                    [10.0, 7.46],
                    [8.0, 6.77],
                    [13.0, 12.74],
                    [9.0, 7.11],
                    [11.0, 7.81],
                    [14.0, 8.84],
                    [6.0, 6.08],
                    [4.0, 5.39],
                    [12.0, 8.15],
                    [7.0, 6.42],
                    [5.0, 5.73]
                ]),
        )
        .series(
            Scatter::new()
                .name("IV")
                .x_axis_index(3)
                .y_axis_index(3)
                .mark_line(
                    MarkLine::new()
                        .label(
                            Label::new()
                                .formatter("y = 0.5 * x + 3")
                                .align(LabelAlign::Right),
                        )
                        .line_style(LineStyle::new().type_(LineStyleType::Solid))
                        .data(vec![MarkLineVariant::StartToEnd(
                            MarkLineData::new().symbol(Symbol::None).coord(vec![0, 3]),
                            MarkLineData::new().symbol(Symbol::None).coord(vec![20, 13]),
                        )]),
                )
                .data(df![
                    [8.0, 6.58],
                    [8.0, 5.76],
                    [8.0, 7.71],
                    [8.0, 8.84],
                    [8.0, 8.47],
                    [8.0, 7.04],
                    [8.0, 5.25],
                    [19.0, 12.5],
                    [8.0, 5.56],
                    [8.0, 7.91],
                    [8.0, 6.89]
                ]),
        )
}
