use charming::{
    component::{Axis, Grid, Legend, Title},
    element::{AxisPointer, AxisPointerType, AxisType, Tooltip, Trigger},
    series::Bar,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("World Population"))
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        )
        .legend(Legend::new())
        .grid(
            Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .boundary_gap(("0", "0.01")),
        )
        .y_axis(Axis::new().type_(AxisType::Category).data(vec![
            "Brazil",
            "Indonesia",
            "USA",
            "India",
            "China",
            "World",
        ]))
        .series(
            Bar::new()
                .name("2011")
                .data(vec![18203, 23489, 29034, 104970, 131744, 630230]),
        )
        .series(
            Bar::new()
                .name("2012")
                .data(vec![19325, 23438, 31000, 121594, 134141, 681807]),
        )
}
