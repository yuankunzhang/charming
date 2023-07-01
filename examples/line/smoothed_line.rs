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
            line::Line::new()
                .smooth(0.5)
                .data(vec![820.0, 932.0, 901.0, 934.0, 1290.0, 1330.0, 1320.0]),
        ));
    println!("{}", chart.to_string());
}
