use echarts::{
    component::axis,
    element::axis_type,
    series::{line, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(axis_type::AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(axis_type::AxisType::Value))
        .series(Series::Line(
            line::Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]),
        ))
}
