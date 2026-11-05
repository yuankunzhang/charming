#!/usr/bin/env python3
"""Shared local, GitHub Actions and Nix checks (Python 3.11+)."""

import argparse
import base64
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parent.parent
EXAMPLES = (
    "dioxus-desktop-demo",
    "dioxus-web-demo",
    "leptos-demo",
    "sycamore-demo",
    "yew-demo",
)
FLAGS = ("native", "lint", "msrv", "stable", "examples_enabled", "nix_full")
JOBS = ("preflight", "native", "lint", "msrv", "stable", "examples", "nix")
LOCKS = ("Cargo.lock", *(f"examples/{name}/Cargo.lock" for name in EXAMPLES))


def command(argv, cwd=ROOT, env=None, capture=False):
    if not capture:
        print(f"[{Path(cwd).relative_to(ROOT)}] {shlex.join(argv)}", flush=True)
    result = subprocess.run(
        argv, cwd=cwd, env=env, check=True, text=True,
        stdout=subprocess.PIPE if capture else None,
    )
    return result.stdout if capture else None


def tracked_files():
    return command(["git", "ls-files", "-z"], capture=True).split("\0")[:-1]


def read_toml(path):
    text = (ROOT / path).read_text()
    if re.search(r"^(?:<<<<<<<|=======|>>>>>>>)", text, re.MULTILINE):
        raise ValueError(f"{path}: unresolved conflict marker")
    return tomllib.loads(text)


def tools():
    return read_toml("ci/tools.toml")


def rust_version():
    return read_toml("rust-toolchain.toml")["toolchain"]["channel"]


def require_version(label, actual, expected):
    if actual != expected:
        raise ValueError(f"{label}: expected {expected}, got {actual}")


def preflight():
    manifests = {
        p: read_toml(p) for p in tracked_files() if Path(p).name == "Cargo.toml"
    }
    require_version("MSRV/toolchain", rust_version(),
                    manifests["charming/Cargo.toml"]["package"]["rust-version"])
    versions = tools()
    required = ("wasm-bindgen", "js-sys", "web-sys")
    coupled = (*required, "wasm-bindgen-macro", "wasm-bindgen-macro-support",
               "wasm-bindgen-shared", "wasm-bindgen-futures", "v8", "dioxus")
    for lock in LOCKS:
        packages = read_toml(lock)["package"]
        for name in required:
            if not any(p["name"] == name for p in packages):
                raise ValueError(f"{lock}: missing {name}")
        if lock == "Cargo.lock" and not any(p["name"] == "v8" for p in packages):
            raise ValueError(f"{lock}: missing v8")
        if "/dioxus-" in lock and not any(p["name"] == "dioxus" for p in packages):
            raise ValueError(f"{lock}: missing dioxus")
        for package in packages:
            name = package["name"]
            if name in coupled:
                key = "dioxus-cli" if name == "dioxus" else name
                if name.startswith("wasm-bindgen-") and name != "wasm-bindgen-futures":
                    key = "wasm-bindgen"
                require_version(f"{lock}: {name}", package["version"], versions[key])
    print("preflight: manifests, locks and tool versions agree")


def cli_version(binary, expected):
    output = command([binary, "--version"], capture=True).strip()
    print(output, flush=True)
    match = re.search(r"\b\d+\.\d+\.\d+(?:[-+][\w.-]+)?\b", output)
    require_version(binary, match[0] if match else output, expected)


def environment():
    versions = tools()
    for binary in ("rustc", "cargo"):
        cli_version(binary, rust_version())
    for binary, key in (("dx", "dioxus-cli"), ("trunk", "trunk"),
                        ("wasm-bindgen", "wasm-bindgen")):
        cli_version(binary, versions[key])
    targets = command(["rustc", "--print", "target-libdir", "--target",
                       "wasm32-unknown-unknown"], capture=True).strip()
    if not any(Path(targets).glob("libstd-*.rlib")):
        raise ValueError("wasm32-unknown-unknown standard library is missing")
    archive = Path(os.environ.get("RUSTY_V8_ARCHIVE", ""))
    if not archive.is_file() or f"librusty_v8-{versions['v8']}-" not in archive.name:
        raise ValueError(f"RUSTY_V8_ARCHIVE: missing archive or wrong V8 version: {archive}")
    packages = read_toml("Cargo.lock")["package"]
    for package in packages:
        if package["name"] == "v8":
            require_version("V8 archive/crate", package["version"], versions["v8"])
            break
    else:
        raise ValueError("Cargo.lock: missing v8")


