use dioxus::prelude::*;
use log::LevelFilter;

use charming::{component::{Axis, Title}, element::AxisType, series::Line, Chart, WasmRenderer};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {

    let renderer: WasmRenderer = WasmRenderer::new(600, 400);
    use_future!(cx, || async move {
      let chart = Chart::new()
      .x_axis(
          Axis::new()
              .type_(AxisType::Category)
              .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
      )
      .y_axis(Axis::new().type_(AxisType::Value))
      .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

      renderer.render("chart",&chart).unwrap();
    });


    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "🌗 Dioxus + Charming 🚀" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
        div {
          style: "width: 100%; text-align: center;",
          div { 
            id: "chart",
            style: "display: inline-block;",
          }
        }
    ))
}
