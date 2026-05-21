# Syscall / hard-blocker triage for LTP expansion (2026-05-21)

Scope: read-only triage for the first expanded LTP batches and known full-sweep blockers. This report does not propose fake PASS, hardcoded case success, or failure hiding. Any promoted case still needs a real run on both `la` and `rv`, both libc variants.

## Evidence snapshot

Current default evaluator outputs are stable for the 16-case core list:

- `python3 scripts/ltp_summary.py output_la.md` -> `PASS LTP CASE: 32`, `FAIL LTP CASE: 0`, internal `TCONF: 2`, ENOSYS `0`.
- `python3 scripts/ltp_summary.py output_rv.md` -> `PASS LTP CASE: 32`, `FAIL LTP CASE: 0`, internal `TCONF: 2`, ENOSYS `0`.
- The only current internal LTP marker in the core run is `chdir01.c:122: TCONF: Skipping symlink loop test, not supported` in both libc runs and both arches (`output_la.md:1767`, `output_la.md:2446`, `output_rv.md:1278`, `output_rv.md:1957`).
- Older full-LTP attempt: RV started 242 cases and stopped after `cve-2017-17052` / `cve-2017-17053`; LA started 220 cases and stopped at `crash01` (`eval-reports/full-ltp-20260519-132237/full-ltp-report.md:13-14`).

## Risk-ranked low-risk syscall / ABI wins

### P0: `symlinkat` / symlink metadata tracking

Why likely high score value:
- Current core `chdir01` is green but still has a symlink-loop `TCONF` on all variants.
- Older full-LTP shows early cross-arch blockers caused by `symlink(... ) failed: ENOSYS`: `access02` on LA/RV and `chroot03` on LA/RV (`la.full-ltp.output.md:488`, `rv.full-ltp.output.md:549`, `la.full-ltp.output.md:2484`, `rv.full-ltp.output.md:2473`).

Likely targets:
- `examples/shell/src/uspace/syscall_dispatch.rs`: add dispatch for `general::__NR_symlinkat` near other path syscalls (`openat` / `mkdirat` / `unlinkat` around lines 101-105).
- `examples/shell/src/uspace/metadata.rs`: add `sys_symlinkat` and extend `sys_readlinkat`; current `sys_readlinkat` only handles `/proc/self/exe` and returns `EINVAL` for real files (`metadata.rs:544-575`).
- `examples/shell/src/uspace/mod.rs` / `process_lifecycle.rs`: if symlinks are represented synthetically, add a per-process `BTreeMap<String, String>` copied on fork like `path_modes` / `path_owners` (`mod.rs:72-74`, `process_lifecycle.rs:471-473`).
- `examples/shell/src/uspace/fd_table.rs`: path resolution needs a bounded symlink-follow path for `stat/open/chdir/access` if the target should behave as Linux, not just `readlinkat`.

Risk: medium. Creating/readlinking symlinks is low risk; full path-follow semantics and symlink-loop detection can affect many path syscalls. Recommended staged approach: first support synthetic `symlinkat` + `readlinkat`, then bounded path-follow with `ELOOP` for the `chdir01` loop variant.

### P1: `/proc/config.gz` or kernel config stub for config-gated cases

Why likely useful:
- Older full-LTP repeatedly broke on `tst_kconfig.c:207: TBROK: Cannot parse kernel .config` on both arches (`la.full-ltp.output.md:588`, `la.full-ltp.output.md:1089`, `la.full-ltp.output.md:1284`; `rv.full-ltp.output.md:645`, `rv.full-ltp.output.md:1113`, `rv.full-ltp.output.md:1366`). These are not real syscall failures but prevent many cases from clean `TCONF`/skip classification.

Likely targets:
- `examples/shell/src/uspace/synthetic_fs.rs`: add synthetic file content for `/proc/config.gz` and/or `/proc/config`.
- `examples/shell/src/uspace/fd_table.rs` / `metadata.rs`: route synthetic proc config paths similarly to existing `/proc/self/maps`, `/proc/<pid>/stat`, and user database synthetic files.

Risk: low-to-medium. Safe if represented honestly as a minimal config exposing unsupported features as disabled; risky if it claims features the kernel does not implement. Do not mark capabilities/cgroups/network modules enabled unless real support exists.

### P1: user/group database lookup path

Why likely useful:
- Older full-LTP has chmod/creat failures from NSS/userdb lookup: `Group ID lookup failed: ENOTSOCK` (`la.full-ltp.output.md:2157`, `rv.full-ltp.output.md:2161`, `rv.full-ltp.output.md:4386`).
- Synthetic `/etc/passwd` and `/etc/group` already exist in `synthetic_fs.rs` (`DEFAULT_PASSWD_CONTENT`, `DEFAULT_GROUP_CONTENT` routes), so failures may be path-routing or libc NSS behavior rather than missing data.

