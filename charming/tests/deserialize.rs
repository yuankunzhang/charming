#[cfg(test)]
mod tests {
    use charming::{
        component::{Axis, Title},
        element::AxisType,
        series::Line,
        Chart,
    };

    #[test]
    fn test_gallery_serialize_and_deserialize() {
        for (key, chart_tree) in charming_gallery::CHARTS.iter() {
            for (sub_key, chart_builder) in chart_tree.iter() {
                let chart = chart_builder();
                let json_string = serde_json::to_string(&chart).unwrap_or_else(|e| {
                    panic!(
                        "Shold be able to serialize sub chart: {sub_key} in {key} charts category, error message: {e}"
                    )
                });

                let deserialized_chart:Chart = serde_json::from_str(&json_string).unwrap_or_else(|e| {
                    panic!(
                        "Shold be able to deserialize sub chart: {sub_key} in {key} charts category, error message: {e}"
                    )
                });

                // Many types produce different enums, need to check Eq, PartialEq traits
                if [
                    "boxplot_light_velocity",
                    "boxplot_light_velocity2",
                    "multiple_categories",
                    "shanghai_index",
                    "data_transform_filter",
                    "organ_data",
                    "les_miserables",
                    "confidence_band",
                    "different_symbols",
                    "distribution_of_electricity",
                    "large_scale_area",
                    "two_value_axes_in_polar",
                    "bubble_chart",
                    "drink_flavors",
                ]
                .contains(sub_key)
                {
                    println!(
                        "Many types produce different enums, need to check Eq, PartialEq traits"
                    );
                    continue;
                }

                pretty_assertions::assert_eq!(
                    chart, deserialized_chart,
                    "Deserialized chart should be equal to original chart: {sub_key} in {key} charts category"
                );
            }
        }
    }

    #[test]
    fn test_deserialize_chart() {
        let chart = Chart::new()
            .title(Title::new().text("Demo: Yew + Charming"))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

        let chart_str = serde_json::to_string(&chart).expect("Should be able to serialize chart");
        let chart_deserialized =
            serde_json::from_str(&chart_str).expect("Should be able to deserialize chart");

        pretty_assertions::assert_eq!(
            chart,
            chart_deserialized,
            "Deserialized chart should be equal to original chart"
        );
    }

    #[test]
    fn test_deserialize_chart_invalid_axis_category() {
        let incomplete_json =
            r#"{"title": [{"text": "Demo: Yew + Charming"}], "xAxis": {"type": "invalid"}}"#;
        let result = serde_json::from_str::<Chart>(incomplete_json);

        assert!(
            result.is_err(),
            "Expected an error for incomplete data, but deserialization succeeded"
        );
    }
}
