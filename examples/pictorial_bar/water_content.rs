use echarts::{
    component::{axis, grid, legend, tooltip},
    element::{axis_label, axis_line, axis_tick, mark_line, split_line},
    series::{pictorial_bar, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .tooltip(tooltip::Tooltip::new())
        .legend(
            legend::Legend::new()
                .selected_mode(legend::SelectedMode::Single)
                .data(vec!["typeA", "typeB"]),
        )
        .x_axis(
            axis::Axis::new()
                .axis_tick(axis_tick::AxisTick::new().show(false))
                .axis_line(axis_line::AxisLine::new().show(false))
                .axis_label(axis_label::AxisLabel::new().show(false))
                .data(vec!["a", "b", "c", "d", "e"]),
        )
        .y_axis(
            axis::Axis::new()
                .max(150)
                .offset(20)
                .split_line(split_line::SplitLine::new().show(false)),
        )
        .grid(grid::Grid::new().top("center").height("230"))
        .mark_line(mark_line::MarkLine::new().z(-100))
        .series(Series::PictorialBar(
            pictorial_bar::PictorialBar::new().name("typeA"),
        ));
}
