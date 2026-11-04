#![cfg(all(target_arch = "wasm32", feature = "wasm"))]

use charming::{
    Chart, EchartsError, WasmRenderer,
    component::{Axis, DataZoom, DataZoomType, Grid, Legend, RangeMode, Title},
    datatype::{Dataset, Source, Transform},
    element::{
        AnimationTime, AreaStyle, AxisLabel, AxisType, Color, DataBackground, Formatter, ItemStyle,
        JsFunction, Label, Symbol, SymbolSize, TextStyle, Tooltip,
    },
    series::{Bar, Custom, Line},
};
use js_sys::{Reflect, eval};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn js(source: &str) -> JsValue {
    eval(source).unwrap()
}

fn setup() {
    js(include_str!("../src/asset/echarts-5.5.1.min.js"));
    js(
        "document.getElementById('charming-test-root')?.remove(); const fixture = document.createElement('div'); fixture.id = 'charming-test-root'; fixture.innerHTML = '<div id=chart></div><div id=expression></div>'; document.body.appendChild(fixture); globalThis.customCalls = 0; globalThis.formatterCalls = 0;",
    );
    assert_eq!(js("echarts.version").as_string().unwrap(), "5.5.1");
}

fn chart() -> Chart {
    Chart::new().animation(false)
        .x_axis(Axis::new()).y_axis(Axis::new())
        .dataset(Dataset::new()
            .source(Source::from(vec![vec![1, 10], vec![3, 30]]).id("input").dimensions(vec!["x", "y"]))
            .transform(Transform::from("{type: 'filter', config: {dimension: 0, gte: value}}")
                .id("filtered").from_dataset_id("input")))
        .series(Line::new().dataset_id("filtered").label(Label::new().show(true)
            .formatter(JsFunction::new_with_args("p", "globalThis.formatterCalls++; globalThis.formatted = option + values + p.value[0]; return globalThis.formatted;"))))
        .series(Custom::new().data(vec![vec![0, 0]]).render_item(
            "function() { globalThis.customCalls++; return {type: 'circle', shape: {cx: 10, cy: 10, r: 2}}; }"))
}

fn read_back(chart: &Chart) -> Chart {
    serde_json::from_str(&serde_json::to_string(chart).unwrap()).unwrap()
}

fn assert_executed(instance: &JsValue) {
    Reflect::set(&js_sys::global(), &"instance".into(), instance).unwrap();
    assert_eq!(
        js("instance.getModel().getSeriesByIndex(0).getData().count()").as_f64(),
        Some(1.0)
    );
    assert_eq!(
        js("instance.getModel().getSeriesByIndex(0).getData().getRawDataItem(0)[0]").as_f64(),
        Some(3.0)
    );
    assert_eq!(
        js("globalThis.formatted").as_string().as_deref(),
        Some("kept-3")
    );
    assert!(
        js("globalThis.customCalls > 0 && globalThis.formatterCalls > 0")
            .as_bool()
            .unwrap()
    );
}

#[wasm_bindgen_test]
fn wasm_and_javascript_execute_readback_custom_formatter_and_transform() {
    setup();
    // Bare global names must remain visible to both callbacks and transform expressions.
    js("globalThis.value = 2; globalThis.option = 'kept'; globalThis.values = '-';");
    let chart = read_back(&chart());
    let renderer = WasmRenderer::new(400, 300);
    let instance = renderer.render("chart", &chart).unwrap();
    assert_executed(instance.as_ref());
    js("globalThis.customCalls = 0; globalThis.formatterCalls = 0; globalThis.formatted = null;");
    let option = js(&chart.to_javascript().unwrap());
    Reflect::set(&js_sys::global(), &"exportedOption".into(), &option).unwrap();
    let instance = js(
        "echarts.init(document.getElementById('expression'), null, {renderer:'svg', width:400, height:300})",
    );
    Reflect::set(&js_sys::global(), &"expressionInstance".into(), &instance).unwrap();
    js("expressionInstance.setOption(exportedOption)");
    assert_executed(&instance);
    js(
        "echarts.dispose(document.getElementById('chart')); expressionInstance.dispose(); delete globalThis.value; delete globalThis.option; delete globalThis.values; delete globalThis.formatted;",
    );
}

