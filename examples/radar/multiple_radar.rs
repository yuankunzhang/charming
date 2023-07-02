use echarts::basic::area_style;
use echarts::component::legend;
use echarts::component::radar_coordinate;
use echarts::component::title;
use echarts::component::tooltip;
use echarts::series;
use echarts::Chart;

fn main() {
    let chart = Chart::new()
        .title(title::Title::new().text("Multiple Radar"))
        .tooltip(tooltip::Tooltip::new().trigger(tooltip::Trigger::Axis))
        .legend(legend::Legend::new().left("center").data(vec![
            "A Software",
            "A Phone",
            "Another Phone",
            "Precipitation",
            "Evaporation",
        ]))
        .radars(vec![
            radar_coordinate::RadarCoordinate::new()
                .indicator(vec![
                    ("Brand", 0, 100),
                    ("Content", 0, 100),
                    ("Usability", 0, 100),
                    ("Function", 0, 100),
                ])
                .center(("25%", "40%"))
                .radius(80.0),
            radar_coordinate::RadarCoordinate::new()
                .indicator(vec![
                    ("Look", 0, 100),
                    ("Photo", 0, 100),
                    ("System", 0, 100),
                    ("Performance", 0, 100),
                    ("Screen", 0, 100),
                ])
                .center(("50%", "60%"))
                .radius(80.0),
            radar_coordinate::RadarCoordinate::new()
                .indicator(vec![
                    ("1月", 0, 100),
                    ("2月", 0, 100),
                    ("3月", 0, 100),
                    ("4月", 0, 100),
                    ("5月", 0, 100),
                    ("6月", 0, 100),
                    ("7月", 0, 100),
                    ("8月", 0, 100),
                    ("9月", 0, 100),
                    ("10月", 0, 100),
                    ("11月", 0, 100),
                    ("12月", 0, 100),
                ])
                .center(("75%", "40%"))
                .radius(80.0),
        ])
        .series(series::Series::Radar(
            series::radar::Radar::new()
                .name("Radar")
                .tooltip(tooltip::Tooltip::new().trigger(tooltip::Trigger::Item))
                .area_style(area_style::AreaStyle::new())
                .data(vec![("A Software", vec![60, 73, 85, 40])]),
        ))
        .series(series::Series::Radar(
            series::radar::Radar::new()
                .radar_index(1)
                .area_style(area_style::AreaStyle::new())
                .data(vec![
                    ("A Phone", vec![85, 90, 90, 95, 95]),
                    ("Another Phone", vec![95, 80, 95, 90, 93]),
                ]),
        ))
        .series(series::Series::Radar(
            series::radar::Radar::new()
                .radar_index(2)
                .area_style(area_style::AreaStyle::new())
                .data(vec![
                    (
                        "Precipitation",
                        vec![
                            2.6, 5.9, 9.0, 26.4, 28.7, 70.7, 75.6, 82.2, 48.7, 18.8, 6.0, 2.3,
                        ],
                    ),
                    (
                        "Evaporation",
                        vec![
                            2.0, 4.9, 7.0, 23.2, 25.6, 76.7, 35.6, 62.2, 32.6, 20.0, 6.4, 3.3,
                        ],
                    ),
                ]),
        ));
    println!("{}", chart.to_string());
}
