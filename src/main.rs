// main.rs
mod chart;
mod dataset;
mod renderer;
mod style;

use chart::option::*;
use chart::series::*;

fn main() {
    let s = renderer::render_string().unwrap();
    println!("{}", s);

    let opt = ChartOption::new()
        .title(Title::new().text("My Line Chart"))
        .legend(Legend::new().data(vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
            "Line 4".to_string(),
            "Line 5".to_string(),
        ]))
        .x_axis(Axis::new().type_(AxisType::Category).data(vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ]))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Series::Line(Line::new().data(vec![
            vec![0.0, 140.0],
            vec![1.0, 232.0],
            vec![2.0, 101.0],
            vec![3.0, 264.0],
            vec![4.0, 90.0],
            vec![5.0, 340.0],
            vec![6.0, 250.0],
        ])));
    println!("{}", serde_json::to_string_pretty(&opt).unwrap());
}
