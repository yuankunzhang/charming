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
    wasm-bindgen-cli
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
in
{
  packages = my-packages-lib ++ my-packages;

  enterShell = ''
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
