use charming::{component::Axis, df, series::Candlestick, Chart};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(Axis::new().data(vec!["2017-10-24", "2017-10-25", "2017-10-26", "2017-10-27"]))
        .y_axis(Axis::new())
        .series(Candlestick::new().data(df![
            [20, 34, 10, 38],
            [40, 35, 30, 50],
            [31, 38, 33, 44],
            [38, 15, 5, 42]
        ]))
}
