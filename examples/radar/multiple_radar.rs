use echarts::chart::{
    legend::Legend,
    radar::*,
    title::Title,
    tooltip::{Tooltip, TooltipTrigger},
    Chart,
};
use echarts::series;

fn main() {
    let chart = Chart::new()
        .title(Title::new().text("Multiple Radar"))
        .tooltip(Tooltip::new().trigger(TooltipTrigger::Axis))
        .legend(Legend::new().left("center").data(vec![
            "A Software",
            "A Phone",
            "Another Phone",
            "Precipitation",
            "Evaporation",
        ]))
        .radars(vec![
            Radar::new()
                .indicator(vec![
                    RadarIndicator::new().name("Brand").max(100),
                    RadarIndicator::new().name("Content").max(100),
                    RadarIndicator::new().name("Usability").max(100),
                    RadarIndicator::new().name("Function").max(100),
                ])
                .center(("25%", "40%"))
                .radius(80.0),
            Radar::new()
                .indicator(vec![
                    RadarIndicator::new().name("Look").max(100),
                    RadarIndicator::new().name("Photo").max(100),
                    RadarIndicator::new().name("System").max(100),
                    RadarIndicator::new().name("Performance").max(100),
                    RadarIndicator::new().name("Screen").max(100),
                ])
                .center(("50%", "60%"))
                .radius(80.0),
            Radar::new()
                .indicator(vec![
                    RadarIndicator::new().name("1月").max(100),
                    RadarIndicator::new().name("2月").max(100),
                    RadarIndicator::new().name("3月").max(100),
                    RadarIndicator::new().name("4月").max(100),
                    RadarIndicator::new().name("5月").max(100),
                    RadarIndicator::new().name("6月").max(100),
                    RadarIndicator::new().name("7月").max(100),
                    RadarIndicator::new().name("8月").max(100),
                    RadarIndicator::new().name("9月").max(100),
                    RadarIndicator::new().name("10月").max(100),
                    RadarIndicator::new().name("11月").max(100),
                    RadarIndicator::new().name("12月").max(100),
                ])
                .center(("75%", "40%"))
                .radius(80.0),
        ])
        .series(series::Series::Radar(
            series::radar::Radar::new()
                .name("Radar")
                .data(vec![series::radar::RadarData {
                    name: "A Software".into(),
                    value: vec![60.0, 73.0, 85.0, 40.0],
                }]),
        ))
        .series(series::Series::Radar(
            series::radar::Radar::new().name("A Phone").data(vec![
                series::radar::RadarData {
                    name: "A Phone".into(),
                    value: vec![85.0, 90.0, 90.0, 95.0, 95.0],
                },
                series::radar::RadarData {
                    name: "Another Phone".into(),
                    value: vec![95.0, 80.0, 95.0, 90.0, 93.0],
                },
            ]),
        ));
    println!("{}", chart.to_string());
}
