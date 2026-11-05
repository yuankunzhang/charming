//! Deterministic examples shared by the gallery and renderer acceptance tests.
use charming::{
    Chart,
    component::{Axis, Grid, Matrix, MatrixDimension, Title, VisualMap},
    element::{AxisType, CoordinateSystem, DimensionEncode, ItemStyle, Label},
    series::{Chord, ChordEdge, ChordNode, Custom, Heatmap},
};

pub const EXTENSIONS: &str = concat!(
    include_str!("../asset/extensions/violin.js"),
    "\necharts.use(violinCustomSeriesInstaller);\n",
    include_str!("../asset/extensions/bar-range.js"),
    "\necharts.use(barRangeCustomSeriesInstaller);\n",
);

pub fn matrix() -> Chart {
    Chart::new()
        .animation(false)
        .title(Title::new().text("Matrix heatmap"))
        .matrix(
            Matrix::new()
                .id("matrix")
                .left(70)
                .right(30)
                .top(65)
                .bottom(70)
                .x(MatrixDimension::new().data(vec!["East", "West"]))
                .y(MatrixDimension::new().data(vec!["Spring", "Summer"])),
        )
        .visual_map(VisualMap::new().min(0).max(10).show(false))
        .series(
            Heatmap::new()
                .coordinate_system(CoordinateSystem::Matrix)
                .matrix_id("matrix")
                .label(Label::new().show(true))
                .data(vec![
                    vec![0.into(), 0.into(), 2.into()],
                    vec![1.into(), 0.into(), 5.into()],
                    vec![0.into(), 1.into(), 7.into()],
                    vec![1.into(), 1.into(), 9.into()],
                ]),
        )
}

pub fn chord() -> Chart {
    Chart::new()
        .animation(false)
        .title(Title::new().text("Chord flows"))
        .series(
            Chord::new()
                .name("Flow")
                .radius(vec!["50%", "65%"])
                .pad_angle(4)
                .label(Label::new().show(true))
                .data(vec![
                    ChordNode::new().name("A"),
                    ChordNode::new().name("B"),
                    ChordNode::new().name("C"),
                ])
                .links(vec![
                    ChordEdge::new().source("A").target("B").value(8),
                    ChordEdge::new().source("B").target("C").value(5),
                    ChordEdge::new().source("C").target("A").value(3),
                ]),
        )
}

pub fn violin() -> Chart {
    Chart::new()
        .animation(false)
        .title(Title::new().text("Official violin extension"))
        .grid(Grid::new().left(60).right(30).top(65).bottom(50))
        .x_axis(Axis::new().type_(AxisType::Category).data(vec!["A", "B"]))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Custom::new()
                .render_item("violin")
                .item_payload(
                    serde_json::json!({"symbolSize": 4, "bandWidthScale": 0.7, "areaOpacity": 0.7})
                        .as_object()
                        .unwrap()
                        .clone(),
                )
                .item_style(ItemStyle::new().color("#5070dd"))
                .data(vec![
                    vec![0, 2],
                    vec![0, 3],
                    vec![0, 4],
                    vec![0, 5],
                    vec![0, 7],
                    vec![1, 4],
                    vec![1, 5],
                    vec![1, 6],
                    vec![1, 8],
                    vec![1, 9],
                ]),
        )
}

pub fn bar_range() -> Chart {
    Chart::new()
        .animation(false)
        .title(Title::new().text("Official barRange extension"))
        .grid(Grid::new().left(60).right(30).top(65).bottom(50))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["A", "B", "C"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Custom::new()
                .render_item("barRange")
                .encode(
                    DimensionEncode::new()
                        .x(0)
                        .y(vec![1, 2])
                        .tooltip(vec![1, 2]),
                )
                .item_payload(
                    serde_json::json!({"barWidth": "50%", "borderRadius": 4})
                        .as_object()
                        .unwrap()
                        .clone(),
                )
                .item_style(ItemStyle::new().color("#10b981"))
                .data(vec![vec![0, 1, 4], vec![1, 3, 8], vec![2, 2, 6]]),
        )
}
