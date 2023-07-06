use echarts::{
    component::{ParallelAxis, ParallelAxisType},
    element::LineStyle,
    row,
    series::Parallel,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .parallel_axis(ParallelAxis::new().dim(0).name("Price"))
        .parallel_axis(ParallelAxis::new().dim(1).name("Net Weight"))
        .parallel_axis(ParallelAxis::new().dim(2).name("Amount"))
        .parallel_axis(
            ParallelAxis::new()
                .dim(3)
                .name("Score")
                .type_(ParallelAxisType::Category)
                .data(vec!["Excellent", "Good", "OK", "Bad"]),
        )
        .series(
            Parallel::new()
                .line_style(LineStyle::new().width(4))
                .data(vec![
                    row![12.99, 100, 82, "Good"],
                    row![9.99, 80, 77, "OK"],
                    row![20, 120, 60, "Excellent"],
                ]),
        )
}
