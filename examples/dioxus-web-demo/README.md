# charming with dioxus-web

## Prerequisites

Before you start working on this project, make sure you have the following prerequisites installed:

- [Rust](https://www.rust-lang.org/)
- [Dioxus CLI](https://dioxus.org/)
- Rust target for WebAssembly: `wasm32-unknown-unknown`

```sh
cargo install dioxus-cli --force
rustup target add wasm32-unknown-unknown
```

## Project Structure

The project follows the following directory structure:

```
dioxus-web-demo/
  ├── src/
  │    └─ main.rs
  ├── ...
  └── dioxus.toml
```

- `src/main.rs`: This directory contains the main Rust files for your front-end application.
- `dioxus.toml`: This is a file used to write configurations required for building dioxus-web, specifically to declare elements outside the <div id="main"></div> tag. Here's an example of what it may contain:

```toml
...
script = [
    "https://cdn.jsdelivr.net/npm/echarts@5.4.3/dist/echarts.min.js",
    "https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js"
]
...
```

## Getting started

```sh
dioxus serve
```

<http://localhost:8080/>