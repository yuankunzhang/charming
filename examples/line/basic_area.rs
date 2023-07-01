use echarts::{
    basic::area_style,
    component::axis,
    series::{line, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(axis::Type::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis::Type::Value))
        .series(Series::Line(
            line::Line::new()
                .area_style(area_style::AreaStyle::new())
                .data(vec![150, 230, 224, 218, 135, 147, 260]),
        ));
    println!("{}", chart.to_string());
}
