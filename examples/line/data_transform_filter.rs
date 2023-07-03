use echarts::{
    component::{axis, dataset, title, tooltip},
    datatype::Value,
    series::{line, Series},
    Chart,
};

fn main() {
    let data: Vec<Vec<Value>> =
        serde_json::from_str(include_str!("life-expectancy-table.json")).unwrap();
    let dataset = dataset::Dataset::new().source(data);

    let chart = Chart::new()
        .title(title::Title::new().text("Income of Germany and France since 1950"))
        .tooltip(tooltip::Tooltip::new().trigger(tooltip::Trigger::Axis))
        .x_axis(
            axis::Axis::new()
                .type_(axis::AxisType::Category)
                .name_location("middle"),
        )
        .y_axis(axis::Axis::new().name("Income"))
        .series(Series::Line(
            line::Line::new()
                .dataset_id("dataset_since_1950_of_germany")
                .show_symbol(false)
                .encode(
                    line::Encode::new()
                        .x("Year")
                        .y("Income")
                        .item_name("Year")
                        .tooltip(vec!["Income"]),
                ),
        ))
        .series(Series::Line(
            line::Line::new()
                .dataset_id("dataset_since_1950_of_france")
                .show_symbol(false)
                .encode(
                    line::Encode::new()
                        .x("Year")
                        .y("Income")
                        .item_name("Year")
                        .tooltip(vec!["Income"]),
                ),
        ))
        .dataset(dataset);

    println!("{}", chart.to_string());
}
