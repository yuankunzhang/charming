#![cfg(not(target_arch = "wasm32"))]

use charming::{
    Chart,
    component::Axis,
    datatype::{Dataset, Source, Transform},
    element::{AnimationTime, Color, Formatter, JsFunction, Label, Symbol, SymbolSize, Tooltip},
    series::{Custom, Line},
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

fn round_trip<T: Serialize + DeserializeOwned>(value: &T) -> T {
    serde_json::from_str(&serde_json::to_string(value).unwrap()).unwrap()
}

fn chart() -> Chart {
    Chart::new()
        .animation(false)
        .x_axis(Axis::new())
        .y_axis(Axis::new())
        .dataset(
            Dataset::new()
                .source(
                    Source::from(vec![vec![1, 10], vec![3, 30]])
                        .id("input")
                        .dimensions(vec!["x", "y"]),
                )
                .transform(
                    Transform::from("{type: 'filter', config: {dimension: 0, gte: 2}}")
                        .id("filtered")
                        .from_dataset_id("input"),
                ),
        )
        .series(
            Line::new()
                .dataset_id("filtered")
                .label(Label::new().show(true).formatter(JsFunction::new_with_args(
                    "p",
                    "return 'kept-' + p.value[0];",
                ))),
        )
        .series(Custom::new().data(vec![vec![0, 0]]).render_item(
            "function() { return {type: 'text', style: {x: 20, y: 40, text: 'custom-executed'}}; }",
        ))
}

#[test]
fn source_json_and_callback_variants_round_trip() {
    let function = JsFunction::new_with_args("value", "return value * 2;");
    assert_eq!(
        serde_json::to_value(&function).unwrap(),
        json!({
            "type": "function", "source": "function(value) { return value * 2; }",
        })
    );
    assert_eq!(function, round_trip(&function));
    assert_eq!(
        function,
        JsFunction::from("function(value) { return value * 2; }".to_owned())
    );
    assert!(JsFunction::from("a") < JsFunction::from("b"));
    for variant in [
        Formatter::Function(function.clone()),
        Formatter::from("#*#*#*#literal#*#*#*#"),
    ] {
        assert_eq!(variant, round_trip(&variant));
    }
    let symbol = Symbol::Callback(function.clone());
    assert_eq!(symbol, round_trip(&symbol));
    let time = AnimationTime::Function(function.clone());
    assert_eq!(time, round_trip(&time));
    let size = SymbolSize::Function(function);
    assert_eq!(size, round_trip(&size));
    assert_eq!(
        AnimationTime::Number(12.0),
        round_trip(&AnimationTime::Number(12.0))
    );
    assert_eq!(
        SymbolSize::Number(12.0),
        round_trip(&SymbolSize::Number(12.0))
    );
    for value in [
        json!({"type":"expression","source":"x"}),
        json!({"source":"x"}),
        json!("old source"),
    ] {
        assert!(serde_json::from_value::<JsFunction>(value).is_err());
    }
}

#[test]
fn color_callback_round_trip_and_validation() {
    let color = Color::Function(JsFunction::new_with_args("p", "return p.color;"));
    assert_eq!(
        serde_json::to_value(&color).unwrap(),
        json!({"type":"function", "source":"function(p) { return p.color; }"})
    );
    assert_eq!(color, round_trip(&color));
    let plain = Color::from("#123456");
    assert_eq!(serde_json::to_value(&plain).unwrap(), json!("#123456"));
    assert_eq!(plain, round_trip(&plain));
    for value in [
        json!({"type":"expression", "source":"x"}),
        json!({"type":"function", "source":42}),
        json!({"type":"function"}),
    ] {
        assert!(serde_json::from_value::<Color>(value).is_err());
    }
}

#[test]
fn chart_dataset_round_trip_and_errors() {
    let chart = chart();
    let saved = serde_json::to_value(&chart).unwrap();
    assert_eq!(
        saved["dataset"][1]["transform"],
        json!({
            "type":"expression", "source":"{type: 'filter', config: {dimension: 0, gte: 2}}",
        })
    );
    assert_eq!(chart, round_trip(&chart));
    for dataset in [
        json!([{"source":"invalid"}]),
        json!([{"transform":{"type":"function","source":"x"}}]),
        json!([{}, {"source":[[1,2]]}]),
    ] {
        assert!(serde_json::from_value::<Chart>(json!({"dataset":dataset})).is_err());
    }
}

#[cfg(feature = "html")]
#[test]
fn html_uses_the_shared_expression_without_validating_source() {
    use charming::HtmlRenderer;
    let chart = chart().tooltip(Tooltip::new().formatter("</script>#*#*#*#literal#*#*#*#"));
    let script = chart.to_javascript().unwrap();
    assert!(!script.contains("</script>"));
    assert!(script.contains("\\u003c"));
    assert!(
        HtmlRenderer::new("callbacks", 400, 300)
            .render(&chart)
            .unwrap()
            .contains(&script)
    );
    let invalid = Chart::new().tooltip(Tooltip::new().formatter(JsFunction::from("function(")));
    assert!(
        HtmlRenderer::new("invalid", 400, 300)
            .render(&invalid)
            .is_ok()
    );
}

#[cfg(feature = "ssr")]
#[test]
fn ssr_executes_custom_formatter_and_transform_after_readback() {
    use charming::{EchartsError, ImageRenderer};
    let mut renderer = ImageRenderer::new(400, 300);
    let svg = renderer.render(&round_trip(&chart())).unwrap();
    assert!(svg.contains("custom-executed"));
    assert!(svg.contains("kept-3"));
    assert!(!svg.contains("kept-1"));
    for source in ["function(", "42"] {
        let invalid = Chart::new().tooltip(Tooltip::new().formatter(JsFunction::from(source)));
        assert!(matches!(
            renderer.render(&invalid),
            Err(EchartsError::JsRuntimeError(_))
        ));
    }
}
