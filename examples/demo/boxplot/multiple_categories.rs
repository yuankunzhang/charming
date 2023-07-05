use echarts::{
    component::{axis, data_zoom, grid, legend, title, tooltip},
    element::{AxisPointer, AxisPointerType, AxisType, SplitArea, SplitLine, TooltipTrigger},
    series::{boxplot, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(
            title::Title::new()
                .text("Multiple Categories")
                .left("center"),
        )
        .legend(legend::Legend::new().top("10%"))
        .tooltip(
            tooltip::Tooltip::new()
                .trigger(TooltipTrigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        )
        .grid(
            grid::Grid::new()
                .left("10%")
                .top("20%")
                .right("10%")
                .bottom("15%"),
        )
        .x_axis(
            axis::Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(true)
                .name_gap(30)
                .split_area(SplitArea::new().show(true))
                .split_line(SplitLine::new().show(false)),
        )
        .y_axis(
            axis::Axis::new()
                .type_(AxisType::Value)
                .name("Value")
                .min(-400)
                .max(600)
                .split_area(SplitArea::new().show(false)),
        )
        .data_zoom(data_zoom::DataZoom::Inside(
            data_zoom::InsideDataZoom::new().start(0).end(20),
        ))
        .data_zoom(data_zoom::DataZoom::Slider(
            data_zoom::SliderDataZoom::new()
                .start(0)
                .end(20)
                .top("90%")
                .x_axis_index(0),
        ))
        .series(Series::Boxplot(
            boxplot::Boxplot::new().name("category0").dataset_index(3),
        ))
        .series(Series::Boxplot(
            boxplot::Boxplot::new().name("category1").dataset_index(4),
        ))
        .series(Series::Boxplot(
            boxplot::Boxplot::new().name("category2").dataset_index(5),
        ))
}
