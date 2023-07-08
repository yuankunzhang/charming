use echarts::{
    component::{Legend, LegendType, RadarCoordinate, Title, Tooltip, VisualMap},
    element::TooltipTrigger,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(
            Title::new()
                .text("Proportion of Browsers")
                .subtext("Fake Data")
                .top("10")
                .left("10"),
        )
        .tooltip(Tooltip::new().trigger(TooltipTrigger::Item))
        .legend(
            Legend::new().type_(LegendType::Scroll).bottom("10").data(
                (1..=28)
                    .into_iter()
                    .map(|i| (i + 2000).to_string())
                    .collect::<Vec<_>>(),
            ),
        )
        .visual_map(
            VisualMap::new()
                .top("middle")
                .right("10")
                .color(vec!["red", "yellow"]),
        )
        .radar(vec![RadarCoordinate::new().indicator(vec![
            ("IE8-", 0, 400),
            ("IE9+", 0, 400),
            ("Safari", 0, 400),
            ("Firefox", 0, 400),
            ("Chrome", 0, 400),
        ])])
}
