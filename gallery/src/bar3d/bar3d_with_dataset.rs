use echarts::{
    component::{Axis3D, Grid3D, Tooltip, VisualMap},
    element::AxisType,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .grid3d(Grid3D::new())
        .tooltip(Tooltip::new())
        .x_axis3d(Axis3D::new().type_(AxisType::Category))
        .y_axis3d(Axis3D::new().type_(AxisType::Category))
        .z_axis3d(Axis3D::new())
        .visual_map(VisualMap::new().max(1e8))
}
