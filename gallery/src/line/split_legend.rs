use charming::{
    component::{Axis, Feature, Grid, Legend, SaveAsImage, Title, Toolbox},
    element::{AxisType, Tooltip, Trigger},
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Split Legend"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(vec![
            Legend::new().data(vec![
                "Penguins",
                "Giraffes",
                "Lions",
                "Elephants",
                "Monkeys",
            ]),
            Legend::new().data(vec!["Total"]).right("50"),
        ])
        .grid(
            Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .toolbox(Toolbox::new().feature(Feature::new().save_as_image(SaveAsImage::new())))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value).name("Sightings"))
        .series(
            Line::new()
                .name("Penguins")
                .data(vec![8, 12, 6, 9, 15, 16, 18]),
        )
        .series(
            Line::new()
                .name("Giraffes")
                .data(vec![5, 4, 6, 7, 8, 10, 9]),
        )
        .series(Line::new().name("Lions").data(vec![3, 2, 4, 3, 5, 6, 4]))
        .series(
            Line::new()
                .name("Elephants")
                .data(vec![4, 5, 3, 6, 7, 8, 6]),
        )
        .series(
            Line::new()
                .name("Monkeys")
                .data(vec![12, 15, 10, 14, 16, 18, 20]),
        )
        .series(
            Line::new()
                .name("Total")
                .data(vec![32, 38, 29, 39, 51, 58, 57]),
        )
}
