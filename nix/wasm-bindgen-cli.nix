{ pkgs, version }:
let
  src = pkgs.fetchCrate {
    pname = "wasm-bindgen-cli";
    inherit version;
    hash = "sha256-pcecKQd7E8Opw6bkFoE569epUi7gh5qpQF1e5PJY6V8=";
  };
in
pkgs.buildWasmBindgenCli {
  inherit src;
  cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
    inherit src;
    inherit (src) pname version;
    hash = "sha256-vmUrWVU7kPJJxO5qIVeAkwQyWDELO1Z4Z5gitz2kco8=";
  };
}
