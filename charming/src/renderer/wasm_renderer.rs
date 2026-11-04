use crate::{
    Chart, EchartsError,
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
        let option = prepare(chart)?;
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
            self.theme.to_str().0,
            to_value(&ChartSize {
                width: self.width,
                height: self.height,
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
}

fn prepare(chart: &Chart) -> Result<JsValue, EchartsError> {
    RenderOption::new(chart)
        .map_err(|error| EchartsError::WasmError(error.to_string()))?
        .to_wasm()
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

    #[wasm_bindgen(catch, js_namespace = echarts, js_name = init)]
    fn init(id: &web_sys::Element, theme: &str, size: JsValue) -> Result<Echarts, JsValue>;

    #[wasm_bindgen(catch, js_namespace = echarts, js_name = getInstanceByDom)]
    fn get_instance_by_dom(element: &web_sys::Element) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = "setOption")]
    fn set_option(this: &Echarts, option: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    fn dispose(this: &Echarts) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "resize")]
    pub fn resize(this: &Echarts, opts: JsValue);
}
