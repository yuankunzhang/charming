"""Control-logic tests; subprocesses are stubbed and no product is compiled."""

import contextlib
import io
import json
import os
import re
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import check


class SelectionTests(unittest.TestCase):
    def test_path_table(self):
        all_jobs = {"native", "lint", "msrv", "stable", "examples_enabled", "nix_full"}
        cases = {
            "Cargo.toml": all_jobs,
            "Cargo.lock": all_jobs,
            "charming/Cargo.toml": all_jobs,
            "gallery/Cargo.toml": all_jobs,
            "charming_macros/Cargo.toml": all_jobs,
            "rust-toolchain.toml": all_jobs,
            ".cargo/config.toml": all_jobs,
            "scripts/check.py": all_jobs,
            "ci/tools.toml": all_jobs,
            ".github/workflows/rust.yml": all_jobs,
            "flake.nix": all_jobs,
            "flake.lock": all_jobs,
            "nested/custom.nix": all_jobs,
            "nix/package.patch": all_jobs,
            ".envrc": all_jobs,
            "charming/src/lib.rs": all_jobs - {"nix_full"},
            "charming_macros/src/lib.rs": all_jobs - {"nix_full"},
            "gallery/src/images.rs": {"native", "lint"},
            "img/chart.svg": {"native", "lint"},
            "README.md": set(),
            "CHANGELOG.md": set(),
            "CONTRIBUTING.md": set(),
            "AGENTS.md": set(),
            "LICENSE-MIT": set(),
            ".github/pull_request_template.md": set(),
            "unknown/path": all_jobs,
        }
        for name in check.EXAMPLES:
            cases[f"examples/{name}/Cargo.toml"] = all_jobs
            cases[f"examples/{name}/Cargo.lock"] = all_jobs
            cases[f"examples/{name}/README.md"] = set()
            cases[f"examples/{name}/src/main.rs"] = {"examples_enabled"}
        for path, expected in cases.items():
            with self.subTest(path=path):
                selected = check.select_paths([path])
                self.assertEqual({k for k in check.FLAGS if selected[k]}, expected)
                names = [e["name"] for e in selected["examples"]["include"]]
                if expected == {"examples_enabled"}:
                    self.assertEqual(names, [path.split("/")[1]])
                elif "examples_enabled" in expected:
                    self.assertEqual(names, list(check.EXAMPLES))
                else:
                    self.assertEqual(names, [])

    def test_mixed_paths_union(self):
        selected = check.select_paths([
            "README.md", "gallery/src/images.rs", "examples/yew-demo/src/main.rs",
            "examples/leptos-demo/src/main.rs", "examples/yew-demo/src/other.rs",
        ])
        self.assertTrue(selected["native"])
        self.assertTrue(selected["lint"])
        self.assertFalse(selected["msrv"])
        self.assertFalse(selected["nix_full"])
        self.assertEqual(selected["examples"]["include"],
                         [{"name": "leptos-demo"}, {"name": "yew-demo"}])

    def test_empty_paths_have_empty_matrix(self):
        selected = check.select_paths([])
        self.assertFalse(any(selected[k] for k in check.FLAGS))
        self.assertEqual(selected["examples"], {"include": []})
        self.assertTrue(selected["reasons"])

    def test_manual_unknown_events_and_bad_shas_fall_back(self):
        with patch.object(check, "command") as command:
            for event, base, head in (
                ("workflow_dispatch", "a" * 40, "b" * 40),
                ("unknown", "a" * 40, "b" * 40),
                ("push", "", "b" * 40),
                ("push", "0" * 40, "b" * 40),
                ("pull_request", "a" * 40, ""),
                ("push", "--bad", "b" * 40),
            ):
                with self.subTest(event=event, base=base, head=head):
                    selected = check.selection(event, base, head)
                    self.assertTrue(all(selected[k] for k in check.FLAGS))
                    self.assertTrue(selected["reasons"])
            command.assert_not_called()

    def test_pr_uses_merge_base_and_keeps_rename_and_delete_paths(self):
        with patch.object(check, "command", side_effect=[
            "c" * 40 + "\n",
            "examples/yew-demo/old.rs\0gallery/new.rs\0charming/deleted.rs\0",
        ]) as command:
            selected = check.selection("pull_request", "a" * 40, "b" * 40)
        self.assertEqual(command.call_args_list[0].args[0],
                         ["git", "merge-base", "a" * 40, "b" * 40])
        self.assertEqual(command.call_args_list[1].args[0], [
            "git", "diff", "--name-only", "-z", "--no-renames",
            "c" * 40 + ".." + "b" * 40, "--",
        ])
        self.assertTrue(selected["stable"])
        self.assertFalse(selected["nix_full"])
        self.assertEqual(len(selected["reasons"]), 3)

    def test_push_diffs_before_after_without_merge_base(self):
        with patch.object(check, "command", return_value="README.md\0") as command:
            selected = check.selection("push", "a" * 40, "b" * 40)
        self.assertEqual(command.call_count, 1)
        self.assertIn("a" * 40 + ".." + "b" * 40, command.call_args.args[0])
        self.assertFalse(selected["native"])

    def test_unavailable_base_and_diff_failure_fall_back(self):
        for event, results in (
            ("push", [subprocess.CalledProcessError(128, ["git", "diff"])]),
            ("pull_request", [subprocess.CalledProcessError(128, ["git", "merge-base"])]),
            ("pull_request", ["c" * 40, OSError("diff unavailable")]),
        ):
            with self.subTest(event=event, results=results):
                with patch.object(check, "command", side_effect=results):
                    selected = check.selection(event, "a" * 40, "b" * 40)
                self.assertTrue(all(selected[k] for k in check.FLAGS))
                self.assertIn("unavailable", selected["reasons"][0])

    def test_github_outputs_are_literal_json_and_booleans(self):
        selected = check.select_paths(['README.md', 'unknown/line\noutput=false'])
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "output"
            with patch.dict(os.environ, GITHUB_OUTPUT=str(path)):
                with contextlib.redirect_stdout(io.StringIO()):
                    check.emit_selection(selected)
            lines = dict(line.split("=", 1) for line in path.read_text().splitlines())
        self.assertEqual(set(lines), {*check.FLAGS, "examples", "selection"})
        self.assertEqual(lines["native"], "true")
        self.assertEqual(json.loads(lines["examples"]), selected["examples"])
        self.assertEqual(json.loads(lines["selection"]), selected)


