use echarts::{
    component::axis,
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(axis::Type::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis::Type::Value))
        .series(Series::Line(
            line::Line::new()
                .smooth(0.5)
                .data(vec![150, 230, 224, 218, 135, 147, 260]),
        ));
    println!("{}", chart.to_string());
}
