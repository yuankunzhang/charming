// main.rs
mod chart;
mod dataset;
mod renderer;

fn main() {
    let s = renderer::render_string().unwrap();
    println!("{}", s);
}
