use echarts::{
    component::{AngleAxis, Polar, RadiusAxis, Title, Tooltip},
    element::{AxisType, CoordinateSystem, Label, Position},
    series::Bar,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Tangential Polar Bar Label Position (middle)"))
        .polar(Polar::new().radius(("30", "80%")))
        .angle_axis(AngleAxis::new().start_angle(75).max(4))
        .radius_axis(
            RadiusAxis::new()
                .type_(AxisType::Category)
                .data(vec!["a", "b", "c", "d"]),
        )
        .tooltip(Tooltip::new())
        .series(
            Bar::new()
                .coordinate_system(CoordinateSystem::Polar)
                .label(
                    Label::new()
                        .show(true)
                        .position(Position::Middle)
                        .formatter("{b}: {c}"),
                )
                .data(vec![2.0, 1.2, 2.4, 3.6]),
        )
}
