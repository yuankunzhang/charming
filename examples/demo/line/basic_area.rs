use echarts::{
    component::axis,
    element::{AreaStyle, AxisType},
    series::{line, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            axis::Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(axis::Axis::new().type_(AxisType::Value))
        .series(Series::Line(
            line::Line::new()
                .area_style(AreaStyle::new())
                .data(vec![150, 230, 224, 218, 135, 147, 260]),
        ))
}
