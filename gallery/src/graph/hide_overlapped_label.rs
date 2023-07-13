use charming::{
    component::Legend,
    element::{Label, LabelLayout, LabelPosition, LineStyle, ScaleLimit, Tooltip},
    series::{Graph, GraphData, GraphLayout},
    Chart,
};

pub fn chart() -> Chart {
    let data: GraphData = serde_json::from_str(include_str!("les-miserables.json")).unwrap();
    Chart::new()
        .tooltip(Tooltip::new())
        .legend(Legend::new().data(data.categories.iter().map(|c| c.name.clone()).collect()))
        .series(
            Graph::new()
                .name("Les Miserables")
                .layout(GraphLayout::None)
                .roam(true)
                .label(
                    Label::new()
                        .show(true)
                        .position(LabelPosition::Right)
                        .formatter("{b}"),
                )
                .label_layout(LabelLayout::new().hide_overlap(true))
                .scale_limit(ScaleLimit::new().min(0.4).max(2.0))
                .line_style(LineStyle::new().color("source").curveness(0.3))
                .data(data),
        )
}
