use charming::{Chart, WasmRenderer};
use leptos::*;

mod bar;

use bar::set_style_of_single_bar::chart;
// use bar::world_population::chart;
// use bar::radial_polar_bar_label_position::chart;

#[component]
pub fn App() -> impl IntoView {
    let action = create_action(|_input: &()| async {
        let chart: Chart = chart(); //just change your chart
        WasmRenderer::new(600, 400).render("chart", &chart).unwrap();
    });
    // runtime.dispose();
    view! {
        <div>
            <button on:click=move |_| {action.dispatch(());}>"Show chart"</button>
            <div  id="chart"></div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
