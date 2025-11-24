use charming::{Chart, element::Tooltip};

pub fn chart() -> Chart {
    let svg = include_str!("../../asset/veins-medical-diagram.svg");
    Chart::new()
        .geo_map(("organ_diagram", svg))
        .tooltip(Tooltip::new())
}