class GateTests(unittest.TestCase):
    def setUp(self):
        self.selected = check.select_paths(["gallery/src/images.rs"])
        self.needs = {
            job: {"result": "success" if job in ("preflight", "native", "lint", "nix") else "skipped"}
            for job in check.JOBS
        }

    def invoke(self, selected=None, needs=None):
        with contextlib.redirect_stdout(io.StringIO()):
            check.gate(self.selected if selected is None else selected,
                       self.needs if needs is None else needs)

    def test_success_and_expected_skip(self):
        self.invoke()
        self.invoke(needs={job: {"result": "success"} for job in check.JOBS})
        docs = check.select_paths(["README.md"])
        self.invoke(selected=docs, needs={
            job: {"result": "success" if job in ("preflight", "nix") else "skipped"}
            for job in check.JOBS
        })

    def test_every_job_rejects_failure_cancelled_missing_invalid(self):
        for job in check.JOBS:
            for result in ("failure", "cancelled", None, "", "pending", 5):
                with self.subTest(job=job, result=result):
                    needs = dict(self.needs, **{job: {"result": result}})
                    with self.assertRaisesRegex(ValueError, job):
                        self.invoke(needs=needs)
            needs = self.needs.copy()
            del needs[job]
            with self.assertRaisesRegex(ValueError, job):
                self.invoke(needs=needs)

    def test_required_jobs_reject_unexpected_skip(self):
        for job in ("preflight", "native", "lint", "nix"):
            with self.subTest(job=job), self.assertRaisesRegex(ValueError, job):
                self.invoke(needs=dict(self.needs, **{job: {"result": "skipped"}}))
        full = check.select_paths([], "manual")
        for job in check.JOBS:
            needs = {j: {"result": "success"} for j in check.JOBS}
            needs[job] = {"result": "skipped"}
            with self.subTest(job=job), self.assertRaisesRegex(ValueError, job):
                self.invoke(selected=full, needs=needs)

    def test_malformed_selection_and_needs(self):
        invalid = [None, [], {}, dict(self.selected, native="true"),
                   dict(self.selected, examples=None), dict(self.selected, reasons=[]),
                   dict(self.selected, examples={"include": [{"name": "bad"}]}),
                   dict(self.selected, examples_enabled=True),
                   dict(self.selected, examples_enabled=True, examples={"include": [
                       {"name": "yew-demo"}, {"name": "yew-demo"}]}),
                   dict(self.selected, examples={"include": [None]})]
        for selected in invalid:
            with self.subTest(selected=selected), self.assertRaises(ValueError):
                with contextlib.redirect_stdout(io.StringIO()):
                    check.gate(selected, self.needs)
        for needs in (None, [], {"preflight": "success"}):
            with self.subTest(needs=needs), self.assertRaises(ValueError):
                with contextlib.redirect_stdout(io.StringIO()):
                    check.gate(self.selected, needs)


