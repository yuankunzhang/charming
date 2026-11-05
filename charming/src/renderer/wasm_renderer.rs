use crate::{
    Chart, EchartsError,
    component::SaveAsImageType,
    element::Easing,
    render_option::{RenderOption, wasm_error},
    theme::Theme,
};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

pub struct WasmRenderer {
    theme: Theme,
    width: Option<u32>,
    height: Option<u32>,
    locale: Option<String>,
}

impl WasmRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            theme: Theme::Default,
            locale: None,
            width: Some(width),
            height: Some(height),
        }
    }

    pub fn new_opt(width: Option<u32>, height: Option<u32>) -> Self {
        Self {
            theme: Theme::Default,
            locale: None,
            width,
            height,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Set the locale passed to ECharts initialization.
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Execute trusted registration source in the global ECharts environment.
    pub fn register_extension(source: &str) -> Result<(), EchartsError> {
        let source = serde_json::to_string(source).expect("a string serializes to JSON");
        js_sys::eval(&format!("(0, eval)({source})"))
            .map(|_| ())
            .map_err(wasm_error)
    }

    pub fn render(&self, id: &str, chart: &Chart) -> Result<Echarts, EchartsError> {
        let option = prepare(chart)?;
        let (theme, source) = self.theme.to_str();
        if !source.is_empty() {
            Self::register_extension(source)?;
        }
        let window = web_sys::window().ok_or(EchartsError::WasmError(
            "no `window` object found".to_string(),
        ))?;
        let document = window.document().ok_or(EchartsError::WasmError(
            "no `document` object found".to_string(),
        ))?;
        let element = document
            .get_element_by_id(id)
            .ok_or(EchartsError::WasmError(format!(
                "no element with id `{id}` found",
            )))?;
        let is_new = get_instance_by_dom(&element)
            .map_err(wasm_error)?
            .is_undefined();
        let echarts = init(
            &element,
            theme,
            to_value(&ChartInit {
                width: self.width,
                height: self.height,
                locale: self.locale.as_deref(),
                renderer: match chart.save_as_image_type() {
                    Some(SaveAsImageType::Svg) => "svg",
                    _ => "canvas",
                },
            })
            .map_err(|error| EchartsError::WasmError(error.to_string()))?,
        )
        .map_err(wasm_error)?;
        if let Err(error) = echarts.set_option(option) {
            if is_new {
                let _ = echarts.dispose();
            }
            return Err(wasm_error(error));
        }

        Ok(echarts)
    }

    /// Resizes a chart with options specified in [`ChartResize`]
    pub fn resize_chart(echarts: &Echarts, chart_size: ChartResize) {
        echarts
            .resize(to_value(&chart_size).expect("could not convert resize options to `JsValue`"));
    }

    /// Prepare all source before applying options. Synchronous failures return `WasmError`.
    /// Existing instances are retained on failure; updates do not promise rollback.
    /// Later interaction callback errors are handled by the JavaScript runtime.
    pub fn update(echarts: &Echarts, chart: &Chart) -> Result<(), EchartsError> {
        echarts.set_option(prepare(chart)?).map_err(wasm_error)
    }

    /// Replace all options. Call this with the complete current chart before changing theme.
    pub fn replace(echarts: &Echarts, chart: &Chart) -> Result<(), EchartsError> {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"notMerge".into(), &true.into()).map_err(wasm_error)?;
        echarts
            .set_option_with_options(prepare(chart)?, options.into())
            .map_err(wasm_error)
    }

    /// Change theme after [`Self::replace`]; merge-only updates may not survive a theme change.
    pub fn set_theme(echarts: &Echarts, theme: Theme, silent: bool) -> Result<(), EchartsError> {
        let (name, source) = theme.to_str();
        if !source.is_empty() {
            Self::register_extension(source)?;
        }
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"silent".into(), &silent.into()).map_err(wasm_error)?;
        echarts
            .change_theme(
                if name.is_empty() { "default" } else { name },
                options.into(),
            )
            .map_err(wasm_error)
    }

    pub fn convert_to_layout(
        echarts: &Echarts,
        finder: JsValue,
        value: JsValue,
        options: Option<JsValue>,
    ) -> Result<JsValue, EchartsError> {
        echarts
            .to_layout(finder, value, options.unwrap_or(JsValue::UNDEFINED))
            .map_err(wasm_error)
    }

    pub fn convert_to_pixel(
        echarts: &Echarts,
        finder: JsValue,
        value: JsValue,
        options: Option<JsValue>,
    ) -> Result<JsValue, EchartsError> {
        echarts
            .to_pixel(finder, value, options.unwrap_or(JsValue::UNDEFINED))
            .map_err(wasm_error)
    }

    pub fn convert_from_pixel(
        echarts: &Echarts,
        finder: JsValue,
        value: JsValue,
        options: Option<JsValue>,
    ) -> Result<JsValue, EchartsError> {
        echarts
            .from_pixel(finder, value, options.unwrap_or(JsValue::UNDEFINED))
            .map_err(wasm_error)
    }

    pub fn dispatch_action(echarts: &Echarts, payload: JsValue) -> Result<(), EchartsError> {
        echarts.action(payload).map_err(wasm_error)
    }

    /// The caller must retain the function (and its Rust Closure) until it is unbound.
    pub fn on(
        echarts: &Echarts,
        event: &str,
        handler: &js_sys::Function,
    ) -> Result<(), EchartsError> {
        echarts.bind_event(event, handler).map_err(wasm_error)
    }

    /// Unbind the same function previously passed to [`Self::on`].
    pub fn off(
        echarts: &Echarts,
        event: &str,
        handler: &js_sys::Function,
    ) -> Result<(), EchartsError> {
        echarts.unbind_event(event, handler).map_err(wasm_error)
    }
    /// Enable disposing of chart instance
    /// # Example
    /// ```no_run
    /// use charming::renderer::WasmRenderer;
    /// use charming::Chart;
    /// // assume `chart` is an instance of `Echarts`
    /// # let chart = WasmRenderer::new(800,600).render("chart-id",&Chart::new()).unwrap();
    /// WasmRenderer::dispose_chart(&chart);
    /// ```
    /// # Leptos Integration Example
    ///
    /// ```no_run
    /// use charming::renderer::WasmRenderer;
    /// use charming::Chart;
    /// use leptos::prelude::*;
    /// use std::sync::{Arc, Mutex};
    /// #[component]
    /// fn MyChartComponent(chart: Box<dyn Fn()-> Chart>) -> impl IntoView {
    ///     let e_charts: Arc<Mutex<Option<Echarts>>> = Arc::new(Mutex::new(None));
    ///     let chart_clone = e_charts.clone();
    ///     on_cleanup(move || {
    ///         if let Some(chart) = chart_clone.lock().unwrap().as_ref() {
    ///             // In sometimes, you might want to dispose of the chart to free up resources,specially when the chart using custom renderItem.
    ///            WasmRenderer::dispose_chart(chart);
    ///         }
    ///     });
    ///     Effect::new(
    ///         move |_| {
    ///             let render = WasmRenderer::new_opt(None,None);
    ///            *e_charts.lock().unwrap()= render.render("my-chart", &chart()).ok();
    ///         }
    ///     );
    ///     view! {
    ///         <div id="my-chart" class="w-full h-full"></div>
    ///     }
    /// }
    /// ```
    pub fn dispose_chart(echarts: &Echarts) {
        echarts.dispose();
    }
}

