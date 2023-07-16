use charming::{
    component::{Axis3D, Grid3D, VisualMap},
    datatype::{CompositeValue, Dataset},
    element::{AxisType, DimensionEncode, Tooltip},
    series::Bar3d,
    Chart,
};

pub fn chart() -> Chart {
    let data: Vec<Vec<CompositeValue>> =
        serde_json::from_str(include_str!("../../asset/life-expectancy-table.json")).unwrap();

    Chart::new()
        .grid3d(Grid3D::new())
        .tooltip(Tooltip::new())
        .x_axis3d(Axis3D::new().type_(AxisType::Category))
        .y_axis3d(Axis3D::new().type_(AxisType::Category))
        .z_axis3d(Axis3D::new())
        .visual_map(VisualMap::new().max(1e8).dimension("Population"))
        .dataset(Dataset::new().source(data))
        .series(
            Bar3d::new().shading("lambert").encode(
                DimensionEncode::new()
                    .x("Year")
                    .y("Country")
                    .z("Life Expectancy")
                    .tooltip(vec![0, 1, 2, 3, 4]),
            ),
        )
}
