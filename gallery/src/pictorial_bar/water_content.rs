use charming::{
    component::{Axis, Grid, Legend, LegendSelectedMode},
    element::{AxisLabel, AxisLine, AxisTick, MarkLine, SplitLine, Tooltip},
    series::PictorialBar,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .tooltip(Tooltip::new())
        .legend(
            Legend::new()
                .selected_mode(LegendSelectedMode::Single)
                .data(vec!["typeA", "typeB"]),
        )
        .x_axis(
            Axis::new()
                .axis_tick(AxisTick::new().show(false))
                .axis_line(AxisLine::new().show(false))
                .axis_label(AxisLabel::new().show(false))
                .data(vec!["a", "b", "c", "d", "e"]),
        )
        .y_axis(
            Axis::new()
                .max(150)
                .offset(20)
                .split_line(SplitLine::new().show(false)),
        )
        .grid(Grid::new().top("center").height("230"))
        .mark_line(MarkLine::new().z(-100))
        .series(PictorialBar::new().name("typeA"))
}
