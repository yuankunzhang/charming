use echarts::{
    basic::item_style,
    component::{legend, toolbox},
    series::{pie, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
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
                    pie::DataPoint {
                        value: 40.0,
                        name: Some("rose 1".to_string()),
                    },
                    pie::DataPoint {
                        value: 38.0,
                        name: Some("rose 2".to_string()),
                    },
                    pie::DataPoint {
                        value: 32.0,
                        name: Some("rose 3".to_string()),
                    },
                    pie::DataPoint {
                        value: 30.0,
                        name: Some("rose 4".to_string()),
                    },
                    pie::DataPoint {
                        value: 28.0,
                        name: Some("rose 5".to_string()),
                    },
                    pie::DataPoint {
                        value: 26.0,
                        name: Some("rose 6".to_string()),
                    },
                    pie::DataPoint {
                        value: 22.0,
                        name: Some("rose 7".to_string()),
                    },
                    pie::DataPoint {
                        value: 18.0,
                        name: Some("rose 8".to_string()),
                    },
                ]),
        ));
    println!("{}", chart.to_string());
}
