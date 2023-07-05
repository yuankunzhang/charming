use echarts::{
    component::{legend, toolbox},
    element::item_style,
    series::{pie, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .legend(legend::Legend::new().top("bottom"))
        .toolbox(
            toolbox::Toolbox::new().show(true).feature(
                toolbox::Feature::new()
                    .data_view(toolbox::DataView::new().show(true))
                    .restore(toolbox::Restore::new().show(true))
                    .save_as_image(toolbox::SaveAsImage::new().show(true)),
            ),
        )
        .series(Series::Pie(
            pie::Pie::new()
                .name("Nightingale Chart")
                .rose_type(pie::RoseType::Radius)
                .radius(("50", "250"))
                .center(("50%", "50%"))
                .item_style(item_style::ItemStyle::new().border_radius(8))
                .data(vec![
                    (40.0, "rose 1"),
                    (38.0, "rose 2"),
                    (32.0, "rose 3"),
                    (30.0, "rose 4"),
                    (28.0, "rose 5"),
                    (26.0, "rose 6"),
                    (22.0, "rose 7"),
                    (18.0, "rose 8"),
                ]),
        ))
}
