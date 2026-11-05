#![cfg(all(target_arch = "wasm32", feature = "wasm", feature = "html"))]

use charming::{
    Chart, Echarts, HtmlRenderer, WasmRenderer,
    component::{
        Axis, BrushComponent, Calendar, DataZoom, DataZoomType, Feature, Geo, Grid, HandleLabel,
        Legend, Matrix, MatrixDimension, SaveAsImage, SaveAsImageType, Thumbnail, Timeline,
        Toolbox,
    },
    element::{
        AxisBreak, AxisType, CoordinateSystem, CoordinateSystemUsage, ItemStyle, JsFunction,
    },
    series::{Bar, Custom, Line, Lines, Map, MapData, Scatter},
    theme::Theme,
};
use js_sys::{Promise, Reflect, eval, futures::JsFuture};
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_test::*;

#[path = "../../gallery/src/bar3d/bar3d_with_dataset.rs"]
mod bar3d;
#[path = "../../gallery/src/echarts6.rs"]
mod examples;

wasm_bindgen_test_configure!(run_in_browser);

const LOCALE: &str = "echarts.registerLocale('ACCEPTANCE', {time:{monthAbbr:['Jan-test','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec']}});";
const THEME: Theme = Theme::Custom(
    "acceptance",
    "echarts.registerTheme('acceptance',{backgroundColor:'#f4f7fa'});",
);
const MAP: &str = r#"echarts.registerMap('acceptance-map', {type:'FeatureCollection',features:[
    {type:'Feature',properties:{name:'West',echartsStyle:{itemStyle:{areaColor:'#5070dd'}}},geometry:{type:'Polygon',coordinates:[[[0,0],[1,0],[1,1],[0,1],[0,0]]]}},
    {type:'Feature',properties:{name:'East'},geometry:{type:'Polygon',coordinates:[[[1,0],[2,0],[2,1],[1,1],[1,0]]]}}
]});"#;

fn js(source: &str) -> JsValue {
    eval(source).unwrap()
}
fn expose(value: &JsValue) {
    Reflect::set(&js_sys::global(), &"instance".into(), value).unwrap();
}
async fn promise(source: &str) -> JsValue {
    JsFuture::from(js(source).unchecked_into::<Promise>())
        .await
        .unwrap()
}
fn setup() {
    WasmRenderer::register_extension(include_str!("../src/asset/echarts-6.1.0.min.js")).unwrap();
    js(
        "document.getElementById('echarts6-fixture')?.remove(); var root = document.createElement('div'); root.id='echarts6-fixture'; root.innerHTML='<div id=chart></div>'; document.body.appendChild(root); globalThis.renderErrors=[]; window.onerror=(m)=>renderErrors.push(String(m)); window.onunhandledrejection=(e)=>renderErrors.push(String(e.reason));",
    );
    assert_eq!(js("echarts.version").as_string().as_deref(), Some("6.1.0"));
}
fn svg(chart: Chart) -> Chart {
    chart.toolbox(
        Toolbox::new()
            .show(false)
            .feature(Feature::new().save_as_image(SaveAsImage::new().type_(SaveAsImageType::Svg))),
    )
}
fn render(chart: &Chart) -> Echarts {
    let chart: Chart = serde_json::from_str(&serde_json::to_string(chart).unwrap()).unwrap();
    let instance = WasmRenderer::new(600, 450).render("chart", &chart).unwrap();
    expose(instance.as_ref());
    instance
}
async fn foreground() {
    let count = promise(r#"new Promise((resolve,reject)=>{
        const image = new Image(); image.onerror=()=>reject(new Error('image decode'));
        image.onload=()=>{ const c=document.createElement('canvas'); c.width=600;c.height=450;
            const ctx=c.getContext('2d');ctx.drawImage(image,0,0);const p=ctx.getImageData(75,85,485,295).data;
            let n=0;for(let i=0;i<p.length;i+=4)if(p[i+3]>100&&Math.max(p[i],p[i+1],p[i+2])-Math.min(p[i],p[i+1],p[i+2])>40)n++;
            resolve(n);}; image.src=instance.getDataURL({type:'png'});
    })"#).await.as_f64().unwrap();
    assert!(count > 300.0, "only {count} colored foreground pixels");
    assert_eq!(
        js("JSON.stringify(renderErrors)").as_string().as_deref(),
        Some("[]")
    );
}
fn dispose() {
    js("instance.dispose()");
}

