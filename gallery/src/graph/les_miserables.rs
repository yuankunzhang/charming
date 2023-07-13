use charming::{
    component::{Legend, Title},
    element::{Label, LabelPosition, LineStyle, Tooltip},
    series::{Graph, GraphData, GraphLayout, GraphLayoutCircular, GraphNodeLabel},
    Chart,
};

pub fn chart() -> Chart {
    let mut data: GraphData = serde_json::from_str(include_str!("les-miserables.json")).unwrap();
    for d in data.nodes.iter_mut() {
        if d.symbol_size > 30.0 {
            d.label = Some(GraphNodeLabel::new().show(true));
        }
    }
    let legend: Vec<String> = data.categories.iter().map(|c| c.name.clone()).collect();
    Chart::new()
        .title(
            Title::new()
                .text("Les Miserables")
                .subtext("Circular layout")
                .top("bottom")
                .left("right"),
        )
        .legend(Legend::new().data(legend))
        .tooltip(Tooltip::new())
        .series(
            Graph::new()
                .name("Les Miserables")
                .layout(GraphLayout::Circular)
                .circular(GraphLayoutCircular::new().rotate_label(true))
                .roam(true)
                .label(Label::new().position(LabelPosition::Right).formatter("{b}"))
                .line_style(LineStyle::new().color("source").curveness(0.3))
                .data(data),
        )
}
