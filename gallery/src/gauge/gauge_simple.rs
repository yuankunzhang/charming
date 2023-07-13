use charming::{
    element::Tooltip,
    series::{Gauge, GaugeDetail, GaugeProgress},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .tooltip(Tooltip::new().formatter("{a} <br/>{b} : {c}%"))
        .series(
            Gauge::new()
                .name("Pressure")
                .progress(GaugeProgress::new().show(true))
                .detail(
                    GaugeDetail::new()
                        .formatter("{value}")
                        .value_animation(true),
                )
                .data(vec![(50, "Score")]),
        )
}