#[wasm_bindgen_test]
fn source_contract_and_nested_callback_paths() {
    setup();
    let f = JsFunction::new_with_args("v", "return v * 2;");
    assert_eq!(
        f,
        JsFunction::from("function(v) { return v * 2; }".to_string())
    );
    assert!(JsFunction::from("a") < JsFunction::from("b"));
    assert_eq!(
        serde_json::to_value(&f).unwrap(),
        serde_json::json!({"type":"function", "source":"function(v) { return v * 2; }"})
    );
    for value in [
        serde_json::json!({"type":"expression","source":"x"}),
        serde_json::json!("x"),
    ] {
        assert!(serde_json::from_value::<JsFunction>(value).is_err());
    }
    let chart = read_back(
        &Chart::new()
            .animation_duration(f.clone())
            .legend(vec![Legend::new().animation_duration_update(f.clone())])
            .x_axis(Axis::new())
            .y_axis(Axis::new())
            .series(
                Line::new()
                    .symbol(Symbol::Callback(f.clone()))
                    .symbol_size(f.clone()),
            )
            .tooltip(Tooltip::new().formatter(f.clone())),
    );
    let json = serde_json::to_value(&chart).unwrap();
    assert_eq!(
        json["tooltip"]["formatter"],
        serde_json::to_value(Formatter::Function(f.clone())).unwrap()
    );
    assert_eq!(
        serde_json::from_value::<AnimationTime>(json["animationDuration"].clone()).unwrap(),
        AnimationTime::Function(f.clone())
    );
    assert_eq!(
        serde_json::from_value::<SymbolSize>(json["series"][0]["symbolSize"].clone()).unwrap(),
        SymbolSize::Function(f)
    );
    js(
        "globalThis.originalInit = echarts.init; echarts.init = function(...args) { const c = originalInit(...args); c.setOption = function(o) { globalThis.captured = o; }; return c; };",
    );
    WasmRenderer::new(400, 300).render("chart", &chart).unwrap();
    assert!(js("captured.animationDuration(3) === 6 && captured.legend[0].animationDurationUpdate(3) === 6 && captured.series[0].symbol(3) === 6 && captured.series[0].symbolSize(3) === 6 && captured.tooltip.formatter(3) === 6").as_bool().unwrap());
    js("echarts.dispose(document.getElementById('chart')); echarts.init = originalInit;");
}

#[wasm_bindgen_test]
fn ordinary_data_maps_and_nonfinite_numbers_stay_data() {
    setup();
    let literal = "</script>#*#*#*#globalThis.shouldNotRun = true#*#*#*#";
    let chart = Chart::new()
        .title(Title::new().text(literal))
        .legend(Legend::new().selected([("other", true), ("line", false)]))
        .tooltip(Tooltip::new().formatter(literal))
        .series(Line::new().data(vec![f64::NAN, f64::INFINITY, f64::NEG_INFINITY]));
    js(
        "globalThis.originalInit = echarts.init; echarts.init = function(...args) { const c = originalInit(...args); c.setOption = function(o) { globalThis.captured = o; }; return c; };",
    );
    WasmRenderer::new(400, 300).render("chart", &chart).unwrap();
    assert!(js("Object.getPrototypeOf(captured.legend.selected) === Object.prototype && captured.legend.selected.other === true && captured.legend.selected.line === false && Number.isNaN(captured.series[0].data[0]) && captured.series[0].data[1] === Infinity && captured.series[0].data[2] === -Infinity").as_bool().unwrap());
    let script = chart.to_javascript().unwrap();
    assert!(!script.contains("</script>"));
    let option = js(&script);
    Reflect::set(&js_sys::global(), &"option".into(), &option).unwrap();
    assert!(js("option.series[0].data.every(v => v === null) && option.tooltip.formatter === captured.tooltip.formatter && globalThis.shouldNotRun === undefined && option.legend.selected.other === true").as_bool().unwrap());
    js("echarts.dispose(document.getElementById('chart')); echarts.init = originalInit;");
}

