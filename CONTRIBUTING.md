# Contributing

## Environment and checks

Use Python 3.11 or newer for the shared entry point. Rust comes from
[rust-toolchain.toml](rust-toolchain.toml); other expected tool versions are in
[ci/tools.toml](ci/tools.toml).

On Linux, enter the development environment with `nix develop`. The flake
provides Rust, the WASM target, example CLIs, desktop libraries, a pinned V8
archive, Python and formatters. It installs Nixfmt, Taplo, Rustfmt and Clippy
Git hooks; formatting hooks can edit files, while Clippy uses the shared runner.
Entering the shell does not build or test Charming. The x86_64 Linux environment
is the CI target; the ARM Linux output is retained but has not been validated.

For automatic entry, install direnv with nix-direnv, then run `direnv allow`
after pulling the `.envrc` migration. Direct `nix develop` needs neither.
Existing ignored local caches and user configuration can stay in place.

Run tasks from any working directory:

```sh
python3 scripts/check.py run preflight
python3 scripts/check.py run native
python3 scripts/check.py run fmt
python3 scripts/check.py run clippy
python3 scripts/check.py run msrv
python3 scripts/check.py run stable
python3 scripts/check.py run example yew-demo
python3 scripts/check.py run all
python3 scripts/check.py run nix-smoke
python3 scripts/check.py run nix-full
```

Use the repository path to `scripts/check.py` when outside its root. `all` runs
preflight, native, format, Clippy, MSRV and all five examples. `stable` separately
checks core without default features, default HTML, and WASM on the latest
stable toolchain. `nix-smoke` checks tools and environment without a product
build; `nix-full` runs environment, all and Nix/TOML formatting checks in one
shell. Cargo/Dioxus use `--locked`, and Nix uses `--no-update-lock-file`.
Trunk's metadata step can still edit its example lock: the runner checks that
lock before and after the build, including failed builds, and leaves any change
available for inspection.

The separate `stable` task requires rustup on PATH and an up-to-date stable
installation with the WASM target:

```sh
rustup toolchain install stable --profile minimal --target wasm32-unknown-unknown
```

It invokes Cargo through `rustup run stable` and selects that toolchain's
compiler explicitly, including inside `nix develop`. The flake itself provides
the fixed project toolchain; `all` and `nix-full` use that fixed toolchain.

## CI selection and local verification

The Rust workflow runs on main pull requests, main pushes and manual dispatch.
Preflight and Nix smoke always run. Shared build inputs (manifests, locks,
toolchains, runner, tools, workflows and Nix configuration) select all jobs and
Nix full. Library code selects native, lint, MSRV, stable and all examples;
gallery/snapshot changes select native and lint. Example changes select that
example. Documentation selects preflight only, alongside Nix smoke. Rules form
a union; unknown paths, unavailable comparisons and manual dispatch select all.
PR selection uses the merge base, while jobs test GitHub's merge checkout.
`ci-gate` requires every selected job to succeed. A pending, cancelled, missing
or unexpectedly skipped job does not pass.

Leave product builds and Nix checks to applicable CI. The local exception for
changes to CI control logic is:

```sh
python3 -B -m unittest discover -s scripts -p test_check.py
actionlint -shellcheck='' .github/workflows/rust.yml
git diff --check
```

Report local results and pending CI separately. A push, workflow dispatch or
other remote action requires separate authorization.

## Compatibility and coverage

MSRV means the minimum Rust version supported for downstream library compilation.
The library manifest declares it, and the fixed toolchain must agree. Raise it
when dependencies or supported behavior require it, normally allowing two stable
release cycles of notice; lowering it requires separate verification. Full
validation uses the fixed toolchain, with relevant changes also compiled on
latest stable.

Preserve Rust API and serialized JSON compatibility when changing chart models.
Explain intentional compatibility changes in the pull request. The native task
builds the workspace, generates `img_test`, then runs existing tests, including
87 SSR (server-side rendering) image snapshots. Keep their baselines and
assertions meaningful. Native `--all-features` linting does not establish WASM
target support or a complete feature matrix. Browser execution and new raster
runtime tests are outside this CI migration.

When upgrading WASM tools, synchronize wasm-bindgen and its macro crates,
js-sys/web-sys and wasm-bindgen-futures (where present) across all six Cargo
locks, the CLI in `ci/tools.toml`, and its Nix source/vendor hashes. Keep the
Dioxus CLI aligned with the examples' Dioxus crate and the V8 archive aligned
with the locked V8 crate. Preflight and `run env` check these couplings.
