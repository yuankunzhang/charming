# Engineering checks

Read [CONTRIBUTING.md](CONTRIBUTING.md) before changing build tooling,
compatibility contracts or test coverage; it defines task coverage, CI selection
and the MSRV policy.

- Use `python3 scripts/check.py` for checks. Keep product verification in
  applicable CI; locally run the documented control-logic tests, actionlint and
  diff check when changing the runner or workflow. Report pending CI separately.
- Read tool versions from `rust-toolchain.toml` and `ci/tools.toml`. A WASM
  upgrade must synchronize all six Cargo locks and the native/Nix CLI, including
  the Nix source and vendor hashes. Keep Dioxus crate/CLI and V8 crate/archive
  versions aligned.
- Preserve public Rust API and serialized JSON compatibility unless the task
  explicitly changes them. Preserve the existing SSR snapshot assertions and
  baselines; document the separate browser, raster and feature-matrix limits.
