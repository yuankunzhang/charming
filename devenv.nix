{
  pkgs,
  lib,
  ...
}:
let
  my-packages-lib = with pkgs; [
    pkg-config
    gobject-introspection
    glib-networking
    xdotool
    udev
  ];
  my-packages = with pkgs; [
    dioxus-cli
    wasmBindgenCli
    wasm-pack
    trunk
    leptosfmt
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];
  # dx (built with nixpkgs' no-downloads feature) and trunk both refuse to build
  # unless the wasm-bindgen CLI on PATH exactly matches the wasm-bindgen crate in
  # each example's Cargo.lock (enterShell warns when they drift). Bump together.
  wasmBindgenCli = pkgs.wasm-bindgen-cli_0_2_127;

  # The `v8` crate's build script downloads this prebuilt static library with
  # python or curl unless RUSTY_V8_ARCHIVE points at a local file. Fetching it
  # here keeps the download pinned and out of cargo's hands. The build script
  # does not verify the archive matches the crate, so the version must track
  # the `v8` entry in Cargo.lock (enterShell warns when it drifts).
  rustyV8Version = "150.4.0";
  rustyV8Hashes = {
    aarch64-linux = "09x5ck4fh3i90dfx1apambmmif0pwm18p19jdxwsb5m32lw2i7jk";
    x86_64-linux = "0xml6268gjs2pj7xf50gx3ly337c63j5l724b9hgrwfi235651zl";
  };
  librustyV8 = pkgs.fetchurl {
    url = "https://github.com/denoland/rusty_v8/releases/download/v${rustyV8Version}/librusty_v8_simdutf_release_${pkgs.stdenv.hostPlatform.rust.rustcTarget}.a.gz";
    sha256 = rustyV8Hashes.${pkgs.stdenv.hostPlatform.system};
  };
in
{
  packages = my-packages-lib ++ my-packages;

  env.RUSTY_V8_ARCHIVE = "${librustyV8}";

  enterShell = ''
    lockedV8=$(grep -A1 '^name = "v8"$' "$DEVENV_ROOT/Cargo.lock" | sed -n 's/^version = "\(.*\)"$/\1/p')
    if [ "$lockedV8" != "${rustyV8Version}" ]; then
      echo "warning: Cargo.lock has v8 $lockedV8 but devenv.nix pins librusty_v8 ${rustyV8Version}; update rustyV8Version and its hashes"
    fi

    for lock in "$DEVENV_ROOT"/examples/*/Cargo.lock; do
      lockedWb=$(grep -A1 '^name = "wasm-bindgen"$' "$lock" | sed -n 's/^version = "\(.*\)"$/\1/p')
      if [ -n "$lockedWb" ] && [ "$lockedWb" != "${wasmBindgenCli.version}" ]; then
        echo "warning: $lock pins wasm-bindgen $lockedWb but devenv.nix provides wasm-bindgen-cli ${wasmBindgenCli.version}; dx and trunk need an exact match"
      fi
    done

    export GIO_MODULE_DIR=${pkgs.glib-networking.out}/lib/gio/modules/
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath my-packages-lib}"
    export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS;
  '';

  enterTest = ''
    # Building and testing
    cargo build --verbose
    cargo run --bin generate_images img_test
    cargo test --verbose

    # Linting and formatting
    cargo fmt --check
    cargo clippy --all-targets --all-features

    # Build dioxus desktop example
    cd $DEVENV_ROOT/examples/dioxus-desktop-demo
    dx build

    # Build wasm examples 
    cd $DEVENV_ROOT/examples/dioxus-web-demo
    dx build
    cd $DEVENV_ROOT/examples/leptos-demo
    trunk build
    cd $DEVENV_ROOT/examples/sycamore-demo
    trunk build
    cd $DEVENV_ROOT/examples/yew-demo
    trunk build
  '';

  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.98.1";
    targets = [ "wasm32-unknown-unknown" ];
  };

  git-hooks.hooks = {
    nixfmt-rfc-style.enable = true;
    taplo.enable = true;
    rustfmt.enable = true;
    clippy = {
      enable = true;
      settings = {
        allFeatures = true;
        offline = false;
        denyWarnings = true;
        extraArgs = "--all-targets";
      };
    };
  };
}
