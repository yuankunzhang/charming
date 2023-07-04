use echarts::{
    component::{axis, grid, title, tooltip},
    element::{axis_pointer, axis_type, emphasis, item_style, label, split_line, tooltip_trigger},
    series::{bar, Series},
    Chart,
};

fn main() {
    let chart = Chart::new()
        .title(
            title::Title::new()
                .text("Waterfall Chart")
                .subtext("Living Expenses in Shenzhen"),
        )
        .tooltip(
            tooltip::Tooltip::new()
                .trigger(tooltip_trigger::TooltipTrigger::Axis)
                .formatter(r#"{b0}<br />{a1}: {c1}"#)
                .axis_pointer(
                    axis_pointer::AxisPointer::new().type_(axis_pointer::AxisPointerType::Shadow),
                ),
        )
        .grid(
            grid::Grid::new()
                .left("3%")
                .right("4%")
                .bottom("3%")
                .contain_label(true),
        )
        .x_axis(
            axis::Axis::new()
                .type_(axis_type::AxisType::Category)
                .split_line(split_line::SplitLine::new().show(false))
                .data(vec![
                    "Total",
                    "Rent",
                    "Utilities",
                    "Transportation",
                    "Meals",
                    "Other",
                ]),
        )
        .y_axis(axis::Axis::new().type_(axis_type::AxisType::Value))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Placeholder")
                .stack("Total")
                .item_style(
                    item_style::ItemStyle::new()
                        .color("transparent")
                        .border_color("transparent"),
                )
                .emphasis(
                    emphasis::Emphasis::new().item_style(
                        item_style::ItemStyle::new()
                            .color("transparent")
                            .border_color("transparent"),
                    ),
                )
                .data(vec![0, 1700, 1400, 1200, 300, 0]),
        ))
        .series(Series::Bar(
            bar::Bar::new()
                .name("Life Cost")
                .stack("Total")
                .label(
                    label::Label::new()
                        .show(true)
                        .position(label::Position::Inside),
                )
                .data(vec![2900, 1200, 300, 200, 900, 300]),
        ));

    println!("{}", chart.to_string());
}
