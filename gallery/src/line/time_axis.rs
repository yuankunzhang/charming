use charming::{
    component::{Axis, DataZoom, Grid, Title, Toolbox},
    element::{AxisType, Tooltip, Trigger},
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    let csv_data = include_str!("../../asset/sports-facilities-usage.csv");
    let rows: Vec<&str> = csv_data.lines().collect();
    let years: Vec<String> = rows[0]
        .split(',')
        .skip(1)
        .map(|yr| yr.to_string())
        .collect();

    let mut series_data = Vec::new();
    for row in rows.iter().skip(1) {
        let mut split = row.split(',');
        let key = split.next().unwrap().trim();
        let values: Vec<f64> = split
            .map(|val| val.trim().parse::<f64>().unwrap())
            .collect();

        let data: Vec<Vec<String>> = years
            .iter()
            .zip(values.iter())
            .map(|(year, value)| vec![year.clone(), value.to_string()])
            .collect();

        series_data.push((key.to_string(), data));
    }

    let mut chart = Chart::new()
        .title(
            Title::new()
                .text("Usage of Sports Facilities Managed by Sport Singapore (2012–2023)")
                .left("center"),
        )
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .grid(
            Grid::new()
                .left("10%")
                .right("10%")
                .bottom("15%")
                .contain_label(true),
        )
        .toolbox(Toolbox::new())
        .x_axis(
            Axis::new()
                .type_(AxisType::Time)
                .name("Year")
                .boundary_gap(false),
        )
        .y_axis(Axis::new().type_(AxisType::Value).name("Attendance").min(0))
        .data_zoom(DataZoom::new().start_value("2018").end_value("2023"));

    for (key, data) in series_data {
        chart = chart.series(Line::new().name(&key).data(data).smooth(true));
    }

    chart
}
