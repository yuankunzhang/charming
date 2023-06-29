// main.rs
mod chart;
mod dataset;
mod renderer;
mod style;

fn main() {
    let s = renderer::render_string().unwrap();
    println!("{}", s);
}
