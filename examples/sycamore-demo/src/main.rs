use charming::{
    component::{Axis, Title},
    element::AxisType,
    series::Line,
    Chart,
    WasmRenderer,
};
use sycamore::prelude::*;

#[component]
fn App() -> View {
    let data = create_signal(vec![150, 230, 224, 218, 135, 147, 260]);
    let renderer = WasmRenderer::new(600, 400);
    
    on_mount(move || {
        create_effect(move || {
            let chart = Chart::new()
                .title(Title::new().text("Demo: Sycamore + Charming"))
                .x_axis(
                    Axis::new()
                        .type_(AxisType::Category)
                        .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
                )
                .y_axis(Axis::new().type_(AxisType::Value))
                .series(Line::new().data(data.get_clone()));
            renderer.render("chart", &chart).unwrap();
        });
    });
    
    view! {
        div(id="chart")
    }
}

fn main() {
    sycamore::render(App);
}