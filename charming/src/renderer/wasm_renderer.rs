use crate::{theme::Theme, Chart, EchartsError};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub struct WasmRenderer {
    theme: Theme,
    width: u32,
    height: u32,
}

impl WasmRenderer {
    pub fn new(width: u32, height: u32) -> Self {
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

    pub fn render(&self, id: &str, chart: &Chart) -> Result<(), EchartsError> {
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
        Ok(())
    }
}

#[derive(Serialize)]
struct ChartSize {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = echarts)]
    type Echarts;

    #[wasm_bindgen(js_namespace = echarts, js_name = init)]
    fn init(id: &web_sys::Element, theme: &str, size: JsValue) -> Echarts;

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_option(this: &Echarts, option: JsValue);
}
