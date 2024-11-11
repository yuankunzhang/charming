use charming::{
    component::{Axis, Feature, Grid, Legend, SaveAsImage, Title, Toolbox},
    element::{AxisType, Step, Tooltip, Trigger},
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Step Line"))
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .legend(Legend::new())
        .grid(
            Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .toolbox(Toolbox::new().feature(Feature::new().save_as_image(SaveAsImage::new())))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Line::new()
                .name("Step Start")
                .step(Step::Start)
                .data(vec![120, 132, 101, 134, 90, 230, 210]),
        )
        .series(
            Line::new()
                .name("Step Middle")
                .step(Step::Middle)
                .data(vec![220, 282, 201, 234, 290, 430, 410]),
        )
        .series(
            Line::new()
                .name("Step End")
                .step(Step::End)
                .data(vec![450, 432, 401, 454, 590, 530, 510]),
        )
}
