#![cfg(not(target_arch = "wasm32"))]

use charming::{
    Chart,
    component::{
        Axis, DataZoom, DataZoomEmphasis, DataZoomType, MouseWheelModifier, MouseZoom, RangeMode,
    },
    element::{
        AxisLabel, AxisType, BorderCap, BorderJoin, Color, ItemStyle, JsFunction, TextStyle,
    },
    series::Bar,
};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn data_zoom_option_encodings_and_omission() {
    assert_eq!(serde_json::to_value(DataZoom::new()).unwrap(), json!({}));
    assert_eq!(
        serde_json::to_value(DataZoomEmphasis::new()).unwrap(),
        json!({})
    );
    for (zoom, expected) in [
        (
            DataZoom::new()
                .type_(DataZoomType::Slider)
                .width(320)
                .height("12%".to_owned())
                .border_radius(vec![1, 2, 3, 4])
                .handle_size("80%")
                .handle_style(
                    ItemStyle::new()
                        .border_dash_offset(2)
                        .border_cap(BorderCap::Round)
                        .border_join(BorderJoin::Miter)
                        .border_miter_limit(10),
                )
                .move_handle_icon("rect")
                .move_handle_size(7)
                .move_handle_style(ItemStyle::new().color("red"))
                .brush_style(ItemStyle::new().opacity(0.2))
                .emphasis(
                    DataZoomEmphasis::new()
                        .handle_style(ItemStyle::new().border_color("blue"))
                        .move_handle_style(ItemStyle::new().color("green")),
                )
                .show_detail(true)
                .show_data_shadow(true)
                .label_precision("auto")
                .label_formatter("value: {value}")
                .range_mode([RangeMode::Value, RangeMode::Percent])
                .zoom_on_mouse_wheel(MouseZoom::Bool(false))
                .move_on_mouse_move(MouseZoom::Modifier(MouseWheelModifier::Ctrl))
                .move_on_mouse_wheel(MouseZoom::Bool(true))
                .prevent_default_mouse_move(false)
                .z(4)
                .zlevel(1),
            json!({
                "type":"slider", "width":320, "height":"12%", "borderRadius":[1,2,3,4],
                "handleSize":"80%", "handleStyle":{
                    "borderDashOffset":2.0, "borderCap":"round",
                    "borderJoin":"miter", "borderMiterLimit":10.0
                },
                "moveHandleIcon":"rect", "moveHandleSize":7.0,
                "moveHandleStyle":{"color":"red"}, "brushStyle":{"opacity":0.2},
                "emphasis":{"handleStyle":{"borderColor":"blue"}, "moveHandleStyle":{"color":"green"}},
                "showDetail":true, "showDataShadow":true, "labelPrecision":"auto",
                "labelFormatter":"value: {value}", "rangeMode":["value","percent"],
                "zoomOnMouseWheel":false, "moveOnMouseMove":"ctrl", "moveOnMouseWheel":true,
                "preventDefaultMouseMove":false, "z":4.0, "zlevel":1.0
            }),
        ),
        (
            DataZoom::new()
                .width("90%")
                .height(32.5)
                .border_radius(3)
                .handle_size(18)
                .label_precision(2)
                .label_formatter(JsFunction::new_with_args("value", "return 'v-' + value;"))
                .show_data_shadow(false)
                .zoom_on_mouse_wheel(MouseZoom::Modifier(MouseWheelModifier::Shift))
                .move_on_mouse_move(MouseZoom::Modifier(MouseWheelModifier::Alt)),
            json!({
                "width":"90%", "height":32.5, "borderRadius":3, "handleSize":18,
                "labelPrecision":2, "labelFormatter":{
                    "type":"function", "source":"function(value) { return 'v-' + value; }"
                },
                "showDataShadow":false, "zoomOnMouseWheel":"shift", "moveOnMouseMove":"alt"
            }),
        ),
    ] {
        let saved = serde_json::to_value(&zoom).unwrap();
        assert_eq!(saved, expected);
        assert_eq!(serde_json::from_value::<DataZoom>(saved).unwrap(), zoom);
    }
}

#[test]
fn chart_preserves_color_rich_axis_and_data_zoom_after_readback() {
    let chart =
        Chart::new()
            .animation(false)
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["A", "B", "C"])
                    .axis_label(
                        AxisLabel::new()
                            .inside(true)
                            .formatter("{accent|{value}}")
                            .rich(HashMap::from([(
                                "accent".to_owned(),
                                TextStyle::new().color("#123456"),
                            )])),
                    ),
            )
            .y_axis(Axis::new())
            .series(
                Bar::new()
                    .data(vec![10, 20, 30])
                    .item_style(ItemStyle::new().color(Color::Function(
                        JsFunction::new_with_args("p", "return p.dataIndex % 2 ? 'red' : 'blue';"),
                    ))),
            )
            .data_zoom(
                DataZoom::new()
                    .type_(DataZoomType::Slider)
                    .show_data_shadow(true)
                    .label_formatter(JsFunction::new_with_args("value", "return 'v-' + value;")),
            )
            .data_zoom(
                DataZoom::new()
                    .type_(DataZoomType::Inside)
                    .range_mode([RangeMode::Percent, RangeMode::Percent]),
            );
    let saved = serde_json::to_string(&chart).unwrap();
    let restored: Chart = serde_json::from_str(&saved).unwrap();
    assert_eq!(chart, restored);
    let json = serde_json::to_value(&restored).unwrap();
    assert_eq!(
        json["xAxis"]["axisLabel"],
        json!({
            "inside":true, "formatter":"{accent|{value}}", "rich":{"accent":{"color":"#123456"}}
        })
    );
    assert_eq!(json["series"][0]["itemStyle"]["color"]["type"], "function");
    assert_eq!(json["dataZoom"][0]["labelFormatter"]["type"], "function");
    assert_eq!(json["dataZoom"][0]["showDataShadow"], true);
    assert_eq!(
        json["dataZoom"][1]["rangeMode"],
        json!(["percent", "percent"])
    );
}
