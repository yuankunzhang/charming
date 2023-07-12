use charming::{component::Axis, element::AxisType, series::Line, Chart};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Line::new()
                .smooth(0.5)
                .data(vec![820, 932, 901, 934, 1290, 1330, 1320]),
        )
}
