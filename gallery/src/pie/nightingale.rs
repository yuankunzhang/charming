use charming::{
    component::{DataView, Feature, Legend, Restore, SaveAsImage, Toolbox},
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .legend(Legend::new().top("bottom"))
        .toolbox(
            Toolbox::new().show(true).feature(
                Feature::new()
                    .data_view(DataView::new().show(true))
                    .restore(Restore::new().show(true))
                    .save_as_image(SaveAsImage::new().show(true)),
            ),
        )
        .series(
            Pie::new()
                .name("Nightingale Chart")
                .rose_type(PieRoseType::Radius)
                .radius(vec!["50", "250"])
                .center(vec!["50%", "50%"])
                .item_style(ItemStyle::new().border_radius(8))
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
        )
}
