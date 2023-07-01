use echarts::{
    component::axis,
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(axis::AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis::AxisType::Value))
        .series(Series::Line(
            line::Line::new().data(vec![150.0, 230.0, 224.0, 218.0, 135.0, 147.0, 260.0]),
        ));
    println!("{}", chart.to_string());
}
