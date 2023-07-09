# echarts-rs

I need a chart! 📊

Echarts-rs is a powerful and versatile chart rendering library that leverages the power of [Apache ECharts](https://echarts.apache.org/en/index.html) to deliver high-quality data visualizations. Built with the Rust programming language, this library aims to provide Rust users with an intuitive and effective way to generate and manipulate charts, using a declarative and user-friendly API. It supports to render charts into a wide range of formats, including HTML, SVG, PNG, and JPEG.

It is easy to use, this code draws a rose pie chart:

```rs
use echarts::{
    component::Legend,
    df,
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart, ImageRenderer
};

fn main() {
    let chart = Chart::new()
        .legend(Legend::new().top("bottom"))
        .series(
            Pie::new()
                .name("Nightingale Chart")
                .rose_type(PieRoseType::Radius)
                .radius(("50", "250"))
                .center(("50%", "50%"))
                .item_style(ItemStyle::new().border_radius(8))
                .data(df![
                    (40.0, "rose 1"),
                    (38.0, "rose 2"),
                    (32.0, "rose 3"),
                    (30.0, "rose 4"),
                    (28.0, "rose 5"),
                    (26.0, "rose 6"),
                    (22.0, "rose 7"),
                    (18.0, "rose 8"),
                ]),
        )
    let mut renderer = ImageRenderer::new(1000, 800);
    renderer.save(&chart, "/tmp/nightingale.svg");
}
```

![](img/nightingale.svg)

It is powerful, the code file [gallery/src/dataset/encode_and_matrix.rs](./gallery/src/dataset/encode_and_matrix.rs) draws a complex chart with four sub-charts:

![](img/encode-and-matrix.svg)

Please see more examples in the [gallery](./gallery/) module.