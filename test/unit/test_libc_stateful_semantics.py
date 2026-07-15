#!/usr/bin/env python3
"""Regression tests for the libc-stateful-semantics empty-shell static guard."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GUARD = ROOT / "test/checks/check_libc_stateful_semantics.py"
TARGETS = [
    Path("ulib/axlibc/c/signal.c"),
    Path("ulib/axlibc/c/unistd.c"),
    Path("ulib/axlibc/c/dirent.c"),
    Path("ulib/axlibc/c/dlfcn.c"),
    Path("ulib/axlibc/c/stdlib.c"),
    Path("ulib/axlibc/c/time.c"),
    Path("ulib/axlibc/c/pthread.c"),
]


class LibcStatefulSemanticsGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="libc-stateful-semantics-guard-"))
        self.addCleanup(lambda: shutil.rmtree(tmp, ignore_errors=True))
        for rel in TARGETS:
            dst = tmp / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text((ROOT / rel).read_text(encoding="utf-8"), encoding="utf-8")
        return tmp

    def run_guard(self, tree: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(GUARD), "--root", str(tree)],
            check=False,
            capture_output=True,
            text=True,
        )

    def test_current_tree_passes(self) -> None:
        result = self.run_guard(ROOT)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("PASS", result.stdout)

    def test_detects_sigaction_zero_oldact_shell(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/signal.c"
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "*oldact = signal_actions[signum];",
                "*oldact = (struct sigaction){0};",
                1,
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("sigaction_helper", result.stdout)

    def test_detects_raise_enosys_shell(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/signal.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("int raise(int __sig)")
        end = text.index("int sigaddset", start)
        text = text[:start] + "int raise(int __sig)\n{\n    errno = ENOSYS;\n    return -1;\n}\n\n" + text[end:]
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("raise", result.stdout)

    def test_detects_isatty_without_fd_probe(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/unistd.c"
        path.write_text(path.read_text(encoding="utf-8").replace("fcntl(fd, F_GETFD)", "fd >= 0", 1), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("isatty", result.stdout)

    def test_detects_opendir_unconditional_null(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/dirent.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("DIR *opendir")
        end = text.index("struct dirent *readdir", start)
        text = text[:start] + "DIR *opendir(const char *__name)\n{\n    unimplemented();\n    return NULL;\n}\n\n" + text[end:]
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("opendir", result.stdout)

    def test_detects_qsort_noop(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/stdlib.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("void qsort")
        end = text.index("// TODO", start + 1)
        text = text[:start] + "void qsort(void *base, size_t nel, size_t width, cmpfun cmp)\n{\n    unimplemented();\n    return;\n}\n\n" + text[end:]
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("qsort", result.stdout)

    def test_detects_dladdr_fake_success(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/dlfcn.c"
        path.write_text(path.read_text(encoding="utf-8").replace("return 0;", "return 1;", 1), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("dladdr", result.stdout)

    def test_detects_tzset_unimplemented_shell(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/time.c"
        text = path.read_text(encoding="utf-8")
        start = text.index("void tzset")
        end = text.index("// TODO", start)
        text = text[:start] + "void tzset()\n{\n    unimplemented();\n}\n\n" + text[end:]
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("tzset", result.stdout)

    def test_detects_pthread_testcancel_unimplemented_log_shell(self) -> None:
        tree = self.make_tree()
        path = tree / "ulib/axlibc/c/pthread.c"
        path.write_text(path.read_text(encoding="utf-8").replace("void pthread_testcancel(void)\n{\n    return;\n}", "void pthread_testcancel(void)\n{\n    unimplemented();\n    return;\n}", 1), encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("pthread_testcancel", result.stdout)


if __name__ == "__main__":
    unittest.main()
