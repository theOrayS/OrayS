# readlinkat02 LA-musl diagnostic report

Date: 2026-05-26

## Outcome

`readlinkat02` remains **blocked** and must not be promoted. The fresh LA single-case diagnostic reproduces the existing split: LA glibc is clean, while LA musl still has one internal `TFAIL` and wrapper FAIL. No timeout, ENOSYS, panic, or trap was observed.

## Key evidence

- Raw log: `raw/readlinkat02-la-diagnostic-003.log` (local ignored raw evidence, not intended for commit).
- Summary: `raw/readlinkat02-la-diagnostic-003-summary.txt`.
- Parser result: LA `ltp-musl` PASS 0 / FAIL 1 with TFAIL 1; LA `ltp-glibc` PASS 1 / FAIL 0.
- Upstream/contest LTP source shape for 20240524: `tcases[]` first entry passes `bufsiz = 0` and expects `EINVAL` for `readlinkat(dir_fd, symlink_file, buf, 0)`: https://raw.githubusercontent.com/linux-test-project/ltp/20240524/testcases/kernel/syscalls/readlinkat/readlinkat02.c

## Diagnostic excerpt

```text
readlinkat-dbg: path='symlink_file' dirfd=3 pathname=0x120022098 buf=0x3efffffa88 bufsiz=1
readlinkat-dbg: symlink resolved='/tmp/ltp-work/LTP_reaGPoeFI/symlink_file' target='test_file' target_len=9 copy_len=1 buf=0x3efffffa88 bufsiz=1
readlinkat02.c:56: TFAIL: readlinkat(3, symlink_file, NULL, 0) succeeded
readlinkat02.c:56: TPASS: readlinkat(3, test_file, NULL, 256) : EINVAL (22)
readlinkat-dbg: path='symlink_file' dirfd=4 pathname=0x120022098 buf=0x3efffffac0 bufsiz=256
readlinkat02.c:56: TPASS: readlinkat(4, symlink_file, NULL, 256) : ENOTDIR (20)
readlinkat02.c:56: TPASS: readlinkat(3, test_file/test_file, NULL, 256) : ENOTDIR (20)
readlinkat-dbg: path='symlink_file' dirfd=18446744073709551615 pathname=0x120022098 buf=0x3efffffac0 bufsiz=256
readlinkat02.c:56: TPASS: readlinkat(-1, symlink_file, NULL, 256) : EBADF (9)
readlinkat02.c:56: TPASS: readlinkat(3, does_not_exists, NULL, 256) : ENOENT (2)
FAIL LTP CASE readlinkat02 : 1
readlinkat-dbg: raw dirfd=3 pathname=0x34ae8 buf=0x3efffff990 bufsiz=0
readlinkat02.c:56: TPASS: readlinkat(3, symlink_file, NULL, 0) : EINVAL (22)
readlinkat02.c:56: TPASS: readlinkat(3, test_file, NULL, 256) : EINVAL (22)
readlinkat-dbg: path='symlink_file' dirfd=4 pathname=0x34ae8 buf=0x3efffff990 bufsiz=256
readlinkat02.c:56: TPASS: readlinkat(4, symlink_file, NULL, 256) : ENOTDIR (20)
readlinkat02.c:56: TPASS: readlinkat(3, test_file/test_file, NULL, 256) : ENOTDIR (20)
readlinkat-dbg: path='symlink_file' dirfd=18446744073709551615 pathname=0x34ae8 buf=0x3efffff990 bufsiz=256
readlinkat02.c:56: TPASS: readlinkat(-1, symlink_file, NULL, 256) : EBADF (9)
readlinkat02.c:56: TPASS: readlinkat(3, does_not_exists, NULL, 256) : ENOENT (2)
FAIL LTP CASE readlinkat02 : 0
```

## Interpretation

The syscall implementation already has the Linux-compatible guard:

```rust
if bufsiz == 0 {
    return neg_errno(LinuxError::EINVAL);
}
```

The diagnostic proves the LA-musl failing call did **not** reach the kernel as `bufsiz == 0`; it reached `sys_readlinkat` with `bufsiz=1`, copied one byte from target `test_file`, and therefore succeeded. LA glibc reaches the same testcase with `bufsiz=0` and gets `EINVAL` as expected.

A kernel change that forces `EINVAL` for `bufsiz == 1` on symlinks would violate Linux/POSIX-visible `readlinkat(2)` behavior and would be test-specific laundering. Therefore this case is a blocker/skip-for-now, not a promotion candidate.

## Next step

Move away from `readlinkat02` for the stable400 push unless a musl/LA wrapper or ABI argument-passing root cause is found outside the syscall body. Continue searching for fresh four-way-clean cases from less blocked lanes, or diagnose the LA-musl userspace/libc call boundary separately.
