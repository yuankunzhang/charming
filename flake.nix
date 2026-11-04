{
  description = "Charming development environment";

  inputs = {
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      git-hooks,
      ...
    }:
    let
      tools = builtins.fromTOML (builtins.readFile ./ci/tools.toml);
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      devShells = nixpkgs.lib.genAttrs systems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          inherit (pkgs) lib;
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          wasmBindgenCli = import ./nix/wasm-bindgen-cli.nix {
            inherit pkgs;
            version = tools.wasm-bindgen;
          };
          libraryPackages = with pkgs; [
            pkg-config
            gobject-introspection
            glib-networking
            xdotool
            udev
          ];
          packages = with pkgs; [
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
            rustToolchain
            python3
            nixfmt
            taplo
          ];
          rustyV8Hashes = {
            x86_64-linux = "0xml6268gjs2pj7xf50gx3ly337c63j5l724b9hgrwfi235651zl";
            aarch64-linux = "09x5ck4fh3i90dfx1apambmmif0pwm18p19jdxwsb5m32lw2i7jk";
          };
          target = pkgs.stdenv.hostPlatform.rust.rustcTarget;
          librustyV8 = pkgs.fetchurl {
            name = "librusty_v8-${tools.v8}-${target}.a.gz";
            url = "https://github.com/denoland/rusty_v8/releases/download/v${tools.v8}/librusty_v8_simdutf_release_${target}.a.gz";
            sha256 = rustyV8Hashes.${system};
          };
          hooks = git-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              nixfmt-rfc-style.enable = true;
              taplo.enable = true;
              rustfmt = {
                enable = true;
                package = rustToolchain;
              };
              clippy = {
                enable = true;
                package = rustToolchain;
                entry = lib.mkForce "${pkgs.python3}/bin/python3 scripts/check.py run clippy";
                pass_filenames = false;
              };
            };
          };
        in
        {
          default =
            assert pkgs.dioxus-cli.version == tools.dioxus-cli;
            assert pkgs.trunk.version == tools.trunk;
            pkgs.mkShell {
              packages = libraryPackages ++ packages ++ hooks.enabledPackages;
              RUSTY_V8_ARCHIVE = "${librustyV8}";
              shellHook = ''
                ${hooks.shellHook}
                export PATH="${wasmBindgenCli}/bin:$PATH"
                export GIO_MODULE_DIR=${pkgs.glib-networking.out}/lib/gio/modules/
                export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath libraryPackages}"
                export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
              '';
            };
        }
      );
    };
}
