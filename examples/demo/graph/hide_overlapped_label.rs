use echarts::{
    component::{Legend, Tooltip},
    element::{Label, LabelLayout, LineStyle, Position, ScaleLimit},
    series::{graph, Series},
    Chart,
};

pub fn chart() -> Chart {
    let data: graph::Data = serde_json::from_str(include_str!("les-miserables.json")).unwrap();
    Chart::new()
        .tooltip(Tooltip::new())
        .legend(Legend::new().data(data.categories.iter().map(|c| c.name.clone()).collect()))
        .series(Series::Graph(
            graph::Graph::new()
                .name("Les Miserables")
                .layout(graph::Layout::None)
                .roam(true)
                .label(
                    Label::new()
                        .show(true)
                        .position(Position::Right)
                        .formatter("{b}"),
                )
                .label_layout(LabelLayout::new().hide_overlap(true))
                .scale_limit(ScaleLimit::new().min(0.4).max(2.0))
                .line_style(LineStyle::new().color("source").curveness(0.3))
                .data(data),
        ))
}
