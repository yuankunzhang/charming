use echarts::{
    component::{Tooltip, TooltipTriggerOn},
    element::{Emphasis, EmphasisFocus, Label, LabelPosition, LineStyle, Orient, TooltipTrigger},
    series::Sankey,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .tooltip(
            Tooltip::new()
                .trigger(TooltipTrigger::Item)
                .trigger_on(TooltipTriggerOn::Mousemove),
        )
        .series(
            Sankey::new()
                .bottom("10%")
                .emphasis(Emphasis::new().focus(EmphasisFocus::Adjacency))
                .orient(Orient::Vertical)
                .label(Label::new().position(LabelPosition::Top))
                .line_style(LineStyle::new().color("source").curveness(0.5))
                .data(vec!["a", "b", "a1", "b1", "c", "e"])
                .links(vec![
                    ("a", "a1", 5),
                    ("e", "b", 3),
                    ("a", "b1", 3),
                    ("b1", "a1", 1),
                    ("b1", "c", 2),
                    ("b", "c", 1),
                ]),
        )
}