fn prepare(chart: &Chart) -> Result<JsValue, EchartsError> {
    RenderOption::new(chart)
        .map_err(|error| EchartsError::WasmError(error.to_string()))?
        .to_wasm()
}

#[derive(Clone, Debug, Serialize, Copy)]
struct ChartInit<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<&'a str>,
    renderer: &'a str,
}

#[derive(Clone, Debug, Serialize, Copy)]
pub struct ChartResize {
    /// New width in px
    pub width: u32,
    /// New height in px
    pub height: u32,
    /// If true, emits events on resize
    pub silent: bool,
    /// Resize animation options
    pub animation: Option<Animation>,
}

impl ChartResize {
    pub fn new(width: u32, height: u32, silent: bool, animation: Option<Animation>) -> Self {
        Self {
            width,
            height,
            silent,
            animation,
        }
    }
}

#[derive(Clone, Debug, Serialize, Copy)]
pub struct Animation {
    /// duration of the animation
    pub duration: u32,
    /// easing function used for the animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub easing: Option<Easing>,
}

/// available easing functions in echarts
impl Default for Animation {
    fn default() -> Self {
        Self {
            duration: 0,
            easing: Some(Easing::default()),
        }
    }
}

impl Animation {
    pub fn new(duration: u32, easing: Option<Easing>) -> Self {
        Self {
            duration,
            easing: easing.or_else(|| Some(Easing::default())),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = echarts)]
    pub type Echarts;

    #[wasm_bindgen(catch, js_namespace = echarts, js_name = init)]
    fn init(id: &web_sys::Element, theme: &str, size: JsValue) -> Result<Echarts, JsValue>;

    #[wasm_bindgen(catch, js_namespace = echarts, js_name = getInstanceByDom)]
    fn get_instance_by_dom(element: &web_sys::Element) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = "setOption")]
    fn set_option(this: &Echarts, option: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = setOption)]
    fn set_option_with_options(
        this: &Echarts,
        option: JsValue,
        options: JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = setTheme)]
    fn change_theme(this: &Echarts, name: &str, options: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = convertToLayout)]
    fn to_layout(
        this: &Echarts,
        finder: JsValue,
        value: JsValue,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = convertToPixel)]
    fn to_pixel(
        this: &Echarts,
        finder: JsValue,
        value: JsValue,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = convertFromPixel)]
    fn from_pixel(
        this: &Echarts,
        finder: JsValue,
        value: JsValue,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = dispatchAction)]
    fn action(this: &Echarts, payload: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = on)]
    fn bind_event(this: &Echarts, event: &str, handler: &js_sys::Function) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = off)]
    fn unbind_event(this: &Echarts, event: &str, handler: &js_sys::Function)
    -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    fn dispose(this: &Echarts) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "resize")]
    pub fn resize(this: &Echarts, opts: JsValue);

    #[wasm_bindgen(method, js_name = "dispose")]
    pub fn dispose(this: &Echarts);
}

// Enable dispose on on_cleanup function in Leptos
unsafe impl Sync for Echarts {}
unsafe impl Send for Echarts {}
