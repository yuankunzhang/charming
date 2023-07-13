use charming::{element::Tooltip, Chart};

pub fn chart() -> Chart {
    let svg = include_str!("../../asset/veins-medical-diagram.svg");
    Chart::new()
        .geo_map(("organ_diagram", svg))
        .tooltip(Tooltip::new())
}