#[wasm_bindgen_test(async)]
async fn registered_series_layout_conversions_and_theme_changes() {
    setup();
    WasmRenderer::register_extension(examples::EXTENSIONS).unwrap();
    WasmRenderer::register_extension(LOCALE).unwrap();
    assert!(WasmRenderer::register_extension("throw new Error('registration')").is_err());
    for make in [
        examples::matrix,
        examples::chord,
        examples::violin,
        examples::bar_range,
    ] {
        render(&make());
        foreground().await;
        dispose();
    }
    let instance = WasmRenderer::new(600, 450)
        .theme(THEME)
        .locale("ACCEPTANCE")
        .render("chart", &svg(examples::matrix()))
        .unwrap();
    expose(instance.as_ref());
    assert!(js("instance.getZr().painter.getType()==='svg' && instance.getOption().backgroundColor==='#f4f7fa' && instance.getModel().getLocaleModel().get(['time','monthAbbr'])[0]==='Jan-test'").as_bool().unwrap());
    let layout =
        WasmRenderer::convert_to_layout(&instance, js("({matrixId:'matrix'})"), js("[0,0]"), None)
            .unwrap();
    Reflect::set(&js_sys::global(), &"layout".into(), &layout).unwrap();
    assert!(
        js("layout.rect.width>0 && layout.rect.height>0 && Number.isFinite(layout.rect.x)")
            .as_bool()
            .unwrap()
    );
    let pixel =
        WasmRenderer::convert_to_pixel(&instance, js("({matrixId:'matrix'})"), js("[0,0]"), None)
            .unwrap();
    Reflect::set(&js_sys::global(), &"pixel".into(), &pixel).unwrap();
    assert!(
        js("pixel.length===2 && pixel.every(Number.isFinite)")
            .as_bool()
            .unwrap()
    );
    let value =
        WasmRenderer::convert_from_pixel(&instance, js("({matrixId:'matrix'})"), pixel, None)
            .unwrap();
    Reflect::set(&js_sys::global(), &"converted".into(), &value).unwrap();
    assert!(
        js("converted[0]===0 && converted[1]===0")
            .as_bool()
            .unwrap()
    );
    let mut value = serde_json::to_value(examples::matrix()).unwrap();
    value["series"][0]["data"][0][2] = 6.into();
    let current: Chart = serde_json::from_value(value).unwrap();
    for theme in [Theme::Dark, Theme::Default] {
        WasmRenderer::replace(&instance, &current).unwrap();
        WasmRenderer::set_theme(&instance, theme, true).unwrap();
        assert!(js("instance.getOption().series[0].data[0][2]===6 && instance.getOption().matrix[0].id==='matrix'").as_bool().unwrap());
        foreground().await;
    }
    dispose();
}

