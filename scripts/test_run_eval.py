#!/usr/bin/env python3
"""Integration tests for run-eval image precedence, supervision, and cleanup."""

from __future__ import annotations

import os
import subprocess
import tempfile
import time
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "run-eval.sh"


PASSING_MAKE = r"""#!/usr/bin/env bash
set -euo pipefail
overlay=''
image=''
for arg in "$@"; do
    case "$arg" in
        RV_TESTSUITE_RUN_IMG=*|LA_TESTSUITE_RUN_IMG=*) overlay="${arg#*=}" ;;
        RV_TESTSUITE_IMG=*|LA_TESTSUITE_IMG=*) image="${arg#*=}" ;;
    esac
done
test -n "$overlay"
test -n "$image"
: >"$overlay"
printf '%s|%s\n' "$image" "$overlay" >>"$RUN_EVAL_TEST_RECORD"
printf '%s\n' \
  '#### OS COMP TEST GROUP START ltp-musl ####' \
  'ltp case list: inline (1 cases, timeout 30s)' \
  'RUN LTP CASE access01' \
  'FAIL LTP CASE access01 : 0' \
  'ltp cases: 1 passed, 0 failed, 0 timed out' \
  '#### OS COMP TEST GROUP END ltp-musl ####'
"""


TIMEOUT_MAKE = r"""#!/usr/bin/env bash
set -euo pipefail
overlay=''
for arg in "$@"; do
    case "$arg" in
        RV_TESTSUITE_RUN_IMG=*|LA_TESTSUITE_RUN_IMG=*) overlay="${arg#*=}" ;;
    esac
done
test -n "$overlay"
: >"$overlay"
printf 'runner started\n'
python3 -c 'import os,time; os.setsid(); open(os.environ["RUN_EVAL_CHILD_PID"], "w").write(str(os.getpid())); time.sleep(60)' &
sleep 60
"""


NON_LTP_FAILURE_MAKE = PASSING_MAKE + r"""
printf '%s\n' \
  '#### OS COMP TEST GROUP START busybox-musl ####' \
  'testcase busybox echo fail' \
  '#### OS COMP TEST GROUP END busybox-musl ####'
"""


OFFICIAL_SKIP_MAKE = PASSING_MAKE + r"""
printf '%s\n' '[CONTEST][OFFICIAL][SKIP] libctest-glibc: configured skip'
"""


