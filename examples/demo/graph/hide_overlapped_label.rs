use echarts::{
    component::{legend, tooltip},
    element::{label, line_style, scale_limit},
    series::{graph, Series},
    Chart,
};

pub fn chart() -> Chart {
    let data: graph::Data = serde_json::from_str(include_str!("les-miserables.json")).unwrap();
    Chart::new()
        .tooltip(tooltip::Tooltip::new())
        .legend(
            legend::Legend::new().data(data.categories.iter().map(|c| c.name.clone()).collect()),
        )
        .series(Series::Graph(
            graph::Graph::new()
                .name("Les Miserables")
                .layout(graph::Layout::None)
                .roam(true)
                .label(
                    label::Label::new()
                        .show(true)
                        .position(label::Position::Right)
                        .formatter("{b}"),
                )
                .label_layout(label::LabelLayout::new().hide_overlap(true))
                .scale_limit(scale_limit::ScaleLimit::new().min(0.4).max(2.0))
                .line_style(line_style::LineStyle::new().color("source").curveness(0.3))
                .data(data),
        ))
}