#[wasm_bindgen_test]
fn preparation_errors_and_instance_ownership() {
    setup();
    let renderer = WasmRenderer::new(400, 300);
    js(
        "globalThis.originalInit = echarts.init; globalThis.initCalls = 0; echarts.init = function(...args) { globalThis.initCalls++; return originalInit(...args); };",
    );
    for source in [
        "function(",
        "42",
        "(() => { throw new Error('prepare'); })()",
    ] {
        let chart = Chart::new().tooltip(Tooltip::new().formatter(JsFunction::from(source)));
        assert!(matches!(
            renderer.render("chart", &chart),
            Err(EchartsError::WasmError(_))
        ));
        assert!(eval(&chart.to_javascript().unwrap()).is_err());
    }
    assert_eq!(js("initCalls").as_f64(), Some(0.0));
    js("echarts.init = originalInit;");
    let instance = renderer.render("chart", &Chart::new()).unwrap();
    Reflect::set(&js_sys::global(), &"instance".into(), instance.as_ref()).unwrap();
    js(
        "globalThis.setCalls = 0; instance.setOption = function() { globalThis.setCalls++; throw new Error('setOption failed'); };",
    );
    let invalid = Chart::new().tooltip(Tooltip::new().formatter(JsFunction::from("42")));
    assert!(WasmRenderer::update(&instance, &invalid).is_err());
    assert_eq!(js("setCalls").as_f64(), Some(0.0));
    assert!(WasmRenderer::update(&instance, &Chart::new()).is_err());
    assert!(renderer.render("chart", &Chart::new()).is_err());
    assert!(js("echarts.getInstanceByDom(document.getElementById('chart')) === instance && !instance.isDisposed()").as_bool().unwrap());
    js(
        "instance.dispose(); echarts.init = function(...args) { const c = originalInit(...args); globalThis.created = c; c.setOption = function() { throw new Error('new setOption failed'); }; return c; };",
    );
    assert!(renderer.render("chart", &Chart::new()).is_err());
    assert!(js("created.isDisposed() && echarts.getInstanceByDom(document.getElementById('chart')) === undefined").as_bool().unwrap());
    js("echarts.init = function() { throw new Error('init failed'); };");
    assert!(matches!(
        renderer.render("chart", &Chart::new()),
        Err(EchartsError::WasmError(_))
    ));
    js("echarts.init = originalInit;");
}

#[wasm_bindgen_test]
fn repeated_render_and_update_use_current_source() {
    setup();
    let renderer = WasmRenderer::new(400, 300);
    let make =
        |body| Chart::new().tooltip(Tooltip::new().formatter(JsFunction::new_with_args("", body)));
    let instance = renderer.render("chart", &make("return 'first';")).unwrap();
    Reflect::set(&js_sys::global(), &"instance".into(), instance.as_ref()).unwrap();
    assert_eq!(
        js("instance.getOption().tooltip[0].formatter()")
            .as_string()
            .as_deref(),
        Some("first")
    );
    WasmRenderer::update(&instance, &make("return 'second';")).unwrap();
    assert_eq!(
        js("instance.getOption().tooltip[0].formatter()")
            .as_string()
            .as_deref(),
        Some("second")
    );
    let again = renderer
        .render("chart", &read_back(&make("return 'third';")))
        .unwrap();
    assert_eq!(JsValue::from(again), JsValue::from(instance));
    assert_eq!(
        js("instance.getOption().tooltip[0].formatter()")
            .as_string()
            .as_deref(),
        Some("third")
    );
    js("instance.dispose()");
}