class RunnerTests(unittest.TestCase):
    def test_native_stops_at_first_failed_subprocess(self):
        error = subprocess.CalledProcessError(42, ["cargo", "build"])
        with patch.object(check.subprocess, "run", side_effect=error) as run:
            with self.assertRaises(subprocess.CalledProcessError) as caught:
                check.run("native")
        self.assertIs(caught.exception, error)
        run.assert_called_once()
        self.assertTrue(run.call_args.kwargs["check"])
        self.assertEqual(run.call_args.kwargs["cwd"], check.ROOT)

    def test_cli_version_mismatch_stops_example_before_build(self):
        with patch.object(check, "command", return_value="wasm-bindgen 0.2.127") as command:
            with self.assertRaisesRegex(ValueError, "expected 0.2.129"):
                check.example("yew-demo")
        command.assert_called_once_with(["wasm-bindgen", "--version"], capture=True)

    def test_trunk_lock_guard_on_success_failure_and_deleted_lock(self):
        for changed in (False, True, "deleted"):
            for fails in (False, True):
                with self.subTest(changed=changed, fails=fails), tempfile.TemporaryDirectory() as directory:
                    root = Path(directory)
                    lock = root / "examples/yew-demo/Cargo.lock"
                    lock.parent.mkdir(parents=True)
                    lock.write_bytes(b"original")
                    original = subprocess.CalledProcessError(42, ["trunk", "build"])

                    def build(argv, **kwargs):
                        self.assertEqual(argv, ["trunk", "build", "--locked", "--verbose"])
                        self.assertEqual(kwargs["cwd"], lock.parent)
                        if changed == "deleted":
                            lock.unlink()
                        elif changed:
                            lock.write_bytes(b"changed")
                        if fails:
                            raise original

                    stderr = io.StringIO()
                    with patch.object(check, "ROOT", root), patch.object(check, "tools", return_value={"wasm-bindgen": "v", "trunk": "v"}), \
                         patch.object(check, "cli_version"), patch.object(check, "command", side_effect=build), \
                         contextlib.redirect_stderr(stderr):
                        if fails:
                            with self.assertRaises(subprocess.CalledProcessError) as caught:
                                check.example("yew-demo")
                            self.assertIs(caught.exception, original)
                            self.assertEqual("changed Cargo.lock" in stderr.getvalue(), bool(changed))
                        elif changed:
                            with self.assertRaisesRegex(ValueError, "changed Cargo.lock"):
                                check.example("yew-demo")
                        else:
                            check.example("yew-demo")
                    if changed is True:
                        self.assertEqual(lock.read_bytes(), b"changed")

    def test_dioxus_uses_locked_without_lock_snapshots(self):
        with patch.object(check, "cli_version") as version, patch.object(check, "command") as command, \
             patch.object(Path, "read_bytes", side_effect=AssertionError("no snapshot")):
            check.example("dioxus-web-demo")
        self.assertEqual(version.call_args_list[-1].args, ("dx", "0.7.10"))
        command.assert_called_once_with(["dx", "build", "--locked", "--verbose", "--trace"],
                                        cwd=check.ROOT / "examples/dioxus-web-demo")

    def test_all_and_nix_full_enter_shell_only_once(self):
        with patch.object(check, "command") as command:
            check.run("nix-full")
        command.assert_called_once_with([
            "nix", "develop", "--no-update-lock-file", "--command", "python3", "scripts/check.py",
            "run", "all-nix",
        ])
        with patch.object(check, "environment") as environment, \
             patch.object(check, "preflight"), patch.object(check, "command"), \
             patch.object(check, "example") as example:
            check.run("all-nix")
        environment.assert_called_once_with()
        self.assertEqual([c.args[0] for c in example.call_args_list], list(check.EXAMPLES))

    def test_stable_overrides_toolchain_and_checks_three_modes(self):
        with patch.object(check, "command") as command:
            check.run("stable")
        self.assertEqual(command.call_count, 4)
        for call in command.call_args_list:
            self.assertEqual(call.kwargs["env"]["RUSTUP_TOOLCHAIN"], "stable")
        self.assertIn("--no-default-features", command.call_args_list[1].args[0])
        self.assertNotIn("--no-default-features", command.call_args_list[2].args[0])
        self.assertEqual(command.call_args_list[3].args[0][-4:],
                         ["--features", "wasm", "--target", "wasm32-unknown-unknown"])

    def test_preflight_checks_versions_and_conflicts_without_metadata(self):
        manifests = ["Cargo.toml", "charming/Cargo.toml"]
        versions = check.tools()
        documents = {
            "Cargo.toml": {},
            "charming/Cargo.toml": {"package": {"rust-version": check.rust_version()}},
            "rust-toolchain.toml": {"toolchain": {"channel": check.rust_version()}},
            "ci/tools.toml": versions,
        }
        for lock in check.LOCKS:
            documents[lock] = {"package": [
                {"name": name, "version": versions[name]}
                for name in ("wasm-bindgen", "js-sys", "web-sys", "v8")
            ] + ([{"name": "dioxus", "version": versions["dioxus-cli"]}]
                 if "/dioxus-" in lock else [])}
        with patch.object(check, "tracked_files", return_value=manifests), \
             patch.object(check, "read_toml", side_effect=documents.__getitem__), \
             patch.object(check, "command") as command:
            check.preflight()
            documents["Cargo.lock"]["package"][0]["version"] = "0.2.127"
            with self.assertRaisesRegex(ValueError, "Cargo.lock: wasm-bindgen"):
                check.preflight()
            command.assert_not_called()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "Cargo.toml").write_text("<<<<<<< branch\n")
            with patch.object(check, "ROOT", root), self.assertRaisesRegex(ValueError, "conflict marker"):
                check.read_toml("Cargo.toml")


