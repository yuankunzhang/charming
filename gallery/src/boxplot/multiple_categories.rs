use charming::{
    component::{Axis, DataZoom, DataZoomType, Grid, Legend, Title},
    datatype::{Dataset, Transform},
    element::{AxisPointer, AxisPointerType, AxisType, SplitArea, SplitLine, Tooltip, Trigger},
    series::Boxplot,
    Chart,
};
use rand::Rng;

pub fn chart() -> Chart {
    let data0 = make_data();
    let data1 = make_data();
    let data2 = make_data();
    Chart::new()
        .title(Title::new().text("Multiple Categories").left("center"))
        .dataset(
            Dataset::new()
                .source(data0)
                .source(data1)
                .source(data2)
                .transform(
                    Transform::new()
                        .from_dataset_index(0)
                        .transform(r#"{"type": "boxplot"}"#),
                )
                .transform(
                    Transform::new()
                        .from_dataset_index(1)
                        .transform(r#"{"type": "boxplot"}"#),
                )
                .transform(
                    Transform::new()
                        .from_dataset_index(2)
                        .transform(r#"{"type": "boxplot"}"#),
                ),
        )
        .legend(Legend::new().top("10%"))
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        )
        .grid(
            Grid::new()
                .left("10%")
                .top("20%")
                .right("10%")
                .bottom("15%"),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(true)
                .name_gap(30)
                .split_area(SplitArea::new().show(true))
                .split_line(SplitLine::new().show(false)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("Value")
                .min(-400)
                .max(600)
                .split_area(SplitArea::new().show(false)),
        )
        .data_zoom(DataZoom::new().type_(DataZoomType::Inside).start(0).end(20))
        .data_zoom(
            DataZoom::new()
                .type_(DataZoomType::Slider)
                .start(0)
                .end(20)
                .top("90%")
                .x_axis_index(0),
        )
        .series(Boxplot::new().name("category0").dataset_index(3))
        .series(Boxplot::new().name("category1").dataset_index(4))
        .series(Boxplot::new().name("category2").dataset_index(5))
}

fn make_data() -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut data = vec![];
    for _ in 0..18 {
        let mut data0 = vec![];
        for _ in 0..100 {
            data0.push(rng.gen::<f64>() * 200.0);
        }
        data.push(data0);
    }
    data
}
