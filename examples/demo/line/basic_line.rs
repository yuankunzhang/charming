use echarts::{
    component::axis,
    element::AxisType,
    series::{line, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(AxisType::Value))
        .series(Series::Line(
            line::Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]),
        ))
}
