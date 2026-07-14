#!/usr/bin/env python3
"""Unit tests for the fake-success static guard."""

from __future__ import annotations

import unittest
from pathlib import Path
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "checks"))

import check_no_fake_success as guard

REPO_ROOT = Path(__file__).resolve().parents[2]


class NoFakeSuccessGuardTest(unittest.TestCase):
    def test_current_repository_has_complete_inputs_and_passes(self) -> None:
        self.assertEqual(guard.scan_repo(REPO_ROOT), [])

    def test_empty_repository_cannot_pass(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            findings = guard.scan_repo(Path(tmp))

        self.assertEqual(len(findings), 3)
        self.assertEqual(
            {finding.kind for finding in findings},
            {"required-source-set-empty", "required-source-file-missing"},
        )

    def test_missing_repository_root_cannot_pass(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            missing = Path(tmp) / "does-not-exist"
            findings = guard.scan_repo(missing)

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "repository-root-missing")

    def test_c_unimplemented_return_zero_is_flagged(self) -> None:
        findings = guard.scan_c_unimplemented_return_success(
            Path("ulib/axlibc/c/demo.c"),
            """
int truncate(const char *path, off_t length)
{
    unimplemented();
    return 0;
}
""",
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "axlibc-unimplemented-return-zero")

    def test_c_unimplemented_errno_failure_is_allowed(self) -> None:
        findings = guard.scan_c_unimplemented_return_success(
            Path("ulib/axlibc/c/demo.c"),
            """
int truncate(const char *path, off_t length)
{
    errno = ENOSYS;
    unimplemented();
    return -1;
}
""",
            root=Path("."),
        )

        self.assertEqual(findings, [])

    def test_api_catch_all_ok_zero_is_flagged(self) -> None:
        findings = guard.scan_api_fake_success(
            Path("api/arceos_posix_api/src/imp/sys.rs"),
            """
match name as u32 {
    ctypes::_SC_PAGE_SIZE => Ok(PAGE_SIZE_4K),
    _ => Ok(0),
}
""",
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "api-fallback-ok-zero")

    def test_api_unsupported_ok_zero_is_flagged(self) -> None:
        findings = guard.scan_api_fake_success(
            Path("api/arceos_posix_api/src/imp/fd_ops.rs"),
            """
warn!("unsupported fcntl parameters: cmd {}", cmd);
Ok(0)
""",
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "api-unsupported-ok-zero")

    def test_api_unsupported_branch_with_errno_is_allowed(self) -> None:
        findings = guard.scan_api_fake_success(
            Path("api/arceos_posix_api/src/imp/time.rs"),
            """
_ => {
    warn!("Called sys_clock_gettime for unsupported clock {}", clk);
    return Err(LinuxError::EINVAL);
}
Ok(0)
""",
            root=Path("."),
        )

        self.assertEqual(findings, [])

    def test_fd_table_high_risk_ok_unit_is_flagged(self) -> None:
        findings = guard.scan_fd_table_fake_success(
            Path("user/shell/src/uspace/fd_table.rs"),
            """
pub(super) fn fallocate_zero_range(&mut self) -> Result<(), LinuxError> {
    match self.entry_mut(fd)? {
        FdEntry::DevNull | FdEntry::BlockDevice(_) | FdEntry::Rtc => Ok(()),
        _ => Err(LinuxError::EINVAL),
    }
}
""",
            root=Path("."),
        )

        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].kind, "fd-table-unsupported-ok-unit")

    def test_fd_table_same_pattern_outside_high_risk_function_is_allowed(self) -> None:
        findings = guard.scan_fd_table_fake_success(
            Path("user/shell/src/uspace/fd_table.rs"),
            """
pub(super) fn close_range_for_process(&mut self) -> Result<(), LinuxError> {
    match self.entry_mut(fd)? {
        FdEntry::DevNull => Ok(()),
        _ => Err(LinuxError::EINVAL),
    }
}
""",
            root=Path("."),
        )

        self.assertEqual(findings, [])


if __name__ == "__main__":
    unittest.main()
