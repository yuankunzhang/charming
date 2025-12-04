use crate::{element::Easing, theme::Theme, Chart, EchartsError};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};
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
        self.render_with_opts(id, chart, RenderOpts::default())
    }

    pub fn render_with_opts(
        &self,
        id: &str,
        chart: &Chart,
        opts: RenderOpts,
    ) -> Result<Echarts, EchartsError> {
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
        Self::update_with_opts(&echarts, chart, opts);

        Ok(echarts)
    }

    /// Resizes a chart with options specified in [`ChartResize`]
    pub fn resize_chart(echarts: &Echarts, chart_size: ChartResize) {
        echarts
            .resize(to_value(&chart_size).expect("could not convert resize options to `JsValue`"));
    }

    pub fn update(echarts: &Echarts, chart: &Chart) {
        Self::update_with_opts(echarts, chart, RenderOpts::default())
    }

    pub fn update_with_opts(echarts: &Echarts, chart: &Chart, opts: RenderOpts) {
        let js = serde_wasm_bindgen::to_value(&chart).unwrap();
        let opts_js = to_value(&opts).expect("could not convert RenderOpts to `JsValue`");
        echarts.set_option(js, opts_js);
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

/// Optional parameters for rendering and updating charts
#[serde_with::apply(
    Option => #[serde(skip_serializing_if = "Option::is_none")],
    Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, PartialOrd, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RenderOpts {
    /// Whether to not merge with previously set option.
    /// If true, all current components will be removed and new components created.
    /// Default: false (merge with previous option)
    not_merge: Option<bool>,

    /// Whether to not update the chart immediately.
    /// Default: false (update immediately)
    lazy_update: Option<bool>,

    /// Whether to prevent events from being thrown when calling setOption.
    /// Default: false (events are thrown)
    silent: Option<bool>,

    /// Specify component main types that will be replaced instead of merged.
    /// For example: ["xAxis", "yAxis", "series"]
    replace_merge: Option<Vec<String>>,
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = echarts)]
    pub type Echarts;

    #[wasm_bindgen(js_namespace = echarts, js_name = init)]
    fn init(id: &web_sys::Element, theme: &str, size: JsValue) -> Echarts;

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_option(this: &Echarts, option: JsValue, opts: JsValue);

    #[wasm_bindgen(method, js_name = "resize")]
    pub fn resize(this: &Echarts, opts: JsValue);
}
