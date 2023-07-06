use echarts::{
    component::{Axis, Dataset, DatasetSource, DatasetTransform, Title, Tooltip},
    datatype::Value,
    element::{AxisType, TooltipTrigger},
    series::{line, Series},
    Chart,
};

pub fn chart() -> Chart {
    let data: Vec<Vec<Value>> =
        serde_json::from_str(include_str!("life-expectancy-table.json")).unwrap();
    let dataset = Dataset::new()
        .source(DatasetSource::from(data).id("dataset_raw"))
        .transform(
            DatasetTransform::new()
                .id("dataset_since_1950_of_germany")
                .from_dataset_id("dataset_raw")
                .transform(
                    r#"{
                        "type": "filter",
                        "config": {
                            "and": [
                                {"dimension": "Year", "gte": 1950},
                                {"dimension": "Country", "=": "Germany"}
                            ]
                        }
                    }"#,
                ),
        )
        .transform(
            DatasetTransform::new()
                .id("dataset_since_1950_of_france")
                .from_dataset_id("dataset_raw")
                .transform(
                    r#"{
                        "type": "filter",
                        "config": {
                            "and": [
                                {"dimension": "Year", "gte": 1950},
                                {"dimension": "Country", "=": "France"}
                            ]
                        }
                    }"#,
                ),
        );

    Chart::new()
        .title(Title::new().text("Income of Germany and France since 1950"))
        .tooltip(Tooltip::new().trigger(TooltipTrigger::Axis))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .name_location("middle"),
        )
        .y_axis(Axis::new().name("Income"))
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
        .dataset(dataset)
}