Likely targets:
- `examples/shell/src/uspace/synthetic_fs.rs`: verify contents include expected `daemon` / `users` groups used by LTP.
- `examples/shell/src/uspace/fd_socket.rs` or socket bridge path: ENOTSOCK suggests libc may be touching NSS service mechanisms or trying paths incorrectly; confirm `openat` and `stat` route `/etc/group` to synthetic memory files before treating this as a socket issue.

Risk: low if only adding realistic default group/passwd entries or fixing path routing; medium if changing socket errno behavior.

### P2: `chroot` errno / restricted semantics

Evidence:
- `chroot01` expected `EPERM` for unprivileged chroot but got `ENOSYS` on both arches (`la.full-ltp.output.md:2438`, `rv.full-ltp.output.md:2429`).
- `chroot02` expected successful root change and later test result but got `ENOSYS`/TBROK (`la.full-ltp.output.md:2459`, `rv.full-ltp.output.md:2449`).
- `chroot04` expected `EACCES` but got `ENOSYS` (`la.full-ltp.output.md:2506`, `rv.full-ltp.output.md:2494`).

Likely targets:
- `examples/shell/src/uspace/syscall_dispatch.rs`: dispatch `general::__NR_chroot` (currently falls through to `_ => ENOSYS` at line 404).
- `examples/shell/src/uspace/fd_table.rs` / `runtime_paths.rs` / `process_lifecycle.rs`: per-process root prefix if supporting successful chroot; otherwise a narrow errno-compatible implementation may improve negative tests only.

Risk: medium. Negative errno cases are low risk, but real chroot changes path resolution globally. Avoid fake success for positive `chroot02` unless path resolution is actually rooted.

### P2: `copy_file_range`

Evidence:
- `copy_file_range03` failed with `copy_file_range unexpectedly failed: ENOSYS` on both arches (`la.full-ltp.output.md:3495`, `rv.full-ltp.output.md:3439`).
- `copy_file_range01/02` hit test-device acquisition first (`la.full-ltp.output.md:3455`, `la.full-ltp.output.md:3473`, `rv.full-ltp.output.md:3401`, `rv.full-ltp.output.md:3418`).

Likely targets:
- `examples/shell/src/uspace/syscall_dispatch.rs`: dispatch `general::__NR_copy_file_range`.
- `examples/shell/src/uspace/fd_table.rs`: implement using existing read/write/pread/pwrite paths, preserving offsets and Linux errno constraints.

Risk: medium. Straight file-to-file copy is manageable, but Linux offset semantics and special fd cases need care.

### P2: `rt_sigsuspend`

Evidence:
- `cpuctl_fj_cpu-hog` hit `sigsuspend(): Function not implemented` on both arches (`la.full-ltp.output.md:3588`, `rv.full-ltp.output.md:3516`).

Likely targets:
- `examples/shell/src/uspace/syscall_dispatch.rs`: dispatch `general::__NR_rt_sigsuspend` near existing signal syscalls (`rt_sigtimedwait`, `rt_sigaction`, `rt_sigreturn`, `rt_sigprocmask` around lines 359-368).
- `examples/shell/src/uspace/signal_abi.rs`: implement mask swap + sleep/wakeup behavior using existing pending-signal and futex/yield primitives.

Risk: medium-high. A stub returning `EINTR` may pass some simple checks but would be semantically weak; real behavior needs signal delivery and mask restoration.

### P3: clone variants beyond current `clone01`

Evidence:
- `clone02` fails with `clone() failed: TEST_ERRNO=ENOSYS` (`la.full-ltp.output.md:3023`, `rv.full-ltp.output.md:2995`).
- `clone08` fails for `CLONE_PARENT` (`la.full-ltp.output.md:3153`, `rv.full-ltp.output.md:3118`).
- `clone03`, `clone07` have wait/return-value symptoms in nearby log chunks, so not all clone failures are pure ENOSYS.

Likely targets:
- `examples/shell/src/uspace/process_lifecycle.rs`: `sys_clone` flag validation returns ENOSYS for unsupported exit signals and clone flag combinations; expand only one flag family at a time.
- `examples/shell/src/uspace/task_context.rs`: child-return fixups, especially RV branch/PC handling.
- `examples/shell/src/uspace/process_lifecycle.rs`: `sys_wait4` semantics for pid/group values.

Risk: high for broad flag support. Do after simpler path/proc fixes. `CLONE_PARENT` can be a narrow medium-risk increment if parent/ppid semantics are understood.

## Known hard blockers for broader/full LTP

### LA `crash01`: user illegal-instruction trap still panics in old full run

