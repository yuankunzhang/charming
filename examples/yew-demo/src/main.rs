use yew::prelude::*;
use charming::{component::{Axis, Title}, element::AxisType, series::Line, Chart, WasmRenderer};

#[function_component]
fn App() -> Html {
    let f = yew_hooks::use_async::<_, _, ()>({
        let chart = Chart::new()
            .title(Title::new().text("Demo: Yew + Charming"))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

        let renderer = WasmRenderer::new(600, 400);
        
        async move {
            renderer.render("chart", &chart).unwrap();
            Ok(())
        }
    });
    
    use_effect_with_deps(move |_| {
        f.run();
        || ()
    }, ());

    html! {
        <div id="chart"></div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
