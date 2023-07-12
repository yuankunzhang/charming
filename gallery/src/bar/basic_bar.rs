use charming::{component::Axis, element::AxisType, series::Bar, Chart};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Bar::new().data(vec![120, 200, 150, 80, 70, 110, 130]))
}