Evidence:
- Full report: LA stopped at `crash01` due to unhandled `InstructionNotExist` and kernel panic (`full-ltp-report.md:13-14`, `full-ltp-report.md:30-33`).
- Old log: `Unhandled trap Exception(InstructionNotExist) @ 0x10000b3020` (`la.full-ltp.output.md:3728`).

Current source targets:
- `vendor/axcpu/src/loongarch64/trap.rs`: user `InstructionNotExist` path now maps to signal 4 (`SIGILL`) through `handle_user_signal` (`trap.rs:80-93`). The old log points at the same file but line numbers from the older revision, so this may already be partly fixed since the full-LTP attempt.
- `examples/shell/src/uspace/signal_abi.rs`: `user_exception` converts `SIGILL`/`SIGSEGV` into `128 + signal` process exit (`signal_abi.rs:152-162`).
- `examples/shell/src/uspace/process_lifecycle.rs`: ensure signal-triggered exit tears down the process and wakes waiters without leaving the kernel in a panic path.

Risk: medium-high. The first validation should be a single LA `crash01` run with the current source before editing. If it still panics, verify whether the trap arrives with `from_user=false`; that would explain why the user-signal branch is bypassed.

### RV `cve-2017-17052` / `cve-2017-17053`: full-sweep memory exhaustion

Evidence:
- Full report: RV stopped around `cve-2017-17052` / `cve-2017-17053`, hit `free_frames=0`, and could not map `/glibc/ltp_testcode.sh` (`full-ltp-report.md:18-23`).
- Old log shows repeated `COW frame allocation failed` / `frame allocation failed` with `free_frames=0` and `allocated_frames=261290` (`rv.full-ltp.output.md:22236-22263`), then glibc loader failure (`rv.full-ltp.output.md:22336-22338`).
- The same region also has many per-process `process-teardown ... reclaimed_frames=...` lines, so cleanup exists but is insufficient under stress.

Current source targets:
- `kernel/memory/axmm/src/aspace.rs`: allocation and unmap paths (`map_alloc` / `unmap`, around `aspace.rs:253-289`) and COW fault allocation in backend logs.
- `examples/shell/src/uspace/process_lifecycle.rs`: `cleanup_user_processes`, `run_user_program_in_with_timeout`, and `terminate_current_thread_inner` (`process_lifecycle.rs:102-179`, `process_lifecycle.rs:907-915`). Confirm every fork/timeout/error path calls teardown and drops child references.
- `examples/shell/src/uspace/memory_map.rs`: `sys_munmap` / deferred self-unmap (`memory_map.rs:234+`) and `forget_mmap_range` accounting.
- `examples/shell/src/uspace/sysv_shm.rs`: shared-memory attach/detach cleanup if CVE cases use SysV shm.

Risk: high. Do not try to fix this as part of first score expansion unless it blocks curated batches. First step should be a single-case RV run of `cve-2017-17052` with memory counters before/after and a smaller child limit if the test intentionally stress-forks.

## Recommended implementation order

1. Validate current-source `crash01` on LA before touching trap code; the current source appears to already route user `InstructionNotExist` to `SIGILL`.
2. Implement and validate `symlinkat` + `readlinkat` synthetic tracking. This is the best low-risk/high-signal target because it removes current core `chdir01` TCONF and unblocks early `access02` / `chroot03` failures.
3. Add honest `/proc/config.gz` or `/proc/config` synthetic disabled-feature config so config-gated tests classify correctly instead of TBROK.
4. Verify `/etc/group` / `/etc/passwd` synthetic routing and contents for `daemon` / `users`; fix before changing socket errno paths.
5. Consider narrow negative-case `chroot` errno handling; defer full successful chroot until path-rooting is real.
6. Implement `copy_file_range` only for regular files after the path/syscall batch is stable.
7. Defer clone-family expansion and RV CVE/OOM cleanup to separate hard-blocker work, because broad changes can destabilize process lifecycle and memory management.

## Validation commands used for this report

- `python3 scripts/ltp_summary.py output_la.md`
- `python3 scripts/ltp_summary.py output_rv.md`
- `rg --text -n "PASS LTP CASE|FAIL LTP CASE|TFAIL|TBROK|TCONF|ENOSYS|not implemented" output_la.md output_rv.md`
- `rg --text -n "symlink\(|brk\(\) not implemented|chroot\(|clone\(\)|copy_file_range|sigsuspend|Cannot parse kernel \.config|Group ID lookup|Failed to acquire device|InstructionNotExist|free_frames=0|failed to map ELF segment" eval-reports/full-ltp-20260519-132237/la.full-ltp.output.md eval-reports/full-ltp-20260519-132237/rv.full-ltp.output.md`
- `grep -R` / `nl -ba` source inspections of the file/function targets listed above.

Changed source files: none. Changed report files: `docs/ltp-score-improvement-2026-05-21/syscall-hardblocker-triage.md`.
