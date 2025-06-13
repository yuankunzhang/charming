use charming::{
    component::Axis,
    element::{AxisType, Tooltip},
    series::Line,
    Chart,
};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    info!("starting app");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let chart = Chart::new()
        .tooltip(Tooltip::new().formatter("Value: {c}"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));
    let chart_json = chart.to_string();
    let mount_code = format!(
        r#"
            var millis = 100;
            setTimeout(function() {{
                var chart = echarts.init(document.getElementById('chart'), null, {{renderer: 'canvas'}});
                window.addEventListener('resize', function() {{
                    chart.resize();
                }});
                chart.setOption({chart_json});
            }}, millis)
        "#
    );
    rsx! {
        document::Script { src: asset!("/assets/echarts-5.5.1.min.js") }
        div { style: "text-align: center;",
            h1 { "🌗 Dioxus + Charming 🚀" }
            h3 { "Frontend that scales." }
            p {
                "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust."
            }
        }
        div { style: "width: 100%; text-align: center;",
            div { id: "chart", style: "display: inline-block; width: 600px; height: 400px;", onmounted: move |_| {
                document::eval(&mount_code);
            },}
        }
    }
}
