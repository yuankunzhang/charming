use charming::{
    component::{Axis, Title},
    datatype::{CompositeValue, Dataset, Source, Transform},
    element::{AxisType, DimensionEncode, NameLocation, Tooltip, Trigger},
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    let data: Vec<Vec<CompositeValue>> =
        serde_json::from_str(include_str!("../../asset/life-expectancy-table.json")).unwrap();
    let dataset = Dataset::new()
        .source(Source::new(data.into()).id("dataset_raw"))
        .transform(
            Transform::new()
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
            Transform::new()
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
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .name_location(NameLocation::Middle),
        )
        .y_axis(Axis::new().name("Income"))
        .series(
            Line::new()
                .dataset_id("dataset_since_1950_of_germany")
                .show_symbol(false)
                .encode(
                    DimensionEncode::new()
                        .x("Year")
                        .y("Income")
                        .item_name("Year")
                        .tooltip(vec!["Income"]),
                ),
        )
        .series(
            Line::new()
                .dataset_id("dataset_since_1950_of_france")
                .show_symbol(false)
                .encode(
                    DimensionEncode::new()
                        .x("Year")
                        .y("Income")
                        .item_name("Year")
                        .tooltip(vec!["Income"]),
                ),
        )
        .dataset(dataset)
}
