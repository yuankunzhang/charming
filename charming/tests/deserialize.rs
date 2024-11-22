#[cfg(test)]
mod tests {
    use charming::{
        component::{Axis, Title},
        element::AxisType,
        series::Line,
        Chart,
    };

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

        assert_eq!(
            chart, chart_deserialized,
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
