use charming::{
    component::ParallelAxis,
    df,
    element::{AxisType, LineStyle},
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
                .type_(AxisType::Category)
                .data(vec!["Excellent", "Good", "OK", "Bad"]),
        )
        .series(
            Parallel::new()
                .line_style(LineStyle::new().width(4))
                .data(df![
                    [12.99, 100, 82, "Good"],
                    [9.99, 80, 77, "OK"],
                    [20, 120, 60, "Excellent"],
                ]),
        )
}
