use charming::{
    component::{Aria, Axis, Decal, Feature, SaveAsImage, SaveAsImageType, Toolbox},
    element::AxisType,
    series::Bar,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .toolbox(
            Toolbox::new().feature(
                Feature::new().save_as_image(SaveAsImage::new().type_(SaveAsImageType::Svg)),
            ),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .aria(Aria::new().enabled(true).decal(Decal::new().show(true)))
        .series(Bar::new().data(vec![120, 200, 150, 80, 70, 110, 130]))
        .series(Bar::new().data(vec![20, 40, 90, 40, 30, 70, 120]))
        .series(Bar::new().data(vec![140, 230, 120, 50, 30, 150, 120]))
}
