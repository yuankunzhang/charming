use wasm_bindgen::prelude::*;

use crate::Chart;

#[wasm_bindgen]
extern "C" {
    type Echarts;

    #[wasm_bindgen(constructor)]
    fn init(id: &str, theme: &str, width: u32, height: u32) -> Echarts;

    #[wasm_bindgen(method, js_name = "setOption")]
    fn set_option(this: &Echarts, option: &str);
}

pub async fn render(id: &str, chart: &Chart) {
    let echarts = Echarts::init(id, "default", 1000, 800);
    echarts.set_option(chart.to_string().as_str());
}
