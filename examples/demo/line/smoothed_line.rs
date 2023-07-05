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
            line::Line::new()
                .smooth(0.5)
                .data(vec![820, 932, 901, 934, 1290, 1330, 1320]),
        ))
}