class WorkflowTests(unittest.TestCase):
    def test_job_outputs_matrix_and_gate_connect_to_runner(self):
        workflow = (check.ROOT / ".github/workflows/rust.yml").read_text()
        jobs = workflow.split("jobs:\n", 1)[1]
        self.assertEqual(set(re.findall(r"^  ([a-z-]+):$", jobs, re.MULTILINE)),
                         {*check.JOBS, "ci-gate"})
        for key in (*check.FLAGS, "examples", "selection"):
            self.assertIn(f"{key}: ${{{{ steps.select.outputs.{key} }}}}", workflow)
        for job in ("native", "lint", "msrv", "stable", "examples"):
            flag = "examples_enabled" if job == "examples" else job
            self.assertIn(f"if: needs.preflight.outputs.{flag} == 'true'", workflow)
        self.assertIn("matrix: ${{ fromJSON(needs.preflight.outputs.examples) }}", workflow)
        self.assertIn("run example \"$EXAMPLE_NAME\"", workflow)
        self.assertIn("needs: [preflight, native, lint, msrv, stable, examples, nix]", workflow)
        self.assertIn("CI_SELECTION_JSON: ${{ needs.preflight.outputs.selection }}", workflow)
        self.assertIn("CI_NEEDS_JSON: ${{ toJSON(needs) }}", workflow)
        self.assertIn("run: python3 scripts/check.py gate", workflow)


if __name__ == "__main__":
    unittest.main()
