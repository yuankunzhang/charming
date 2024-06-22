use charming::{
    component::Axis,
    datatype::DataPointItem,
    df,
    element::{AxisType, ItemStyle},
    series::Bar,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Bar::new().data(df![
            120,
            DataPointItem::new(200).item_style(ItemStyle::new().color("#a90000")),
            150,
            80,
            70,
            110,
            130,
        ]))
}
