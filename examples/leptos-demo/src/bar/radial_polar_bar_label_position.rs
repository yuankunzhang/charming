use charming::{
    component::{AngleAxis, PolarCoordinate, RadiusAxis, Title},
    datatype::CompositeValue,
    element::{AxisType, CoordinateSystem, Label, LabelPosition, Tooltip},
    series::Bar,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Radial Polar Bar Label Position (middle)"))
        .polar(
            PolarCoordinate::new()
                .radius(vec![CompositeValue::from(30), CompositeValue::from("80%")]),
        )
        .radius_axis(RadiusAxis::new().max(4))
        .angle_axis(
            AngleAxis::new()
                .type_(AxisType::Category)
                .data(vec!['a', 'b', 'c', 'd'])
                .start_angle(75),
        )
        .tooltip(Tooltip::new())
        .series(
            Bar::new()
                .coordinate_system(CoordinateSystem::Polar)
                .label(
                    Label::new()
                        .show(true)
                        .position(LabelPosition::Middle)
                        .formatter("{b}: {c}"),
                )
                .data(vec![2., 1.2, 2.4, 3.6]),
        )
}
