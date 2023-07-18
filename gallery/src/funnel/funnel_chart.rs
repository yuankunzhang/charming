use charming::{
    component::{DataView, Feature, Legend, Restore, SaveAsImage, Title, Toolbox},
    df,
    element::{
        Emphasis, ItemStyle, Label, LabelLine, LabelPosition, LineStyle, LineStyleType, Sort,
        Tooltip, Trigger,
    },
    series::Funnel,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Funnel"))
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Item)
                .formatter("{a} <br/>{b} : {c}%"),
        )
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_view(DataView::new().read_only(false))
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
        .legend(Legend::new().data(vec!["Show", "Click", "Visit", "Inquiry", "Order"]))
        .series(
            Funnel::new()
                .name("Funnel")
                .left("10%")
                .top(60)
                .bottom(60)
                .width("80%")
                .min(0)
                .max(100)
                .min_size("0%")
                .max_size("100%")
                .sort(Sort::Descending)
                .gap(2)
                .label(Label::new().show(true).position(LabelPosition::Inside))
                .label_line(
                    LabelLine::new()
                        .length(10)
                        .line_style(LineStyle::new().width(1).type_(LineStyleType::Solid)),
                )
                .item_style(ItemStyle::new().border_color("#fff").border_width(1))
                .emphasis(Emphasis::new().label(Label::new().font_size(20)))
                .data(df![
                    (60, "Visit"),
                    (30, "Inquiry"),
                    (10, "Order"),
                    (80, "Click"),
                    (100, "Show"),
                ]),
        )
}
