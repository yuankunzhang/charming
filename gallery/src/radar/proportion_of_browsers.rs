use charming::{
    component::{Legend, LegendType, RadarCoordinate, Title, VisualMap},
    datatype::{DataPoint, DataPointItem},
    element::{AreaStyle, Emphasis, LineStyle, Symbol, Tooltip, Trigger},
    series::Radar,
    Chart,
};

pub fn chart() -> Chart {
    let mut series = vec![];
    for i in 1..=28 {
        series.push(
            Radar::new()
                .symbol(Symbol::None)
                .line_style(LineStyle::new().width(1))
                .emphasis(Emphasis::new().area_style(AreaStyle::new().color("rgba(0,250,0,0.3)")))
                .data(vec![DataPoint::Item(
                    DataPointItem::new(vec![
                        (40. - i as f64) * 10.,
                        (38. - i as f64) * 4. + 60.,
                        i as f64 * 5. + 10.,
                        i as f64 * 9.,
                        (i as f64 * i as f64) / 2.,
                    ])
                    .name((i + 2000).to_string()),
                )]),
        );
    }

    let mut chart = Chart::new()
        .title(
            Title::new()
                .text("Proportion of Browsers")
                .subtext("Fake Data")
                .top("10")
                .left("10"),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Item))
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
        .radar(RadarCoordinate::new().indicator(vec![
            ("IE8-", 0, 400),
            ("IE9+", 0, 400),
            ("Safari", 0, 400),
            ("Firefox", 0, 400),
            ("Chrome", 0, 400),
        ]));

    for series in series {
        chart = chart.series(series);
    }

    chart
}
