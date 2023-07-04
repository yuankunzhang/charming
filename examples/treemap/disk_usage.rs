use echarts::{
    component::{title, tooltip},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .title(title::Title::new().text("Disk Usage").left("center"))
        .tooltip(tooltip::Tooltip::new().formatter("{b}: {c}"));

    println!("{}", chart.to_string());
}
