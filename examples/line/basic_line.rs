use echarts::{
    component::axis,
    element::axis_attr,
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(axis_attr::AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis_attr::AxisType::Value))
        .series(Series::Line(
            line::Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]),
        ));
    println!("{}", chart.to_string());
}
