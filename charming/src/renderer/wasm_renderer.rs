use crate::{Chart, EchartsError, element::Easing, theme::Theme};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

pub struct WasmRenderer {
    theme: Theme,
    width: Option<u32>,
    height: Option<u32>,
}

impl WasmRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            theme: Theme::Default,
            width: Some(width),
            height: Some(height),
        }
    }

    pub fn new_opt(width: Option<u32>, height: Option<u32>) -> Self {
        Self {
            theme: Theme::Default,
            width,
            height,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn render(&self, id: &str, chart: &Chart) -> Result<Echarts, EchartsError> {
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
        let echarts = init(
            &element,
            self.theme.to_str().0,
            to_value(&ChartSize {
                width: self.width,
                height: self.height,
            })
            .unwrap(),
        );
        Self::update(&echarts, chart);

        Ok(echarts)
    }

    /// Resizes a chart with options specified in [`ChartResize`]
    pub fn resize_chart(echarts: &Echarts, chart_size: ChartResize) {
        echarts
            .resize(to_value(&chart_size).expect("could not convert resize options to `JsValue`"));
    }

    pub fn update(echarts: &Echarts, chart: &Chart) {
        let js = serde_wasm_bindgen::to_value(&chart).unwrap();
        echarts.set_option(js);
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

#[derive(Clone, Debug, Serialize, Copy)]
struct ChartSize {
    width: Option<u32>,
    height: Option<u32>,
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

    #[wasm_bindgen(js_namespace = echarts, js_name = init)]
    fn init(id: &web_sys::Element, theme: &str, size: JsValue) -> Echarts;

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_option(this: &Echarts, option: JsValue);

    #[wasm_bindgen(method, js_name = "resize")]
    pub fn resize(this: &Echarts, opts: JsValue);

    #[wasm_bindgen(method, js_name = "dispose")]
    pub fn dispose(this: &Echarts);
}

// Enable dispose on on_cleanup function in Leptos
unsafe impl Sync for Echarts {}
unsafe impl Send for Echarts {}