def example(name):
    versions = tools()
    cli_version("wasm-bindgen", versions["wasm-bindgen"])
    directory = ROOT / "examples" / name
    if name.startswith("dioxus-"):
        cli_version("dx", versions["dioxus-cli"])
        command(["dx", "build", "--locked", "--verbose", "--trace"], cwd=directory)
    else:
        cli_version("trunk", versions["trunk"])
        lock = directory / "Cargo.lock"
        before = lock.read_bytes()
        # Trunk's metadata call omits --locked; check even when the build fails.
        try:
            command(["trunk", "build", "--locked", "--verbose"], cwd=directory)
        finally:
            if not lock.exists() or lock.read_bytes() != before:
                message = f"{name}: Trunk changed Cargo.lock; left in place for inspection"
                if sys.exc_info()[0] is not None:
                    print(message, file=sys.stderr)
                else:
                    raise ValueError(message)


def wasm_browser(echarts6=False):
    binaries = {}
    for name in ("wasm-bindgen-test-runner", "chromium", "chromedriver"):
        path = shutil.which(name)
        if path is None:
            raise ValueError(f"wasm-browser: missing {name} on PATH")
        binaries[name] = path
    cli_version(binaries["wasm-bindgen-test-runner"], tools()["wasm-bindgen"])
    for name in ("chromium", "chromedriver"):
        print(command([binaries[name], "--version"], capture=True).strip(), flush=True)
    with tempfile.TemporaryDirectory(prefix="charming-wasm-browser-") as directory:
        directory = Path(directory)
        webdriver = directory / "webdriver.json"
        args = ["--headless", "--disable-dev-shm-usage"]
        if echarts6:
            args.extend(["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"])
        webdriver.write_text(json.dumps({"goog:chromeOptions": {
            "binary": binaries["chromium"],
            "args": args,
        }}))
        runner = directory / "runner"
        runner.write_text("#!/bin/sh\nexec " + shlex.quote(binaries["wasm-bindgen-test-runner"]) + ' "$@"\n')
        runner.chmod(0o700)
        env = dict(os.environ)
        # Isolate this invocation from persistent browser/remote-driver settings.
        for name in ("NO_HEADLESS", "WASM_BINDGEN_TEST_ONLY_NODE", "WASM_BINDGEN_TEST_ADDRESS",
                     "CHROMEDRIVER_REMOTE", "GECKODRIVER", "GECKODRIVER_REMOTE",
                     "SAFARIDRIVER", "SAFARIDRIVER_REMOTE"):
            env.pop(name, None)
        env.update(
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=str(runner),
            WASM_BINDGEN_USE_BROWSER="1",
            WASM_BINDGEN_TEST_ONLY_WEB="1",
            CHROMEDRIVER=binaries["chromedriver"],
            WASM_BINDGEN_TEST_WEBDRIVER_JSON=str(webdriver),
        )
        argv = ["cargo", "test", "-p", "charming", "--locked", "--target",
                "wasm32-unknown-unknown", "--no-default-features", "--features",
                "wasm,html" if echarts6 else "wasm", "--test",
                "echarts6_browser" if echarts6 else "wasm_callbacks"]
        if echarts6:
            argv.extend(["--", "--nocapture"])
            evidence = ROOT / "target/echarts6-evidence"
            evidence.mkdir(parents=True, exist_ok=True)
            # Keep the real browser console and command even when a test fails.
            result = subprocess.run(argv, cwd=ROOT, env=env, text=True,
                                    stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            (evidence / "browser.log").write_text(shlex.join(argv) + "\n" + result.stdout)
            print(re.sub(r"(CHARMING_GL_PNG:data:image/png;base64,)[A-Za-z0-9+/=]+",
                         r"\1<saved in evidence>", result.stdout), end="", flush=True)
            result.check_returncode()
            match = re.search(r"CHARMING_GL_PNG:data:image/png;base64,([A-Za-z0-9+/=]+)", result.stdout)
            if not match:
                raise ValueError("echarts6-browser: missing GL foreground PNG evidence")
            (evidence / "bar3d.png").write_bytes(base64.b64decode(match[1], validate=True))
        else:
            command(argv, env=env)


def run(task, name=None):
    print(f"task: {task}" + (f" {name}" if name else ""), flush=True)
    if task == "preflight":
        preflight()
    elif task == "env":
        environment()
    elif task == "example":
        example(name)
    elif task == "wasm-browser":
        wasm_browser()
    elif task == "echarts6-browser":
        wasm_browser(echarts6=True)
    elif task == "echarts6-raster":
        command(["cargo", "test", "-p", "charming", "--locked", "--no-default-features",
                 "--features", "ssr-raster", "--test", "echarts6_raster"])
    elif task == "snapshots":
        command(["cargo", "run", "--locked", "--bin", "generate_images", "--", "img_test"])
    elif task == "native":
        for argv in (
            ["cargo", "build", "--workspace", "--locked"],
            ["cargo", "run", "--locked", "--bin", "generate_images", "--", "img_test"],
            ["cargo", "test", "--workspace", "--locked"],
        ):
            command(argv)
    elif task == "fmt":
        command(["cargo", "fmt", "--all", "--check"])
    elif task == "clippy":
        command(["cargo", "clippy", "--workspace", "--all-features", "--all-targets",
                 "--locked", "--", "-D", "warnings"])
    elif task == "msrv":
        command(["cargo", "check", "-p", "charming", "--lib", "--locked"],
                env=dict(os.environ, RUSTUP_TOOLCHAIN=rust_version()))
    elif task == "stable":
        rustc = command(["rustup", "which", "--toolchain", "stable", "rustc"], capture=True).strip()
        # Nix's direct compiler can precede rustup proxies on PATH.
        env = dict(os.environ, RUSTC=rustc)
        command(["rustup", "run", "stable", "rustc", "--version"], env=env)
        base = ["rustup", "run", "stable", "cargo", "check", "-p", "charming", "--lib", "--locked"]
        for extra in (["--no-default-features"], [],
                      ["--no-default-features", "--features", "wasm", "--target",
                       "wasm32-unknown-unknown"]):
            command(base + extra, env=env)
    elif task == "nix-format":
        files = tracked_files()
        for binary, suffix in (("nixfmt", ".nix"), ("taplo", ".toml")):
            paths = [p for p in files if p.endswith(suffix)]
            if paths:
                command([binary, *(["fmt"] if binary == "taplo" else []), "--check", *paths])
    elif task in ("nix-smoke", "nix-full"):
        inner = "env" if task == "nix-smoke" else "all-nix"
        command(["nix", "develop", "--no-update-lock-file", "--command",
                 "python3", "scripts/check.py", "run", inner])
    elif task in ("all", "all-nix"):
        if task == "all-nix":
            run("env")
        for child in ("preflight", "native", "fmt", "clippy", "msrv"):
            run(child)
        for name in EXAMPLES:
            run("example", name)
        if task == "all-nix":
            run("nix-format")


def select_paths(paths, full_reason=None):
    enabled = set()
    examples = set()
    reasons = []
    nix_full = False

    def full(reason):
        nonlocal nix_full
        enabled.update(("native", "lint", "msrv", "stable"))
        examples.update(EXAMPLES)
        nix_full = True
        reasons.append(reason)

    if full_reason:
        full(full_reason)
    else:
        for path in paths:
            parts = Path(path).parts
            known_example = len(parts) > 2 and parts[0] == "examples" and parts[1] in EXAMPLES
            manifest = path in ("Cargo.toml", *LOCKS) or (
                Path(path).name == "Cargo.toml" and parts[0] in
                ("charming", "charming_macros", "gallery")
            ) or (known_example and parts[2] == "Cargo.toml")
            infrastructure = (
                path in ("rust-toolchain.toml", "flake.nix", "flake.lock", ".envrc")
                or path.endswith(".nix")
                or path.startswith((".cargo/", "scripts/", "ci/", ".github/workflows/", "nix/"))
            )
            docs = path in ("README.md", "CHANGELOG.md", "CONTRIBUTING.md", "AGENTS.md", "docs/echarts6.md",
                            ".github/pull_request_template.md") or (
                len(parts) == 1 and path.startswith("LICENSE")
            ) or (len(parts) == 3 and parts[0] == "examples" and parts[2].startswith("README"))
            if manifest or infrastructure:
                full(f"{path}: shared build/environment input")
            elif docs:
                reasons.append(f"{path}: documentation only")
            elif path.startswith(("charming/", "charming_macros/")):
                enabled.update(("native", "lint", "msrv", "stable"))
                examples.update(EXAMPLES)
                reasons.append(f"{path}: library")
            elif path.startswith(("gallery/", "img/")):
                enabled.update(("native", "lint"))
                reasons.append(f"{path}: gallery/snapshots")
            elif known_example:
                examples.add(parts[1])
                reasons.append(f"{path}: example")
            else:
                full(f"{path}: unknown path")
    return {
        **{key: key in enabled for key in ("native", "lint", "msrv", "stable")},
        "examples_enabled": bool(examples), "nix_full": nix_full,
        "examples": {"include": [{"name": name} for name in EXAMPLES if name in examples]},
        "reasons": reasons or ["no changed paths"],
    }


def selection(event, base, head):
    if event not in ("pull_request", "push"):
        return select_paths([], f"{event}: full validation")
    if any(not re.fullmatch(r"[0-9a-fA-F]{40}", sha) or set(sha) == {"0"}
           for sha in (base, head)):
        return select_paths([], "missing or zero base/head: full validation")
    try:
        if event == "pull_request":
            base = command(["git", "merge-base", base, head], capture=True).strip()
        paths = command(["git", "diff", "--name-only", "-z", "--no-renames",
                         f"{base}..{head}", "--"], capture=True).split("\0")[:-1]
    except (OSError, subprocess.CalledProcessError) as error:
        return select_paths([], f"unavailable base/diff: {error}")
    return select_paths(paths)


def emit_selection(selected):
    encoded = json.dumps(selected, separators=(",", ":"))
    print(encoded)
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as output:
            for key in (*FLAGS, "examples"):
                output.write(f"{key}={json.dumps(selected[key], separators=(',', ':'))}\n")
            output.write(f"selection={encoded}\n")


def gate(selected, needs):
    if not isinstance(selected, dict) or any(type(selected.get(k)) is not bool for k in FLAGS):
        raise ValueError("selection must contain boolean task flags")
    matrix = selected.get("examples")
    if not isinstance(matrix, dict) or not isinstance(matrix.get("include"), list):
        raise ValueError("selection has no example matrix")
    names = []
    for entry in matrix["include"]:
        if not isinstance(entry, dict) or entry.get("name") not in EXAMPLES:
            raise ValueError("selection has invalid example")
        names.append(entry["name"])
    if len(names) != len(set(names)) or bool(names) != selected["examples_enabled"]:
        raise ValueError("selection has inconsistent example matrix")
    reasons = selected.get("reasons")
    if not isinstance(reasons, list) or not reasons or any(not isinstance(r, str) for r in reasons):
        raise ValueError("selection has invalid reasons")
    if not isinstance(needs, dict):
        raise ValueError("needs must be an object")
    print("Selection: " + json.dumps(selected))
    errors = []
    for job in JOBS:
        required = job in ("preflight", "nix") or selected.get(
            "examples_enabled" if job == "examples" else job, False
        )
        result = needs.get(job, {})
        result = result.get("result") if isinstance(result, dict) else None
        print(f"{job}: {result} (required={required})")
        if result != "success" and (required or result != "skipped"):
            errors.append(f"{job}: {result}")
    if errors:
        raise ValueError("CI gate failed: " + "; ".join(errors))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    runner = commands.add_parser("run")
    runner.add_argument("task", choices=("preflight", "native", "fmt", "clippy", "msrv",
                        "stable", "example", "env", "nix-format", "all", "all-nix",
                        "nix-smoke", "nix-full", "wasm-browser", "echarts6-browser",
                        "echarts6-raster", "snapshots"))
    runner.add_argument("name", nargs="?", choices=EXAMPLES)
    selector = commands.add_parser("select")
    selector.add_argument("--event", required=True)
    selector.add_argument("--base", default="")
    selector.add_argument("--head", default="")
    commands.add_parser("gate")
    args = parser.parse_args()
    try:
        if args.command == "run":
            if (args.task == "example") != (args.name is not None):
                parser.error("only 'run example' requires NAME")
            run(args.task, args.name)
        elif args.command == "select":
            emit_selection(selection(args.event, args.base, args.head))
        else:
            gate(json.loads(os.environ["CI_SELECTION_JSON"]), json.loads(os.environ["CI_NEEDS_JSON"]))
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        print(f"{args.command} {getattr(args, 'task', '')}: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
