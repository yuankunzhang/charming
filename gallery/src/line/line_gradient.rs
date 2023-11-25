use charming::{
    component::{Axis, Grid, Title, VisualMap, VisualMapType},
    element::{Tooltip, Trigger},
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    let raw_data = vec![
        ("2000-06-05", 116),
        ("2000-06-06", 129),
        ("2000-06-07", 135),
        ("2000-06-08", 86),
        ("2000-06-09", 73),
        ("2000-06-10", 85),
        ("2000-06-11", 73),
        ("2000-06-12", 68),
        ("2000-06-13", 92),
        ("2000-06-14", 130),
        ("2000-06-15", 245),
        ("2000-06-16", 139),
        ("2000-06-17", 115),
        ("2000-06-18", 111),
        ("2000-06-19", 309),
        ("2000-06-20", 206),
        ("2000-06-21", 137),
        ("2000-06-22", 128),
        ("2000-06-23", 85),
        ("2000-06-24", 94),
        ("2000-06-25", 71),
        ("2000-06-26", 106),
        ("2000-06-27", 84),
        ("2000-06-28", 93),
        ("2000-06-29", 85),
        ("2000-06-30", 73),
        ("2000-07-01", 83),
        ("2000-07-02", 125),
        ("2000-07-03", 107),
        ("2000-07-04", 82),
        ("2000-07-05", 44),
        ("2000-07-06", 72),
        ("2000-07-07", 106),
        ("2000-07-08", 107),
        ("2000-07-09", 66),
        ("2000-07-10", 91),
        ("2000-07-11", 92),
        ("2000-07-12", 113),
        ("2000-07-13", 107),
        ("2000-07-14", 131),
        ("2000-07-15", 111),
        ("2000-07-16", 64),
        ("2000-07-17", 69),
        ("2000-07-18", 88),
        ("2000-07-19", 77),
        ("2000-07-20", 83),
        ("2000-07-21", 111),
        ("2000-07-22", 57),
        ("2000-07-23", 55),
        ("2000-07-24", 60),
    ];

    let mut data_list = vec![];
    let mut value_list = vec![];

    raw_data.into_iter().for_each(|(date, value)| {
        data_list.push(date);
        value_list.push(value);
    });

    Chart::new()
        .title(
            Title::new()
                .text("Gradient along the y axis")
                .left("center"),
        )
        .title(
            Title::new()
                .text("Gradient along the x axis")
                .left("center")
                .top("55%"),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .visual_map(
            VisualMap::new()
                .show(false)
                .type_(VisualMapType::Continuous)
                .series_index(0)
                .min(0)
                .max(400),
        )
        .visual_map(
            VisualMap::new()
                .show(false)
                .type_(VisualMapType::Continuous)
                .series_index(1)
                .dimension(0)
                .min(0)
                .max(10)
                .max((data_list.len() - 1) as f64),
        )
        .x_axis(Axis::new().data(data_list.to_vec()))
        .x_axis(
            Axis::new()
                .data(data_list.to_vec())
                .grid_index(1),
        )
        .y_axis(Axis::new())
        .y_axis(Axis::new().grid_index(1))
        .grid(Grid::new().bottom("60%"))
        .grid(Grid::new().top("60%"))
        .series(Line::new().show_symbol(false).data(value_list.clone()))
        .series(
            Line::new()
                .show_symbol(false)
                .x_axis_index(1)
                .y_axis_index(1)
                .data(value_list),
        )
}
