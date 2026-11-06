#[cfg(test)]
mod tests {
    use charming::{
        Chart,
        component::{Axis, Title},
        element::AxisType,
        series::Line,
    };

    #[test]
    fn test_gallery_serialize_and_deserialize() {
        for (key, chart_tree) in charming_gallery::CHARTS.iter() {
            for (sub_key, chart_builder) in chart_tree.iter() {
                let chart = chart_builder();
                let json_string = serde_json::to_string(&chart).unwrap_or_else(|e| {
                    panic!(
                        "Should be able to serialize sub chart: {sub_key} in {key} charts category, error message: {e}"
                    )
                });

                let deserialized_chart:Chart = serde_json::from_str(&json_string).unwrap_or_else(|e| {
                    panic!(
                        "Should be able to deserialize sub chart: {sub_key} in {key} charts category, error message: {e}"
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
                    chart,
                    deserialized_chart,
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

    #[test]
    fn test_deserialize_chart_more_flexibility() {
        let chart_str = r##"{
          "animationDurationUpdate": 800,
          "animationEasingUpdate": "quinticInOut",
          "grid": {
            "bottom": "50",
            "containsLabel": true,
            "left": "70",
            "right": "30",
            "show": false,
            "top": "40"
          },
          "legend": {
            "bottom": 10,
            "data": [
              "Type1",
              "Type2",
              "Type3",
              "Type4",
              "Type5",
              "Type6"
            ],
            "itemGap": 15,
            "itemHeight": 14,
            "itemWidth": 14,
            "orient": "horizontal",
            "show": true,
            "textStyle": {
              "fontSize": 12
            }
          },
          "series": [
            {
              "animation": true,
              "animationDuration": 1000,
              "animationEasing": "bounceOut",
              "data": [
                {
                  "name": "Type1",
                  "value": 3750
                },
                {
                  "name": "Type2",
                  "value": 1100
                },
                {
                  "name": "Type3",
                  "value": 3000
                },
                {
                  "name": "Type4",
                  "value": 500
                },
                {
                  "name": "Type5",
                  "value": 12000
                },
                {
                  "name": "Type6",
                  "value": 12000
                }
              ],
              "emphasis": {
                "itemStyle": {
                  "shadowBlur": 10,
                  "shadowColor": "rgba(0, 0, 0, 0.5)",
                  "shadowOffsetX": 0
                }
              },
              "name": "",
              "radius": "50%",
              "type": "pie"
            }
          ],
          "textStyle": {
            "color": "#000A26",
            "fontFamily": "Inter",
            "fontSize": 12
          },
          "title": {
            "left": 5,
            "show": false,
            "text": "Demo pie chart",
            "textStyle": {
              "color": "#000A26",
              "fontSize": 18,
              "fontWeight": 600
            }
          },
          "tooltip": {
            "axisPointer": {
              "type": "shadow"
            },
            "show": true,
            "trigger": "item"
          },
          "xAxis": {
            "axisPointer": {
              "show": false
            },
            "data": [],
            "name": "",
            "nameGap": 30,
            "nameLocation": "middle",
            "show": false,
            "type": "category"
          },
          "yAxis": {
            "axisPointer": {
              "show": false
            },
            "data": null,
            "name": "",
            "nameGap": 70,
            "nameLocation": "middle",
            "show": false,
            "type": "value"
          }
        }"##;

        serde_json::from_str::<Chart>(chart_str).expect("Should be able to deserialize chart");
    }
}
