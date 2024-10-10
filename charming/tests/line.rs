use assert_json_diff::assert_json_eq;
use charming::{component::Axis, element::AxisType, series::Line, Chart};
use serde_json::json;

#[test]
fn basic_line_chart() {
    // https://echarts.apache.org/examples/en/editor.html?c=line-simple
    let echarts_json = json!(
      {
        "xAxis": {
          "type": "category",
          "data": ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        },
        "yAxis": {
          "type": "value"
        },
        "series": [
          {
            "data": [150, 230, 224, 218, 135, 147, 260],
            "type": "line"
          }
        ]
      }
    );

    let chart = Chart::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

    let charming_json = serde_json::from_str::<serde_json::Value>(&chart.to_string()).unwrap();
    println!("{}", serde_json::to_string_pretty(&echarts_json).unwrap());
    println!("{}", serde_json::to_string_pretty(&charming_json).unwrap());
    assert_json_eq!(echarts_json, charming_json);
}
