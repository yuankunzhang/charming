use echarts::{
    component::Axis,
    series::{candlestick, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(Axis::new().data(vec!["2017-10-24", "2017-10-25", "2017-10-26", "2017-10-27"]))
        .y_axis(Axis::new())
        .series(Series::Candlestick(candlestick::Candlestick::new().data(
            vec![
                vec![20, 34, 10, 38],
                vec![40, 35, 30, 50],
                vec![31, 38, 33, 44],
                vec![38, 15, 5, 42],
            ],
        )))
}