class RunEvalTest(unittest.TestCase):
    def environment(self, root: Path, make_script: str = PASSING_MAKE) -> tuple[dict[str, str], Path]:
        bin_dir = root / "bin"
        suite_dir = root / "suite"
        output_root = root / "evidence"
        bin_dir.mkdir()
        suite_dir.mkdir()
        (suite_dir / "sdcard-rv.img").write_bytes(b"rv-image")
        (suite_dir / "sdcard-la.img").write_bytes(b"la-image")
        make = bin_dir / "make"
        make.write_text(make_script)
        make.chmod(0o755)
        for name in ("qemu-img", "qemu-system-riscv64", "qemu-system-loongarch64"):
            stub = bin_dir / name
            stub.write_text("#!/usr/bin/env sh\nexit 0\n")
            stub.chmod(0o755)
        record = root / "record.txt"
        env = dict(os.environ)
        env.update(
            {
                "PATH": f"{bin_dir}:{env['PATH']}",
                "TESTSUITE_DIR": str(suite_dir),
                "RUN_EVAL_OUTPUT_ROOT": str(output_root),
                "RUN_EVAL_TEST_RECORD": str(record),
                "RUN_EVAL_TIMEOUT_SECS": "30",
            }
        )
        return env, record

    def test_testsuite_dir_precedence_and_owned_overlay_cleanup(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            env, record = self.environment(root)
            result = subprocess.run(
                [str(SCRIPT), "rv"],
                cwd=ROOT,
                env=env,
                check=False,
                capture_output=True,
                text=True,
            )
            image, overlay = record.read_text().strip().split("|")
            run_dirs = list((root / "evidence").iterdir())

            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(image, str(root / "suite" / "sdcard-rv.img"))
            self.assertFalse(Path(overlay).exists())
            self.assertEqual(len(run_dirs), 1)
            self.assertTrue((run_dirs[0] / "evaluator.log").is_file())
            self.assertTrue((run_dirs[0] / "ltp-summary.json").is_file())
            self.assertTrue((run_dirs[0] / "ltp-summary.md").is_file())
            self.assertTrue((run_dirs[0] / "failure-report.md").is_file())
            self.assertNotIn(
                root.as_posix(),
                (run_dirs[0] / "ltp-summary.md").read_text(encoding="utf-8"),
            )
            self.assertNotIn(
                root.as_posix(),
                (run_dirs[0] / "failure-report.md").read_text(encoding="utf-8"),
            )

    def test_parallel_runs_use_distinct_overlays(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            env, record = self.environment(root)
            processes = [
                subprocess.Popen(
                    [str(SCRIPT), "rv"],
                    cwd=ROOT,
                    env=env,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                )
                for _ in range(2)
            ]
            results = [process.communicate(timeout=30) for process in processes]
            records = [line.split("|") for line in record.read_text().splitlines()]

            self.assertEqual([process.returncode for process in processes], [0, 0], results)
            overlays = [Path(row[1]) for row in records]
            self.assertEqual(len(set(overlays)), 2)
            self.assertTrue(all(not overlay.exists() for overlay in overlays))

    def test_timeout_cleans_detached_descendant(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            env, _record = self.environment(root, TIMEOUT_MAKE)
            pid_file = root / "child.pid"
            env["RUN_EVAL_CHILD_PID"] = str(pid_file)
            env["RUN_EVAL_TIMEOUT_SECS"] = "1"
            result = subprocess.run(
                [str(SCRIPT), "rv"],
                cwd=ROOT,
                env=env,
                check=False,
                capture_output=True,
                text=True,
                timeout=20,
            )
            for _ in range(50):
                if pid_file.exists():
                    break
                time.sleep(0.02)
            self.assertTrue(pid_file.exists(), result)
            child_pid = int(pid_file.read_text())

            self.assertEqual(result.returncode, 124, result.stderr)
            self.assertFalse(Path(f"/proc/{child_pid}").exists())

    def test_non_ltp_semantic_failure_is_never_green(self) -> None:
        for make_script in (NON_LTP_FAILURE_MAKE, OFFICIAL_SKIP_MAKE):
            with self.subTest(make_script=make_script[-100:]), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                env, _record = self.environment(root, make_script)
                result = subprocess.run(
                    [str(SCRIPT), "rv"],
                    cwd=ROOT,
                    env=env,
                    check=False,
                    capture_output=True,
                    text=True,
                )
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertIn("semantic non-pass", result.stderr)

    def test_external_sigterm_preserves_log_and_cleans_detached_descendant(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            env, _record = self.environment(root, TIMEOUT_MAKE)
            pid_file = root / "child.pid"
            env["RUN_EVAL_CHILD_PID"] = str(pid_file)
            process = subprocess.Popen(
                [str(SCRIPT), "rv"],
                cwd=ROOT,
                env=env,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            for _ in range(300):
                if pid_file.exists():
                    break
                time.sleep(0.02)
            self.assertTrue(pid_file.exists())
            child_pid = int(pid_file.read_text())
            process.terminate()
            stdout, stderr = process.communicate(timeout=20)
            for _ in range(100):
                if not Path(f"/proc/{child_pid}").exists():
                    break
                time.sleep(0.02)
            run_dirs = list((root / "evidence").iterdir())
            self.assertEqual(process.returncode, 143, (stdout, stderr))
            self.assertFalse(Path(f"/proc/{child_pid}").exists())
            self.assertEqual(len(run_dirs), 1)
            self.assertFalse((run_dirs[0] / "sdcard-overlay.qcow2").exists())
            self.assertIn("runner started", (run_dirs[0] / "evaluator.log").read_text())

    def test_missing_image_is_visible_error(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            env, _record = self.environment(root)
            env["RV_TESTSUITE_IMG"] = str(root / "missing.img")
            result = subprocess.run(
                [str(SCRIPT), "rv"],
                cwd=ROOT,
                env=env,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 2)
            self.assertIn("missing.img", result.stderr)


if __name__ == "__main__":
    unittest.main()
