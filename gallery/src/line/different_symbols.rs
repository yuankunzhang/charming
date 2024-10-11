use charming::{
    component::Axis,
    element::{AxisType, Symbol, SymbolSize},
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260])
        .symbol_size(SymbolSize::Function("function (value, params) { return params.dataIndex % 2 === 0 ? value > 200 ? 10 : 20 : 30; }".into()))
        .symbol(Symbol::Callback("function (value, params) { return params.dataIndex % 2 === 0 ? value > 200 ? 'diamond' : 'circle' : 'triangle'; }".into())))
}
