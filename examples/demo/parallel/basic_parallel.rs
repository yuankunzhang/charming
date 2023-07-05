use echarts::{
    component::parallel_axis,
    datatype::value,
    element::line_style,
    series::{parallel, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(0).name("Price"))
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(1).name("Net Weight"))
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(2).name("Amount"))
        .parallel_axis(
            parallel_axis::ParallelAxis::new()
                .dim(3)
                .name("Score")
                .type_(parallel_axis::ParallelAxisType::Category)
                .data(vec!["Excellent", "Good", "OK", "Bad"]),
        )
        .series(Series::Parallel(
            parallel::Parallel::new()
                .line_style(line_style::LineStyle::new().width(4))
                .data(vec![
                    vec![value(12.99), value(100), value(82), value("Good")],
                    vec![value(9.99), value(80), value(77), value("OK")],
                    vec![value(20), value(120), value(60), value("Excellent")],
                ]),
        ))
}
