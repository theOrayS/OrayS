#!/usr/bin/env python3
"""Unit tests for the G004 rlimit/sysconf/fcntl/FD static guard."""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))

import check_g004_rlimit_fd as guard


class G004RlimitFdGuardTest(unittest.TestCase):
    def test_sysconf_catch_all_ok_zero_is_flagged(self) -> None:
        findings = guard.scan_sysconf(
            Path("api/arceos_posix_api/src/imp/sys.rs"),
            r'''
pub fn sys_sysconf(name: c_int) -> c_long {
    syscall_body!(sys_sysconf, {
        match name as u32 {
            ctypes::_SC_PAGE_SIZE => Ok(PAGE_SIZE_4K),
            _ => Ok(0),
        }
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "g004-sysconf-fallback-ok-zero")

    def test_sysconf_errno_fallback_is_allowed(self) -> None:
        findings = guard.scan_sysconf(
            Path("api/arceos_posix_api/src/imp/sys.rs"),
            r'''
pub fn sys_sysconf(name: c_int) -> c_long {
    syscall_body!(sys_sysconf, {
        match name as u32 {
            ctypes::_SC_PAGE_SIZE => Ok(PAGE_SIZE_4K),
            _ => Err(LinuxError::EINVAL),
        }
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(findings, [])

    def test_api_rlimit_unsupported_ok_zero_is_flagged(self) -> None:
        findings = guard.scan_api_resources(
            Path("api/arceos_posix_api/src/imp/resources.rs"),
            r'''
pub unsafe fn sys_getrlimit(resource: c_int, rlimits: *mut ctypes::rlimit) -> c_int {
    syscall_body!(sys_getrlimit, {
        match resource as u32 {
            ctypes::RLIMIT_STACK => {}
            _ => return Err::<c_int, LinuxError>(LinuxError::EINVAL),
        }
        if rlimits.is_null() { return Err(LinuxError::EFAULT); }
        Ok(0)
    })
}

pub unsafe fn sys_setrlimit(resource: c_int, rlimits: *mut ctypes::rlimit) -> c_int {
    syscall_body!(sys_setrlimit, {
        match resource as u32 {
            ctypes::RLIMIT_STACK => {}
            _ => return Err::<c_int, LinuxError>(LinuxError::EINVAL),
        }
        if rlimits.is_null() { return Err(LinuxError::EFAULT); }
        // Currently do not support changing resource limits.
        Ok(0)
    })
}
''',
            root=Path("."),
        )

        self.assertTrue(any(f.kind == "g004-rlimit-unsupported-ok-zero" for f in findings))

    def test_api_rlimit_errno_paths_are_allowed(self) -> None:
        findings = guard.scan_api_resources(
            Path("api/arceos_posix_api/src/imp/resources.rs"),
            r'''
pub unsafe fn sys_getrlimit(resource: c_int, rlimits: *mut ctypes::rlimit) -> c_int {
    syscall_body!(sys_getrlimit, {
        match resource as u32 {
            ctypes::RLIMIT_STACK => {}
            _ => return Err::<c_int, LinuxError>(LinuxError::EINVAL),
        }
        if rlimits.is_null() { return Err(LinuxError::EFAULT); }
        Err::<c_int, LinuxError>(LinuxError::ENOSYS)
    })
}

pub unsafe fn sys_setrlimit(resource: c_int, rlimits: *mut ctypes::rlimit) -> c_int {
    syscall_body!(sys_setrlimit, {
        match resource as u32 {
            ctypes::RLIMIT_STACK => {}
            _ => return Err::<c_int, LinuxError>(LinuxError::EINVAL),
        }
        if rlimits.is_null() { return Err(LinuxError::EFAULT); }
        Err::<c_int, LinuxError>(LinuxError::ENOSYS)
    })
}
''',
            root=Path("."),
        )

        self.assertEqual(findings, [])

    def test_api_filelike_default_status_ok_zero_is_flagged(self) -> None:
        findings = guard.scan_api_fd_ops(
            Path("api/arceos_posix_api/src/imp/fd_ops.rs"),
            r'''
pub trait FileLike {
    fn status_flags(&self) -> LinuxResult<c_int> {
        Ok(0)
    }
}

pub fn sys_fcntl(fd: c_int, cmd: c_int, arg: usize) -> c_int {
    syscall_body!(sys_fcntl, {
        match cmd as u32 {
            ctypes::F_GETFD => Ok(0),
            ctypes::F_SETFD => Ok(0),
            _ => Err(LinuxError::EINVAL),
        }
    })
}
''',
            root=Path("."),
        )

        self.assertTrue(any(f.kind == "g004-api-filelike-default-status-ok-zero" for f in findings))

    def test_api_fcntl_missing_fd_commands_is_flagged(self) -> None:
        findings = guard.scan_api_fd_ops(
            Path("api/arceos_posix_api/src/imp/fd_ops.rs"),
            r'''
pub trait FileLike {
    fn status_flags(&self) -> LinuxResult<c_int>;
}

pub fn sys_fcntl(fd: c_int, cmd: c_int, arg: usize) -> c_int {
    syscall_body!(sys_fcntl, {
        match cmd as u32 {
            ctypes::F_GETFL => get_file_like(fd)?.status_flags(),
            _ => Err(LinuxError::EINVAL),
        }
    })
}
''',
            root=Path("."),
        )

        kinds = {f.kind for f in findings}
        self.assertIn("g004-api-fcntl-missing-fd-command", kinds)

    def test_api_dupfd_cloexec_without_flag_is_flagged(self) -> None:
        findings = guard.scan_api_fd_ops(
            Path("api/arceos_posix_api/src/imp/fd_ops.rs"),
            r'''
pub trait FileLike {
    fn status_flags(&self) -> LinuxResult<c_int>;
}

pub fn sys_fcntl(fd: c_int, cmd: c_int, arg: usize) -> c_int {
    syscall_body!(sys_fcntl, {
        match cmd as u32 {
            ctypes::F_DUPFD_CLOEXEC => {
                // TODO: Change fd flags
                dup_fd(fd)
            }
            ctypes::F_GETFD => get_fd_flags(fd),
            ctypes::F_SETFD => set_fd_flags(fd, arg as u32),
            _ => Err(LinuxError::EINVAL),
        }
    })
}
''',
            root=Path("."),
        )

        self.assertTrue(any(f.kind == "g004-api-dupfd-cloexec-drops-flag" for f in findings))

    def test_shell_prlimit_invalid_resource_success_is_flagged(self) -> None:
        findings = guard.scan_shell_resource(
            Path("examples/shell/src/uspace/resource_sched.rs"),
            r'''
pub(super) fn sys_prlimit64(process: &UserProcess, pid: i32, resource: u32, new_limit: usize, old_limit: usize) -> isize {
    if !resource_is_valid(resource) {
        return 0;
    }
    0
}
''',
            root=Path("."),
        )

        self.assertTrue(any(f.kind == "g004-prlimit-invalid-resource-success" for f in findings))

    def test_shell_prlimit_einval_is_allowed(self) -> None:
        findings = guard.scan_shell_resource(
            Path("examples/shell/src/uspace/resource_sched.rs"),
            r'''
pub(super) fn sys_prlimit64(process: &UserProcess, pid: i32, resource: u32, new_limit: usize, old_limit: usize) -> isize {
    if !resource_is_valid(resource) {
        return neg_errno(LinuxError::EINVAL);
    }
    0
}
''',
            root=Path("."),
        )

        self.assertEqual(findings, [])

    def test_shell_fcntl_ok_zero_fallback_is_flagged(self) -> None:
        findings = guard.scan_shell_fd_table(
            Path("examples/shell/src/uspace/fd_table.rs"),
            r'''
pub(super) enum FdEntry {
    Stdin(u32),
    Stdout(u32),
    Stderr(u32),
}
pub(super) struct FdTable {
    fd_flags: Vec<u32>,
}
impl FdTable {
    pub(super) fn insert_with_flags(&mut self, entry: FdEntry, fd_flags: u32) -> Result<i32, LinuxError> { Ok(0) }
    pub(super) fn get_fd_flags(&self, fd: i32) -> Result<i32, LinuxError> { Ok(0) }
    pub(super) fn set_fd_flags(&mut self, fd: i32, flags: u32) -> Result<i32, LinuxError> { Ok(0) }
    pub(super) fn fcntl(&mut self, process: &UserProcess, fd: i32, cmd: u32, arg: usize) -> Result<i32, LinuxError> {
        match cmd {
            general::F_GETFD => self.get_fd_flags(fd),
            general::F_SETFD => self.set_fd_flags(fd, arg as u32),
            _ => Ok(0),
        }
    }
}
''',
            root=Path("."),
        )

        self.assertTrue(any(f.kind == "g004-shell-fcntl-fallback-ok-zero" for f in findings))

    def test_shell_fcntl_fd_state_and_errno_fallback_are_allowed(self) -> None:
        findings = guard.scan_shell_fd_table(
            Path("examples/shell/src/uspace/fd_table.rs"),
            r'''
pub(super) enum FdEntry {
    Stdin(u32),
    Stdout(u32),
    Stderr(u32),
}
pub(super) struct FdTable {
    fd_flags: Vec<u32>,
}
impl FdTable {
    pub(super) fn insert_with_flags(&mut self, entry: FdEntry, fd_flags: u32) -> Result<i32, LinuxError> { Ok(0) }
    pub(super) fn get_fd_flags(&self, fd: i32) -> Result<i32, LinuxError> { Ok(0) }
    pub(super) fn set_fd_flags(&mut self, fd: i32, flags: u32) -> Result<i32, LinuxError> { Ok(0) }
    pub(super) fn fcntl(&mut self, process: &UserProcess, fd: i32, cmd: u32, arg: usize) -> Result<i32, LinuxError> {
        match cmd {
            general::F_GETFD => self.get_fd_flags(fd),
            general::F_SETFD => self.set_fd_flags(fd, arg as u32),
            general::F_GETFL => match self.entry(fd)? {
                FdEntry::Stdin(status_flags)
                | FdEntry::Stdout(status_flags)
                | FdEntry::Stderr(status_flags) => Ok(*status_flags as i32),
                _ => Err(LinuxError::EINVAL),
            },
            general::F_SETFL => match self.entry_mut(fd)? {
                FdEntry::Stdin(status_flags)
                | FdEntry::Stdout(status_flags)
                | FdEntry::Stderr(status_flags) => {
                    *status_flags =
                        (*status_flags & general::O_ACCMODE) | fcntl_setfl_flags(arg as u32);
                    Ok(0)
                }
                _ => Err(LinuxError::EINVAL),
            },
            _ => Err(LinuxError::EINVAL),
        }
    }
}
''',
            root=Path("."),
        )

        self.assertEqual(findings, [])


if __name__ == "__main__":
    unittest.main()
