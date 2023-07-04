use echarts::{
    component::{legend, parallel_axis, tooltip, visual_map},
    element::text_style,
    Chart,
};

fn main() {
    let chart = Chart::new()
        .background_color("#333")
        .legend(
            legend::Legend::new()
                .bottom("30")
                .data(vec!["Beijing", "Shanghai", "Guangzhou"])
                .item_gap(20)
                .text_style(text_style::TextStyle::new().color("#fff").font_size(14)),
        )
        .tooltip(
            tooltip::Tooltip::new()
                .padding(10)
                .background_color("#222")
                .border_color("#777")
                .border_width(1),
        )
        .parallel_axis(
            parallel_axis::ParallelAxis::new()
                .dim(0)
                .name("日期")
                .inverse(true)
                .max(31)
                .name_location("start"),
        )
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(1).name("AQI"))
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(2).name("PM2.5"))
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(3).name("PM10"))
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(4).name("CO"))
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(5).name("NO2"))
        .parallel_axis(parallel_axis::ParallelAxis::new().dim(6).name("SO2"))
        .parallel_axis(
            parallel_axis::ParallelAxis::new()
                .dim(7)
                .name("等级")
                .type_(parallel_axis::ParallelAxisType::Category)
                .data(vec![
                    "优",
                    "良",
                    "轻度污染",
                    "中度污染",
                    "重度污染",
                    "严重污染",
                ]),
        )
        .visual_map(
            visual_map::VisualMap::new()
                .show(true)
                .min(0)
                .max(150)
                .dimension(2)
                .in_range(visual_map::InRange::new().color(vec!["#50a3ba", "#eac736", "#d94e5d"])),
        );
}
