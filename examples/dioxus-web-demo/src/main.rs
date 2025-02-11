use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use charming::{
    component::Axis,
    element::{formatter::FormatterFunction, AxisType, Tooltip},
    series::Line,
    Chart, WasmRenderer,
};

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    info!("starting app");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let renderer = use_signal(|| WasmRenderer::new(600, 400));
    use_effect(move || {
        let chart = Chart::new()
            .tooltip(Tooltip::new().formatter(FormatterFunction::new_with_args(
                "params",
                r#"
                    var tooltip = "Value: ".concat(String(params.value));
                    return tooltip;
                "#,
            )))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

        renderer.read_unchecked().render("chart", &chart).unwrap();
    });

    rsx! (
        div { style: "text-align: center;",
            h1 { "🌗 Dioxus + Charming 🚀" }
            h3 { "Frontend that scales." }
            p {
                "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust."
            }
        }
        div { style: "width: 100%; text-align: center;",
            div { id: "chart", style: "display: inline-block;" }
        }
    )
}