#[wasm_bindgen_test]
fn color_rich_axis_and_data_zoom_execute_after_readback() {
    setup();
    let make = |shadow, formatter: Formatter| {
        read_back(&Chart::new()
            .animation(false)
            .grid(Grid::new().bottom(90))
            .x_axis(Axis::new().type_(AxisType::Category).data(vec!["A", "B", "C"])
                .axis_label(AxisLabel::new().inside(false).interval(0).formatter("{accent|{value}}")
                    .rich(HashMap::from([("accent".to_owned(), TextStyle::new().color("#13579b"))]))))
            .y_axis(Axis::new())
            .series(Bar::new().data(vec![10, 20, 30]).item_style(ItemStyle::new()
                .color(Color::Function(JsFunction::new_with_args("p",
                    "globalThis.integrationColorCalls++; return p.dataIndex % 2 ? '#2b7d91' : '#c13a57';")))))
            .data_zoom(DataZoom::new().type_(DataZoomType::Slider).x_axis_index(0)
                .bottom(20).height(30).start(0).end(100)
                .range_mode([RangeMode::Percent, RangeMode::Percent])
                .show_data_shadow(shadow).show_detail(true).label_formatter(formatter)
                .data_background(DataBackground::new().area_style(AreaStyle::new().color("#9b31e7")))
                .selected_data_background(DataBackground::new().area_style(AreaStyle::new().color("#9b31e7"))))
            .data_zoom(DataZoom::new().type_(DataZoomType::Inside).x_axis_index(0)
                .range_mode([RangeMode::Percent, RangeMode::Percent])))
    };
    let initial = make(
        true,
        JsFunction::new_with_args(
            "value, text",
            "globalThis.integrationLabelCalls++; return 'fn-' + text;",
        )
        .into(),
    );
    let updated = make(false, "template-{value}".into());
    for use_wasm in [true, false] {
        js(r#"
            globalThis.integrationColorCalls = 0;
            globalThis.integrationLabelCalls = 0;
            (() => {
                const init = echarts.init;
                globalThis.integrationRestoreInit = () => { echarts.init = init; };
                echarts.init = function(...args) {
                    const instance = init(...args);
                    const setOption = instance.setOption;
                    instance.setOption = function(option, ...rest) {
                        globalThis.integrationCaptured = option;
                        return setOption.call(this, option, ...rest);
                    };
                    return instance;
                };
            })();
        "#);
        let result = (|| -> Result<(), JsValue> {
            let instance = if use_wasm {
                Some(
                    WasmRenderer::new(400, 300)
                        .render("chart", &initial)
                        .map_err(|error| JsValue::from_str(&error.to_string()))?,
                )
            } else {
                None
            };
            if let Some(instance) = &instance {
                Reflect::set(
                    &js_sys::global(),
                    &"integrationInstance".into(),
                    instance.as_ref(),
                )?;
            } else {
                eval(
                    "globalThis.integrationInstance = echarts.init(document.getElementById('expression'), null, {renderer:'svg', width:400, height:300})",
                )?;
                let option = eval(&initial.to_javascript().unwrap())?;
                Reflect::set(&js_sys::global(), &"integrationOption".into(), &option)?;
                eval("integrationInstance.setOption(integrationOption)")?;
            }
            // Slider labels remain hidden until interaction; inspect the engine's text objects.
            eval(
                r#"
                (() => {
                    const data = integrationInstance.getModel().getSeriesByIndex(0).getData();
                    if (!(integrationColorCalls > 0 && data.count() === 3 &&
                        ['#c13a57', '#2b7d91', '#c13a57'].every((color, i) => data.getItemGraphicEl(i).style.fill === color)))
                        throw new Error('color callback did not paint the bars');
                    const rich = integrationCaptured.xAxis.axisLabel.rich;
                    if (!(Object.getPrototypeOf(rich) === Object.prototype &&
                        Object.keys(rich).join(',') === 'accent' && rich.accent.color === '#13579b'))
                        throw new Error('rich styles were not preserved as an ordinary object');
                    const drawn = integrationInstance.getZr().storage.getDisplayList(true);
                    const labels = drawn.filter(el => el.type === 'tspan' && el.style.fill === '#13579b');
                    if (labels.map(el => el.style.text).sort().join(',') !== 'A,B,C')
                        throw new Error('rich axis text or fill was not rendered');
                    if (!drawn.some(el => el.type === 'polygon' && el.style.fill === '#9b31e7'))
                        throw new Error('enabled slider data shadow was not drawn');
                    const view = integrationInstance.getViewOfComponentModel(integrationInstance.getModel().getComponent('dataZoom', 0));
                    if (!(integrationLabelCalls > 0 && view._displayables.handleLabels.map(el => el.style.text).join(',') === 'fn-A,fn-C'))
                        throw new Error('function slider labels were not generated');
                })();
            "#,
            )?;
            if let Some(instance) = &instance {
                WasmRenderer::update(instance, &updated)
                    .map_err(|error| JsValue::from_str(&error.to_string()))?;
            } else {
                let option = eval(&updated.to_javascript().unwrap())?;
                Reflect::set(&js_sys::global(), &"integrationOption".into(), &option)?;
                eval("integrationInstance.setOption(integrationOption)")?;
            }
            eval(
                r#"
                (() => {
                    const drawn = integrationInstance.getZr().storage.getDisplayList(true);
                    if (drawn.some(el => el.type === 'polygon' && el.style.fill === '#9b31e7'))
                        throw new Error('disabled slider data shadow is still drawn');
                    const view = integrationInstance.getViewOfComponentModel(integrationInstance.getModel().getComponent('dataZoom', 0));
                    if (view._displayables.handleLabels.map(el => el.style.text).join(',') !== 'template-A,template-C')
                        throw new Error('template slider labels were not generated');
                    integrationInstance.dispatchAction({type:'dataZoom', dataZoomIndex:1, start:50, end:100});
                    const data = integrationInstance.getModel().getSeriesByIndex(0).getData();
                    if (!(data.count() === 2 && data.getName(0) === 'B' && data.getName(1) === 'C'))
                        throw new Error('inside zoom did not retain only B and C');
                })();
            "#,
            )?;
            Ok(())
        })();
        js(r#"
            echarts.dispose(document.getElementById('chart'));
            echarts.dispose(document.getElementById('expression'));
            integrationRestoreInit();
            delete globalThis.integrationRestoreInit;
            delete globalThis.integrationInstance;
            delete globalThis.integrationCaptured;
            delete globalThis.integrationOption;
            delete globalThis.integrationColorCalls;
            delete globalThis.integrationLabelCalls;
        "#);
        result.unwrap();
    }
}
