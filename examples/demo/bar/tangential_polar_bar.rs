use echarts::{
    component::{angle_axis, polar, radius_axis, title, tooltip},
    element::{AxisType, CoordinateSystem, Label, Position},
    series::{bar, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(title::Title::new().text("Tangential Polar Bar Label Position (middle)"))
        .polar(polar::Polar::new().radius(("30", "80%")))
        .angle_axis(angle_axis::AngleAxis::new().start_angle(75).max(4))
        .radius_axis(
            radius_axis::RadiusAxis::new()
                .type_(AxisType::Category)
                .data(vec!["a", "b", "c", "d"]),
        )
        .tooltip(tooltip::Tooltip::new())
        .series(Series::Bar(
            bar::Bar::new()
                .coordinate_system(CoordinateSystem::Polar)
                .label(
                    Label::new()
                        .show(true)
                        .position(Position::Middle)
                        .formatter("{b}: {c}"),
                )
                .data(vec![2.0, 1.2, 2.4, 3.6]),
        ))
}
