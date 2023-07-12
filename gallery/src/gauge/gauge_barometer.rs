use charming::{
    df,
    element::{Anchor, AxisLabel, AxisLine, AxisTick, ItemStyle, LineStyle, Pointer, SplitLine},
    series::{Gauge, GaugeDetail, GaugeTitle},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .series(
            Gauge::new()
                .min(0)
                .max(100)
                .split_number(10)
                .radius("80%")
                .axis_line(AxisLine::new().line_style((1.0, "#f00", 3.0)))
                .axis_tick(
                    AxisTick::new()
                        .distance(-12)
                        .length(10)
                        .line_style(LineStyle::new().color("#f00")),
                )
                .axis_label(AxisLabel::new().distance(-50).font_size(25).color("#f00"))
                .split_line(
                    SplitLine::new()
                        .distance(-18)
                        .length(18)
                        .line_style(LineStyle::new().color("#f00")),
                )
                .anchor(
                    Anchor::new()
                        .show(true)
                        .size(20)
                        .item_style(ItemStyle::new().border_color("#000").border_width(2)),
                )
                .pointer(
                    Pointer::new()
                        .length("115%")
                        .item_style(ItemStyle::new().color("#000"))
                        .offset_center(("0", "10%"))
                        .icon(ICON),
                )
                .title(GaugeTitle::new().offset_center(("0", "-50%")))
                .detail(GaugeDetail::new().value_animation(true).precision(1))
                .data(df![(58.46, "PLP")]),
        )
        .series(
            Gauge::new()
                .min(0)
                .max(60)
                .split_number(6)
                .axis_line(AxisLine::new().line_style((1.0, "#000", 3.0)))
                .axis_tick(
                    AxisTick::new()
                        .distance(0)
                        .length(10)
                        .line_style(LineStyle::new().color("#000")),
                )
                .axis_label(AxisLabel::new().distance(10).font_size(25).color("#000"))
                .split_line(
                    SplitLine::new()
                        .distance(-3)
                        .length(18)
                        .line_style(LineStyle::new().color("#000")),
                )
                .anchor(
                    Anchor::new()
                        .show(true)
                        .size(14)
                        .item_style(ItemStyle::new().border_color("#000").border_width(2)),
                )
                .pointer(Pointer::new().show(false))
                .title(GaugeTitle::new().show(false)),
        )
}

static ICON: &str = "path://M2090.36389,615.30999 L2090.36389,615.30999 C2091.48372,615.30999 2092.40383,616.194028 2092.44859,617.312956 L2096.90698,728.755929 C2097.05155,732.369577 2094.2393,735.416212 2090.62566,735.56078 C2090.53845,735.564269 2090.45117,735.566014 2090.36389,735.566014 L2090.36389,735.566014 C2086.74736,735.566014 2083.81557,732.63423 2083.81557,729.017692 C2083.81557,728.930412 2083.81732,728.84314 2083.82081,728.755929 L2088.2792,617.312956 C2088.32396,616.194028 2089.24407,615.30999 2090.36389,615.30999 Z";
