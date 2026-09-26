use assert_json_diff::assert_json_eq;
use charming::element::{
    Color, ItemStyle, JsFunction, Label, LabelPosition, MarkPoint, MarkPointData,
    MarkPointDataType, Symbol, SymbolSize,
};
use serde_json::json;

fn round_trip(mark_point: &MarkPoint) -> MarkPoint {
    let json_string = serde_json::to_string(mark_point).expect("MarkPoint should serialize");
    serde_json::from_str(&json_string).expect("MarkPoint should deserialize")
}

#[test]
fn mark_point_styling_round_trip() {
    let mark_point = MarkPoint::new()
        .data(vec![("max", "Max"), ("min", "Min")])
        .symbol(Symbol::Pin)
        .symbol_size(SymbolSize::Number(50.0))
        .label(
            Label::new()
                .show(true)
                .position(LabelPosition::Top)
                .color("#fff")
                .width(40)
                .height(20),
        )
        .item_style(ItemStyle::new().color("#c23531").border_width(1.5));

    pretty_assertions::assert_eq!(mark_point, round_trip(&mark_point));
}

#[test]
fn mark_point_symbol_size_function_round_trip() {
    let mark_point = MarkPoint::new()
        .data(vec![
            MarkPointData::new()
                .type_(MarkPointDataType::Average)
                .name("Avg"),
        ])
        .symbol_size(SymbolSize::Function(JsFunction::new_with_args(
            "value",
            "return value / 10;",
        )));

    pretty_assertions::assert_eq!(mark_point, round_trip(&mark_point));
}

#[test]
fn mark_point_symbol_callback_json_is_stable() {
    let mark_point = MarkPoint::new()
        .data(vec![("max", "Max")])
        .symbol(Symbol::Callback(JsFunction::new_with_args(
            "value, params",
            "return params.dataIndex % 2 === 0 ? 'circle' : 'rect';",
        )));

    let first = serde_json::to_string(&mark_point).unwrap();
    let second = serde_json::to_string(&round_trip(&mark_point)).unwrap();

    pretty_assertions::assert_eq!(first, second);
}

#[test]
fn mark_point_custom_symbol_round_trip() {
    let mark_point = MarkPoint::new()
        .data(vec![MarkPointData::new().x_axis(1).y_axis(-1.5).value(-2)])
        .symbol(Symbol::Custom("path://M0,0 L10,10".to_string()));

    pretty_assertions::assert_eq!(mark_point, round_trip(&mark_point));
}

#[test]
fn mark_point_json_shape() {
    let mark_point = MarkPoint::new()
        .data(vec![("max", "Max")])
        .symbol(Symbol::RoundRect)
        .symbol_size(SymbolSize::Number(30.0))
        .label(Label::new().show(false))
        .item_style(ItemStyle::new().color(Color::Value("red".to_string())));

    let expected = json!({
        "data": [{ "type": "max", "name": "Max" }],
        "symbol": "roundRect",
        "symbolSize": 30.0,
        "label": { "show": false },
        "itemStyle": { "color": "red" }
    });

    let actual: serde_json::Value = serde_json::to_value(&mark_point).unwrap();
    assert_json_eq!(expected, actual);
}

#[test]
fn mark_point_omits_unset_fields() {
    let json_string = serde_json::to_string(&MarkPoint::new().data(vec![("min", "Min")])).unwrap();

    assert!(!json_string.contains("symbol"));
    assert!(!json_string.contains("label"));
    assert!(!json_string.contains("itemStyle"));
}
