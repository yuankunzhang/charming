# leptos-demo

## run charming in wasm by leptos framework

* copy
  we use "gallery/src/bar" files as example,cp what you want in "gallery/src "
```sh
cp ./gallery/src/bar ./examples/leptos-demo/src
```
* install wasm runtime 
more,please check leptos 
```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
```
* run
```sh
cd examples/leptos-demo
trunk serve
```