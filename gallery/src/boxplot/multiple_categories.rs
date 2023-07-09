use echarts::{
    component::{
        Axis, AxisPointer, AxisPointerType, DataZoom, DataZoomType, Grid, Legend, Title, Tooltip,
    },
    element::{AxisType, SplitArea, SplitLine, TooltipTrigger},
    series::Boxplot,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Multiple Categories").left("center"))
        .legend(Legend::new().top("10%"))
        .tooltip(
            Tooltip::new()
                .trigger(TooltipTrigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        )
        .grid(
            Grid::new()
                .left("10%")
                .top("20%")
                .right("10%")
                .bottom("15%"),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(true)
                .name_gap(30)
                .split_area(SplitArea::new().show(true))
                .split_line(SplitLine::new().show(false)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Value")
                .min(-400)
                .max(600)
                .split_area(SplitArea::new().show(false)),
        )
        .data_zoom(DataZoom::new().type_(DataZoomType::Inside).start(0).end(20))
        .data_zoom(
            DataZoom::new()
                .type_(DataZoomType::Slider)
                .start(0)
                .end(20)
                .top("90%")
                .x_axis_index(0),
        )
        .series(Boxplot::new().name("category0").dataset_index(3))
        .series(Boxplot::new().name("category1").dataset_index(4))
        .series(Boxplot::new().name("category2").dataset_index(5))
}
