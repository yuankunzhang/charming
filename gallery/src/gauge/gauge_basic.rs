use charming::{
    element::Tooltip,
    series::{Gauge, GaugeDetail},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .tooltip(Tooltip::new().formatter("{a} <br/>{b} : {c}%"))
        .series(
            Gauge::new()
                .name("Pressure")
                .detail(GaugeDetail::new().formatter("{value}"))
                .data(vec![(50, "Score")]),
        )
}