fn nested_layout() -> Chart {
    Chart::new()
        .animation(false)
        .matrix(
            Matrix::new()
                .left(50)
                .top(30)
                .width(450)
                .height(170)
                .x(MatrixDimension::new().show(false).length(1usize))
                .y(MatrixDimension::new().show(false).length(1usize)),
        )
        .grid(
            Grid::new()
                .coordinate_system(CoordinateSystem::Matrix)
                .matrix_index(0)
                .coordinate_system_usage(CoordinateSystemUsage::Box)
                .coord(vec![0, 0])
                .left(20)
                .right(20)
                .top(20)
                .bottom(20),
        )
        .calendar(
            Calendar::new()
                .id("calendar")
                .range(("2026-11-01", "2026-11-07"))
                .left(50)
                .top(230)
                .width(450)
                .height(170),
        )
        .grid(
            Grid::new()
                .coordinate_system(CoordinateSystem::Calendar)
                .calendar_id("calendar")
                .coord("2026-11-03")
                .left(5)
                .right(5)
                .top(5)
                .bottom(5),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["A", "B", "C"])
                .grid_index(0),
        )
        .y_axis(Axis::new().grid_index(0))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["A", "B", "C"])
                .grid_index(1)
                .show(false),
        )
        .y_axis(Axis::new().grid_index(1).show(false))
        .series(
            Bar::new()
                .data(vec![3, 6, 4])
                .x_axis_index(0)
                .y_axis_index(0),
        )
        .series(
            Line::new()
                .data(vec![2, 5, 3])
                .x_axis_index(1)
                .y_axis_index(1),
        )
}
fn interaction_chart() -> Chart {
    Chart::new()
        .animation(false)
        .legend(Legend::new().data(vec!["values"]))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["A", "B", "C"])
                .jitter(12)
                .jitter_overlap(false),
        )
        .y_axis(Axis::new().breaks(vec![AxisBreak::new().start(10).end(80).gap("10%")]))
        .data_zoom(
            DataZoom::new()
                .type_(DataZoomType::Slider)
                .handle_label(HandleLabel::new().show(true)),
        )
        .brush(BrushComponent::new().x_axis_index(0))
        .series(Scatter::new().name("values").symbol_size(20).data(vec![
            vec![0, 2],
            vec![0, 4],
            vec![1, 5],
            vec![2, 90],
        ]))
}
fn map_chart() -> Chart {
    Chart::new()
        .animation(false)
        .geo(
            Geo::new()
                .id("geo")
                .map("acceptance-map")
                .left(40)
                .right(180)
                .top(50)
                .bottom(60),
        )
        .series(
            Map::new()
                .id("map")
                .geo_id("geo")
                .roam(true)
                .item_style(ItemStyle::new().area_color("#10b981"))
                .data(vec![
                    MapData::new().name("West").value(2).silent(true),
                    MapData::new().name("East").value(5),
                ]),
        )
        .series(
            Lines::new()
                .coordinate_system(CoordinateSystem::Geo)
                .geo_id("geo")
                .data(vec![vec![vec![0.2, 0.2], vec![1.8, 0.8]]]),
        )
        .thumbnail(
            Thumbnail::new()
                .series_id("map")
                .right(10)
                .top(70)
                .width(150)
                .height(120)
                .window_style(ItemStyle::new().color("rgba(80,112,221,0.2)")),
        )
}
fn timeline_chart() -> Chart {
    Chart::new()
        .base_option(
            Chart::new()
                .animation(false)
                .timeline(
                    Timeline::new()
                        .axis_type(AxisType::Category)
                        .data(vec!["First", "Second"]),
                )
                .x_axis(Axis::new().type_(AxisType::Category).data(vec!["A", "B"]))
                .y_axis(Axis::new()),
        )
        .options(Chart::new().series(Bar::new().data(vec![3, 5])))
        .options(Chart::new().series(Bar::new().data(vec![6, 2])))
}
#[wasm_bindgen_test(async)]
async fn composition_axis_interaction_and_existing_component_gaps() {
    setup();
    WasmRenderer::register_extension(MAP).unwrap();
    for make in [nested_layout, map_chart, timeline_chart, time_chart] {
        render(&make());
        foreground().await;
        dispose();
    }
    let instance = render(&interaction_chart());
    foreground().await;
    let calls = Rc::new(Cell::new(0));
    let captured = calls.clone();
    let callback = Closure::<dyn FnMut(JsValue)>::new(move |_| captured.set(captured.get() + 1));
    let function = callback.as_ref().unchecked_ref();
    WasmRenderer::on(&instance, "legendselectchanged", function).unwrap();
    WasmRenderer::dispatch_action(&instance, js("({type:'legendToggleSelect',name:'values'})"))
        .unwrap();
    assert_eq!(calls.get(), 1);
    WasmRenderer::off(&instance, "legendselectchanged", function).unwrap();
    WasmRenderer::dispatch_action(&instance, js("({type:'legendToggleSelect',name:'values'})"))
        .unwrap();
    assert_eq!(calls.get(), 1);
    dispose();
}

