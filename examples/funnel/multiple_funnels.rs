use echarts::{
    component::{legend, title, toolbox, tooltip},
    series::{funnel, Series},
    utility::{label, orient::Orient, sort},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .title(
            title::Title::new()
                .text("Funnel")
                .left("left")
                .top("bottom"),
        )
        .tooltip(
            tooltip::Tooltip::new()
                .trigger(tooltip::Trigger::Item)
                .formatter("{a} <br/>{b} : {c}%"),
        )
        .toolbox(
            toolbox::Toolbox::new()
                .orient(Orient::Vertical)
                .top("center")
                .feature(
                    toolbox::Feature::new()
                        .save_as_image(toolbox::SaveAsImage::new())
                        .restore(toolbox::Restore::new())
                        .data_view(toolbox::DataView::new().read_only(true)),
                ),
        )
        .legend(
            legend::Legend::new()
                .orient(Orient::Vertical)
                .left("left")
                .data(vec!["Show", "Click", "Visit", "Inquiry", "Order"]),
        )
        .series(Series::Funnel(
            funnel::Funnel::new()
                .name("Funnel")
                .width("40%")
                .height("45%")
                .left("5%")
                .top("50%")
                .data(vec![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show"),
                ]),
        ))
        .series(Series::Funnel(
            funnel::Funnel::new()
                .name("Pyramid")
                .width("40%")
                .height("45%")
                .left("5%")
                .top("5%")
                .sort(sort::Sort::Ascending)
                .data(vec![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show"),
                ]),
        ))
        .series(Series::Funnel(
            funnel::Funnel::new()
                .name("Funnel")
                .width("40%")
                .height("45%")
                .left("55%")
                .top("5%")
                .label(label::Label::new().position(label::Position::Left))
                .data(vec![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show"),
                ]),
        ))
        .series(Series::Funnel(
            funnel::Funnel::new()
                .name("Pyramid")
                .width("40%")
                .height("45%")
                .left("55%")
                .top("50%")
                .sort(sort::Sort::Ascending)
                .label(label::Label::new().position(label::Position::Left))
                .data(vec![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show"),
                ]),
        ));

    println!("{}", chart.to_string());
}
