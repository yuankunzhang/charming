use charming::{
    component::{Legend, Title},
    df,
    element::{Emphasis, ItemStyle, Orient, Tooltip, Trigger},
    series::Pie,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(
            Title::new()
                .text("Referer of a Website")
                .subtext("Fake Data")
                .left("center"),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Item))
        .legend(Legend::new().orient(Orient::Vertical).left("left"))
        .series(
            Pie::new()
                .name("Access From")
                .radius("50%")
                .data(df!(
                    (1048, "Search Engine"),
                    (735, "Direct"),
                    (580, "Email"),
                    (484, "Union Ads"),
                    (300, "Video Ads")
                ))
                .emphasis(
                    Emphasis::new().item_style(
                        ItemStyle::new()
                            .shadow_blur(10)
                            .shadow_offset_x(0)
                            .shadow_color("rgba(0, 0, 0, 0.5)"),
                    ),
                ),
        )
}
