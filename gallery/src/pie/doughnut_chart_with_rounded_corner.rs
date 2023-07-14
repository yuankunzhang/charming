use charming::{
    component::Legend,
    element::{Emphasis, Label, LabelLine, LabelPosition, Tooltip, Trigger},
    series::Pie,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .tooltip(Tooltip::new().trigger(Trigger::Item))
        .legend(Legend::new().top("5%").left("center"))
        .series(
            Pie::new()
                .name("Access From")
                .radius(vec!["40%", "55%"])
                .avoid_label_overlap(false)
                .label(Label::new().show(false).position(LabelPosition::Center))
                .emphasis(
                    Emphasis::new()
                        .label(Label::new().show(true).font_size(40).font_weight("bold")),
                )
                .label_line(LabelLine::new().show(false))
                .data(vec![
                    (1048, "Search Engine"),
                    (735, "Direct Access"),
                    (580, "Email Marketing"),
                    (484, "Union Ads"),
                    (300, "Video Ads"),
                ]),
        )
}
