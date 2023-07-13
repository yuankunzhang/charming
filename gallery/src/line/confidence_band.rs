use charming::{
    component::{Axis, Grid, Title},
    element::{
        AreaStyle, AxisLabel, AxisPointer, AxisPointerType, AxisType, Formatter, ItemStyle, Label,
        LineStyle, Symbol, Tooltip, Trigger,
    },
    series::Line,
    Chart,
};
use serde::Deserialize;

pub fn chart() -> Chart {
    let data: Vec<DataItem> =
        serde_json::from_str(include_str!("../../asset/confidence-band.json"))
            .expect("failed to parse data");

    let base = -data
        .iter()
        .fold(f64::INFINITY, |min, val| f64::floor(f64::min(min, val.l)));

    Chart::new()
        .title(
            Title::new()
                .text("Confidence Band")
                .subtext("Example in MetricsGraphics.js")
                .left("center"),
        )
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(
                    AxisPointer::new().type_(AxisPointerType::Cross).label(
                        Label::new()
                            .background_color("#ccc")
                            .border_color("#aaa")
                            .border_width(1)
                            .shadow_blur(0)
                            .shadow_offset_x(0)
                            .shadow_offset_y(0)
                            .color("#222"),
                    ),
                )
                .formatter(
                    Formatter::Function(
                        format!("function (params) {{ return (params[2].name + '<br />' + ((params[2].value - {}) * 100).toFixed(1) + '%'); }}", base
                    ).into())
                ),
        )
        .grid(Grid::new().left("3%").right("4%").bottom("3%").contain_label(true))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(data.iter().map(|x| x.date.clone()).collect())
                .axis_label(
                    AxisLabel::new().formatter(
                        Formatter::Function(
                            "function (value, idx) { var date = new Date(value); return idx === 0 ? value : [date.getMonth() + 1, date.getDate()].join('-'); }".into())
                        )
                    )
                .boundary_gap(false)
        )
        .y_axis(
            Axis::new()
                .axis_label(AxisLabel::new().formatter(
                    Formatter::Function(format!("function (val) {{ return (val - {}) * 100 + '%'; }}", base).into()))
                )
                .axis_pointer(
                    AxisPointer::new().label(
                        Label::new().formatter(
                            Formatter::Function(format!("function (params) {{ return ((params.value - {}) * 100).toFixed(1) + '%'; }}", base).into())
                        )
                    )
                ).split_number(3)
        )
        .series(
            Line::new()
                .name("L")
                .data(data.iter().map(|x| x.l + base).collect())
                .line_style(LineStyle::new().opacity(0))
                .stack("confidence-band")
                .symbol(Symbol::None)
        )
        .series(
            Line::new()
                .name("U")
                .data(data.iter().map(|x| x.u - x.l).collect())
                .line_style(LineStyle::new().opacity(0))
                .area_style(AreaStyle::new().color("#ccc"))
                .stack("confidence-band")
                .symbol(Symbol::None)
        )
        .series(
            Line::new()
                .data(data.iter().map(|x| x.value + base).collect())
                .item_style(ItemStyle::new().color("#333"))
                .show_symbol(false))
}

#[derive(Deserialize)]
struct DataItem {
    date: String,
    value: f64,
    l: f64,
    u: f64,
}