#[wasm_bindgen_test(async)]
async fn html_pages_load_pinned_cdn_and_render_registered_names_and_functions() {
    setup();
    for make in [examples::violin, examples::bar_range, explicit_custom] {
        let html = HtmlRenderer::new("ECharts 6", 600, 450).theme(THEME).locale("ACCEPTANCE")
            .extension_source(examples::EXTENSIONS).extension_source(LOCALE)
            .extension_source("globalThis.explicitCalls=0;globalThis.payloadExecuted=false;globalThis.literal = '</script>'; globalThis.registrationBeforeInit = !echarts.getInstanceByDom(document.getElementById('chart'));")
            .render(&svg(make())).unwrap();
        let html = html.replacen("<head>", "<head><script>globalThis.pageErrors=[];window.onerror=(m)=>pageErrors.push(String(m));window.onunhandledrejection=(e)=>pageErrors.push(String(e.reason));</script>", 1);
        Reflect::set(&js_sys::global(), &"pageSource".into(), &html.into()).unwrap();
        promise(r#"new Promise((resolve,reject)=>{ const frame=document.createElement('iframe');frame.id='html-frame';frame.width=650;frame.height=500;frame.srcdoc=pageSource;document.body.appendChild(frame);
            const started=Date.now();const timer=setInterval(()=>{const w=frame.contentWindow;if(w.pageErrors?.length){clearInterval(timer);reject(new Error(w.pageErrors.join(';')));}else if(w.chart){clearInterval(timer);globalThis.htmlWindow=w;resolve(true);}else if(Date.now()-started>20000){clearInterval(timer);reject(new Error('CDN page timeout'));}},25); })"#).await;
        assert!(js("htmlWindow.echarts.version==='6.1.0' && htmlWindow.literal==='</script>' && htmlWindow.registrationBeforeInit && htmlWindow.chart.getZr().painter.getType()==='svg' && htmlWindow.chart.getOption().backgroundColor==='#f4f7fa' && htmlWindow.chart.getModel().getLocaleModel().get(['time','monthAbbr'])[0]==='Jan-test'").as_bool().unwrap());
        if js("typeof htmlWindow.chart.getOption().series[0].renderItem === 'function'")
            .as_bool()
            .unwrap()
        {
            assert!(
                js("htmlWindow.explicitCalls>0 && !htmlWindow.payloadExecuted")
                    .as_bool()
                    .unwrap()
            );
        }
        expose(&js("htmlWindow.chart"));
        foreground().await;
        assert_eq!(
            js("JSON.stringify(htmlWindow.pageErrors)")
                .as_string()
                .as_deref(),
            Some("[]")
        );
        js("htmlWindow.chart.dispose();document.getElementById('html-frame').remove();");
    }
    // Registered names and ordinary payload strings are data; only JsFunction executes.
    WasmRenderer::register_extension(
        "globalThis.explicitCalls=0;globalThis.payloadExecuted=false;",
    )
    .unwrap();
    let chart = explicit_custom();
    render(&chart);
    foreground().await;
    assert!(js("explicitCalls>0 && !payloadExecuted").as_bool().unwrap());
    dispose();
}

#[wasm_bindgen_test(async)]
async fn gl_bar3d_produces_software_webgl_foreground() {
    setup();
    WasmRenderer::register_extension(include_str!("echarts-gl-2.1.0.min.js")).unwrap();
    render(&bar3d::chart().animation(false));
    promise("new Promise(resolve=>setTimeout(resolve,1500))").await;
    assert!(js("Array.from(document.querySelectorAll('#chart canvas')).some(c=>c.getContext('webgl2')||c.getContext('webgl'))").as_bool().unwrap());
    foreground().await;
    console_log!(
        "CHARMING_GL_PNG:{}",
        js("instance.getDataURL({type:'png'})").as_string().unwrap()
    );
    console_log!("GL renderer: software WebGL (ANGLE SwiftShader)");
    dispose();
}

fn explicit_custom() -> Chart {
    Chart::new().animation(false).series(Custom::new().coordinate_system(CoordinateSystem::None)
        .item_payload(serde_json::json!({"source":"globalThis.payloadExecuted=true"}).as_object().unwrap().clone())
        .render_item(JsFunction::from("function(p){explicitCalls++;return {type:'circle',shape:{cx:200,cy:200,r:60},style:{fill:'#5070dd'}}}"))
        .data(vec![1]))
}

fn time_chart() -> Chart {
    use charming::element::{AreaStyle, AxisLabel, Formatter, TimeFormatterEntry, TimeUnit};
    Chart::new()
        .animation(false)
        .x_axis(
            Axis::new()
                .type_(AxisType::Time)
                .axis_label(AxisLabel::new().formatter(Formatter::from(
                    std::collections::BTreeMap::from([(
                        TimeUnit::Day,
                        TimeFormatterEntry::from("{MMM} {dd}"),
                    )]),
                ))),
        )
        .y_axis(Axis::new())
        .series(Line::new().area_style(AreaStyle::new()).data(vec![
            vec![1793404800000_f64, 2.0],
            vec![1793491200000_f64, 5.0],
            vec![1793577600000_f64, 3.0],
        ]))
}
