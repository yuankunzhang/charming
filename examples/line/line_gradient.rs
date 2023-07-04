use echarts::{
    component::{axis, grid, title, tooltip, visual_map},
    datatype::value,
    element::tooltip_trigger,
    series::{line, Series},
    Chart,
};

fn main() {
    let raw_data = vec![
        (value("2000-06-05"), value(116)),
        (value("2000-06-06"), value(129)),
        (value("2000-06-07"), value(135)),
        (value("2000-06-08"), value(86)),
        (value("2000-06-09"), value(73)),
        (value("2000-06-10"), value(85)),
        (value("2000-06-11"), value(73)),
        (value("2000-06-12"), value(68)),
        (value("2000-06-13"), value(92)),
        (value("2000-06-14"), value(130)),
        (value("2000-06-15"), value(245)),
        (value("2000-06-16"), value(139)),
        (value("2000-06-17"), value(115)),
        (value("2000-06-18"), value(111)),
        (value("2000-06-19"), value(309)),
        (value("2000-06-20"), value(206)),
        (value("2000-06-21"), value(137)),
        (value("2000-06-22"), value(128)),
        (value("2000-06-23"), value(85)),
        (value("2000-06-24"), value(94)),
        (value("2000-06-25"), value(71)),
        (value("2000-06-26"), value(106)),
        (value("2000-06-27"), value(84)),
        (value("2000-06-28"), value(93)),
        (value("2000-06-29"), value(85)),
        (value("2000-06-30"), value(73)),
        (value("2000-07-01"), value(83)),
        (value("2000-07-02"), value(125)),
        (value("2000-07-03"), value(107)),
        (value("2000-07-04"), value(82)),
        (value("2000-07-05"), value(44)),
        (value("2000-07-06"), value(72)),
        (value("2000-07-07"), value(106)),
        (value("2000-07-08"), value(107)),
        (value("2000-07-09"), value(66)),
        (value("2000-07-10"), value(91)),
        (value("2000-07-11"), value(92)),
        (value("2000-07-12"), value(113)),
        (value("2000-07-13"), value(107)),
        (value("2000-07-14"), value(131)),
        (value("2000-07-15"), value(111)),
        (value("2000-07-16"), value(64)),
        (value("2000-07-17"), value(69)),
        (value("2000-07-18"), value(88)),
        (value("2000-07-19"), value(77)),
        (value("2000-07-20"), value(83)),
        (value("2000-07-21"), value(111)),
        (value("2000-07-22"), value(57)),
        (value("2000-07-23"), value(55)),
        (value("2000-07-24"), value(60)),
    ];

    let mut data_list = vec![];
    let mut value_list = vec![];

    raw_data.into_iter().for_each(|(date, value)| {
        data_list.push(date);
        value_list.push(value);
    });

    let chart = Chart::new()
        .title(
            title::Title::new()
                .text("Gradient along the y axis")
                .left("center"),
        )
        .title(
            title::Title::new()
                .text("Gradient along the x axis")
                .left("center")
                .top("55%"),
        )
        .tooltip(tooltip::Tooltip::new().trigger(tooltip_trigger::TooltipTrigger::Axis))
        .visual_map(
            visual_map::VisualMap::new()
                .show(false)
                .type_(visual_map::VisualMapType::Continuous)
                .series_index(0)
                .min(0)
                .max(400),
        )
        .visual_map(
            visual_map::VisualMap::new()
                .show(false)
                .type_(visual_map::VisualMapType::Continuous)
                .series_index(1)
                .dimension(0)
                .min(0)
                .max(10)
                .max((data_list.len() - 1) as f64),
        )
        .x_axis(axis::Axis::new().data(data_list.iter().map(|v| v.as_str().unwrap()).collect()))
        .x_axis(
            axis::Axis::new()
                .data(data_list.iter().map(|v| v.as_str().unwrap()).collect())
                .grid_index(1),
        )
        .y_axis(axis::Axis::new())
        .y_axis(axis::Axis::new().grid_index(1))
        .grid(grid::Grid::new().bottom("60%"))
        .grid(grid::Grid::new().top("60%"))
        .series(Series::Line(
            line::Line::new()
                .show_symbol(false)
                .data(value_list.clone()),
        ))
        .series(Series::Line(
            line::Line::new()
                .show_symbol(false)
                .x_axis_index(1)
                .y_axis_index(1)
                .data(value_list),
        ));

    println!("{}", chart.to_string());
}
