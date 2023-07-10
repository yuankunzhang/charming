use std::vec;

use echarts::{
    component::{
        Legend, Title, Toolbox, ToolboxFeature, ToolboxFeatureDataZoom, ToolboxFeatureRestore,
        ToolboxFeatureSaveAsImage, Tooltip,
    },
    element::{AxisPointer, TooltipTrigger},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Rainfall vs Evaporation").left("center"))
        .tooltip(
            Tooltip::new()
                .trigger(TooltipTrigger::Axis)
                .axis_pointer(AxisPointer::new()),
        )
        .legend(
            Legend::new()
                .left(10)
                .data(vec!["Evaporation", "Precipitation"]),
        )
        .toolbox(
            Toolbox::new().feature(
                ToolboxFeature::new()
                    .data_zoom(ToolboxFeatureDataZoom::new().y_axis_index("none"))
                    .restore(ToolboxFeatureRestore::new())
                    .save_as_image(ToolboxFeatureSaveAsImage::new()),
            ),
        )
}
