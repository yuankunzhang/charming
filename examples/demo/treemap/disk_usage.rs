use echarts::{
    component::{title, tooltip},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(title::Title::new().text("Disk Usage").left("center"))
        .tooltip(tooltip::Tooltip::new().formatter("{b}: {c}"))
}
