#!/usr/bin/env python3
"""Unit tests for the G003 stat/lstat metadata static guard."""

from __future__ import annotations

from pathlib import Path
import unittest

import check_g003_stat_metadata as guard


class G003StatMetadataGuardTest(unittest.TestCase):
    def test_stat_via_read_open_is_flagged(self) -> None:
        findings = guard.scan_fs_rs(
            Path("api/arceos_posix_api/src/imp/fs.rs"),
            r'''
pub unsafe fn sys_stat(path: *const c_char, buf: *mut ctypes::stat) -> c_int {
    syscall_body!(sys_stat, {
        let mut options = OpenOptions::new();
        options.read(true);
        let file = axfs::fops::File::open(path?, &options)?;
        let st = File::new(file).stat()?;
        unsafe { write_stat_output(buf, st) };
        Ok(0)
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "g003-stat-via-read-open")

    def test_stat_real_metadata_lookup_is_allowed(self) -> None:
        findings = guard.scan_fs_rs(
            Path("api/arceos_posix_api/src/imp/fs.rs"),
            r'''
pub unsafe fn sys_stat(path: *const c_char, buf: *mut ctypes::stat) -> c_int {
    syscall_body!(sys_stat, {
        let st = api_metadata_to_stat(axfs::api::metadata(path?)?);
        unsafe { write_stat_output(buf, st) };
        Ok(0)
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(findings, [])

    def test_default_stat_success_path_is_flagged(self) -> None:
        findings = guard.scan_fs_rs(
            Path("api/arceos_posix_api/src/imp/fs.rs"),
            r'''
pub unsafe fn sys_stat(path: *const c_char, buf: *mut ctypes::stat) -> c_int {
    syscall_body!(sys_stat, {
        let st = ctypes::stat::default();
        unsafe { write_stat_output(buf, st) };
        Ok(0)
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "g003-default-stat-success-risk")

    def test_lstat_delegating_to_stat_is_flagged(self) -> None:
        findings = guard.scan_fs_rs(
            Path("api/arceos_posix_api/src/imp/fs.rs"),
            r'''
pub unsafe fn sys_lstat(path: *const c_char, buf: *mut ctypes::stat) -> ctypes::ssize_t {
    sys_stat(path, buf)
}
''',
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "g003-lstat-delegates-to-stat")

    def test_lstat_plain_metadata_without_boundary_is_flagged(self) -> None:
        findings = guard.scan_fs_rs(
            Path("api/arceos_posix_api/src/imp/fs.rs"),
            r'''
pub unsafe fn sys_lstat(path: *const c_char, buf: *mut ctypes::stat) -> ctypes::ssize_t {
    syscall_body!(sys_lstat, {
        let st = api_metadata_to_stat(axfs::api::metadata(path?)?);
        unsafe { write_stat_output(buf, st) };
        Ok(0)
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "g003-lstat-without-nofollow-boundary")

    def test_lstat_honest_unsupported_boundary_is_allowed(self) -> None:
        findings = guard.scan_fs_rs(
            Path("api/arceos_posix_api/src/imp/fs.rs"),
            r'''
pub unsafe fn sys_lstat(path: *const c_char, buf: *mut ctypes::stat) -> ctypes::ssize_t {
    syscall_body!(sys_lstat, {
        if buf.is_null() {
            return Err(LinuxError::EFAULT);
        }
        // axfs has no symlink no-follow metadata boundary here yet.
        Err(LinuxError::ENOSYS)
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(findings, [])

    def test_lstat_real_nofollow_metadata_is_allowed(self) -> None:
        findings = guard.scan_fs_rs(
            Path("api/arceos_posix_api/src/imp/fs.rs"),
            r'''
pub unsafe fn sys_lstat(path: *const c_char, buf: *mut ctypes::stat) -> ctypes::ssize_t {
    syscall_body!(sys_lstat, {
        let st = api_metadata_to_stat(axfs::api::symlink_metadata(path?)?);
        unsafe { write_stat_output(buf, st) };
        Ok(0)
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(findings, [])


if __name__ == "__main__":
    unittest.main()
