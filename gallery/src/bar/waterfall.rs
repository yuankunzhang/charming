use charming::{
    component::{Axis, Grid, Title},
    element::{
        AxisPointer, AxisPointerType, AxisType, Emphasis, ItemStyle, Label, LabelPosition,
        SplitLine, Tooltip, Trigger,
    },
    series::{bar, Bar, Series},
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(
            Title::new()
                .text("Waterfall Chart")
                .subtext("Living Expenses in Shenzhen"),
        )
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .formatter(r#"{b0}<br />{a1}: {c1}"#)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        )
        .grid(
            Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .split_line(SplitLine::new().show(false))
                .data(vec![
                    "Total",
                    "Rent",
                    "Utilities",
                    "Transportation",
                    "Meals",
                    "Other",
                ]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Placeholder")
                .stack("Total")
                .item_style(
                    ItemStyle::new()
                        .color("transparent")
                        .border_color("transparent"),
                )
                .emphasis(
                    Emphasis::new().item_style(
                        ItemStyle::new()
                            .color("transparent")
                            .border_color("transparent"),
                    ),
                )
                .data(vec![0, 1700, 1400, 1200, 300, 0]),
        ))
        .series(
            Bar::new()
                .name("Life Cost")
                .stack("Total")
                .label(Label::new().show(true).position(LabelPosition::Inside))
                .data(vec![2900, 1200, 300, 200, 900, 300]),
        )
}
