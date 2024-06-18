use crate::{theme::Theme, Chart, EchartsError};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

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
                "no element with id `{}` found",
                id
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
        echarts.set_option(to_value(chart).unwrap());

        Ok(echarts)
    }

    /// Resizes a chart with options specified in [`ChartResize`]
    pub fn resize_chart(echarts: &Echarts, chart_size: ChartResize) {
        echarts
            .resize(to_value(&chart_size).expect("could not convert resize options to `JsValue`"));
    }

    pub fn update(echarts: &Echarts, chart: &Chart) {
        echarts.set_option(to_value(chart).unwrap());
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
#[derive(Clone, Debug, Default, Serialize, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Easing {
    #[default]
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    QuarticInOut,
    QuinticIn,
    QuinticOut,
    QuinticInOut,
    SinusoidalIn,
    SinusoidalOut,
    SinusoidalInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    CircularIn,
    CircularOut,
    CircularInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            duration: 100,
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
}
