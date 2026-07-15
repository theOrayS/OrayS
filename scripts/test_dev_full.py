#!/usr/bin/env python3
"""Focused Linux-reference and source regression tests for /dev/full."""

from __future__ import annotations

import ctypes
import errno
import fcntl
import os
import select
import shutil
import stat
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
GUARD = ROOT / "scripts/check_dev_full.py"
TARGETS = [
    Path("user/shell/src/uspace/fd_table.rs"),
    Path("user/shell/src/uspace/metadata.rs"),
]


class DevFullSourceGuardTest(unittest.TestCase):
    def make_tree(self) -> Path:
        tmp = Path(tempfile.mkdtemp(prefix="dev-full-guard-"))
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

    def test_detects_zero_length_write_fake_success(self) -> None:
        tree = self.make_tree()
        path = tree / TARGETS[0]
        text = path.read_text(encoding="utf-8")
        text = text.replace(
            "FdEntry::DevFull(status_flags) => {\n                if !file_is_writable(*status_flags)",
            "FdEntry::DevFull(status_flags) => {\n                if src.is_empty() { return Ok(0); }\n                if !file_is_writable(*status_flags)",
            1,
        )
        path.write_text(text, encoding="utf-8")
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("length zero", result.stdout)

    def test_detects_wrong_device_number(self) -> None:
        tree = self.make_tree()
        path = tree / TARGETS[1]
        path.write_text(
            path.read_text(encoding="utf-8").replace("DEV_FULL_RDEV: u64 = 263", "DEV_FULL_RDEV: u64 = 0"),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("makedev(1, 7)", result.stdout)

    def test_detects_missing_directory_entry(self) -> None:
        tree = self.make_tree()
        path = tree / TARGETS[0]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                '&["cpu_dma_latency", "full"]',
                '&["cpu_dma_latency"]',
            ),
            encoding="utf-8",
        )
        result = self.run_guard(tree)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("getdents64", result.stdout)


class HostLinuxDevFullReferenceTest(unittest.TestCase):
    def assert_oserror(self, expected_errno: int, operation) -> None:
        with self.assertRaises(OSError) as caught:
            operation()
        self.assertEqual(caught.exception.errno, expected_errno)

    def test_read_write_and_access_modes(self) -> None:
        fd = os.open("/dev/full", os.O_RDWR)
        self.addCleanup(os.close, fd)
        data = os.read(fd, 64)
        self.assertEqual(data, bytes(64))
        self.assert_oserror(errno.ENOSPC, lambda: os.write(fd, b""))
        self.assert_oserror(errno.ENOSPC, lambda: os.write(fd, b"x"))

        read_only = os.open("/dev/full", os.O_RDONLY)
        self.addCleanup(os.close, read_only)
        self.assert_oserror(errno.EBADF, lambda: os.write(read_only, b"x"))

        write_only = os.open("/dev/full", os.O_WRONLY)
        self.addCleanup(os.close, write_only)
        self.assert_oserror(errno.EBADF, lambda: os.read(write_only, 1))

        invalid_access = os.open("/dev/full", 3)
        self.addCleanup(os.close, invalid_access)
        self.assert_oserror(errno.EBADF, lambda: os.read(invalid_access, 1))
        self.assert_oserror(errno.EBADF, lambda: os.write(invalid_access, b"x"))
        self.assert_oserror(
            errno.ENOTDIR,
            lambda: os.open("/dev/full", os.O_RDONLY | os.O_DIRECTORY),
        )
        self.assert_oserror(
            errno.EEXIST,
            lambda: os.open("/dev/full", os.O_RDWR | os.O_CREAT | os.O_EXCL),
        )

    def test_lseek_and_poll(self) -> None:
        fd = os.open("/dev/full", os.O_RDWR)
        self.addCleanup(os.close, fd)
        for whence in (os.SEEK_SET, os.SEEK_CUR, os.SEEK_END, 3, 4):
            for offset in (-123, 0, 123):
                self.assertEqual(os.lseek(fd, offset, whence), 0)
        self.assert_oserror(errno.EINVAL, lambda: os.lseek(fd, 0, 5))

        poller = select.poll()
        poller.register(fd, select.POLLIN | select.POLLOUT | select.POLLPRI)
        events = poller.poll(0)
        self.assertEqual(len(events), 1)
        self.assertEqual(events[0][0], fd)
        self.assertEqual(events[0][1], select.POLLIN | select.POLLOUT)

    def test_invalid_user_pointers(self) -> None:
        libc = ctypes.CDLL(None, use_errno=True)
        libc.read.argtypes = [ctypes.c_int, ctypes.c_void_p, ctypes.c_size_t]
        libc.read.restype = ctypes.c_ssize_t
        libc.write.argtypes = [ctypes.c_int, ctypes.c_void_p, ctypes.c_size_t]
        libc.write.restype = ctypes.c_ssize_t
        fd = os.open("/dev/full", os.O_RDWR)
        self.addCleanup(os.close, fd)

        ctypes.set_errno(0)
        self.assertEqual(libc.read(fd, ctypes.c_void_p(1), 1), -1)
        self.assertEqual(ctypes.get_errno(), errno.EFAULT)
        ctypes.set_errno(0)
        self.assertEqual(libc.write(fd, ctypes.c_void_p(1), 1), -1)
        self.assertEqual(ctypes.get_errno(), errno.ENOSPC)
        ctypes.set_errno(0)
        self.assertEqual(libc.write(fd, ctypes.c_void_p(1), 0), -1)
        self.assertEqual(ctypes.get_errno(), errno.ENOSPC)

    def test_stat_getdents_opath_dup_and_fcntl(self) -> None:
        fd = os.open("/dev/full", os.O_RDWR | os.O_NONBLOCK)
        self.addCleanup(os.close, fd)
        st = os.fstat(fd)
        self.assertTrue(stat.S_ISCHR(st.st_mode))
        self.assertEqual(stat.S_IMODE(st.st_mode), 0o666)
        self.assertEqual((os.major(st.st_rdev), os.minor(st.st_rdev)), (1, 7))
        path_st = os.stat("/dev/full")
        self.assertTrue(stat.S_ISCHR(path_st.st_mode))
        self.assertEqual(stat.S_IMODE(path_st.st_mode), 0o666)
        self.assertEqual((os.major(path_st.st_rdev), os.minor(path_st.st_rdev)), (1, 7))
        self.assertIn("full", os.listdir("/dev"))

        path_fd = os.open("/dev/full", os.O_PATH)
        self.addCleanup(os.close, path_fd)
        self.assert_oserror(errno.EBADF, lambda: os.read(path_fd, 1))

        duplicate = os.dup(fd)
        self.addCleanup(os.close, duplicate)
        self.assertEqual(fcntl.fcntl(duplicate, fcntl.F_GETFL) & os.O_ACCMODE, os.O_RDWR)
        self.assert_oserror(errno.ENOSPC, lambda: os.write(duplicate, b"x"))


if __name__ == "__main__":
    unittest.main()
