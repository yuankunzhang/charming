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
            line::Line::new()
                .smooth(0.5)
                .data(vec![820, 932, 901, 934, 1290, 1330, 1320]),
        ));
    println!("{}", chart.to_string());
}
