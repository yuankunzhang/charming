use charming::{
    component::{Feature, Grid, Restore, SaveAsImage, Title, Toolbox, ToolboxDataZoom},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(
            Title::new()
                .text("Rainfall and Flow Relationship")
                .left("center"),
        )
        .grid(Grid::new().bottom(80))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new().y_axis_index("none"))
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
}
