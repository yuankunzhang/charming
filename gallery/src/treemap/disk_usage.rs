use charming::{Chart, component::Title, element::Tooltip};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Disk Usage").left("center"))
        .tooltip(Tooltip::new().formatter("{b}: {c}"))
}
