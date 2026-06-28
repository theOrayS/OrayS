# Syscall/ABI triage from prior blockers

Owner: worker-5  
Date: 2026-05-21 UTC  
Scope: task-5 syscall/ABI lane. This report is evidence-first and intentionally makes **no source patch** because the current available logs do not show a fresh targeted syscall/ABI failure outside the already-green stable set. No `.omx/ultragoal` files were read or edited.

## Decision

Do **not** attempt a syscall/ABI code change in this lane yet.

Current `output_rv.md` and `output_la.md` are already a 63-case-per-libc stable run, not the older 16-core snapshot: both summaries report `PASS LTP CASE: 126`, `FAIL LTP CASE: 0`, internal `TCONF: 4`, `ENOSYS/not implemented matches: 0`, and `panic/trap matches: 0`. The previous symlink/readlink/path-metadata surface is therefore validated for the promoted stable set. Remaining prior blockers need targeted reproduction before any real fix is safe.

## Evidence reviewed

### Required prior reports

- `docs/ltp-score-improvement-2026-05-22/syscall-abi-candidate-report.md` concluded that the prior low-risk symlink/readlink/path-metadata recommendations had already landed, and that further ABI edits needed fresh targeted failures.
- `docs/ltp-score-improvement-2026-05-22/hard-blocker-report.md` separates stable/core promotion from hard blockers: RV CVE/OOM memory pressure and LA `crash01` trap must be validated as independent implementation lanes.

### Current top-level output summaries

Commands run:

```sh
python3 scripts/ltp_summary.py output_rv.md | sed -n '1,120p'
python3 scripts/ltp_summary.py output_la.md | sed -n '1,120p'
```

Observed summary:

| Output | PASS LTP CASE | FAIL LTP CASE | Internal markers | ENOSYS/not implemented | panic/trap |
| --- | ---: | ---: | --- | ---: | ---: |
| `output_rv.md` | 126 | 0 | `TCONF: 4` | 0 | 0 |
| `output_la.md` | 126 | 0 | `TCONF: 4` | 0 | 0 |

The stable source list currently contains 63 cases in `examples/shell/src/cmd.rs:49-113`; both architectures run the list for musl and glibc, yielding 126 PASS markers per arch.

### Relevant source facts

- `examples/shell/src/cmd.rs:49-113` includes stable cases through `sched_yield01`, including `symlink01`, `readlink01`, `lstat01`, `chmod01`, `fchmod01`, `time01`, `kill03`, `rt_sigaction01`, `sigaction01`, `proc01`, and exit/process basics.
- `examples/shell/src/uspace/syscall_dispatch.rs:106,162-164` dispatches `symlinkat` and `readlinkat`; `metadata.rs:81-103,206-230,688-725` stores symlinks, resolves bounded symlink chains, and implements `readlinkat` for synthetic symlinks.
- `examples/shell/src/uspace/linux_abi.rs:97-99` still exposes only `root`, `nobody`, and `nogroup` user/group content; prior full-LTP `chmod05/chmod07/creat08` failures mention `users`/`daemon` group lookup, but current top-level logs do not include those cases.
- `examples/shell/src/uspace/syscall_dispatch.rs:360-369` handles `rt_sigtimedwait`, `rt_sigaction`, `rt_sigreturn`, and `rt_sigprocmask`; there is no adjacent `rt_sigsuspend` dispatch even though `sigsuspend01` is listed in `LTP_TIME_SIGNAL_BASIC_CASES` at `examples/shell/src/cmd.rs:160-180`.
- The fallback `_ => ENOSYS` remains visible at `examples/shell/src/uspace/syscall_dispatch.rs:405`, so missing syscall candidates should still produce honest ENOSYS evidence when reproduced.

## Candidate fixes, gated by required fresh logs

| Priority | Candidate real fix | Why it is plausible | Required fresh log before patch | Expected failure signature if still present | Likely files |
| --- | --- | --- | --- | --- | --- |
| P0 validation-only | Keep `stable` 63-case set promoted | Current RV/LA summaries are green with 126 PASS each and zero FAIL/ENOSYS/panic. | No patch; preserve current stable evidence and avoid unrelated edits. | Any regression would show `FAIL LTP CASE`, internal `TFAIL/TBROK`, ENOSYS, or panic in `output_{rv,la}.md`. | `examples/shell/src/cmd.rs` only if stable membership changes. |
| P1 | User/group database expansion for `chmod05`, `chmod07`, `creat08` | Prior full-LTP saw `Group ID lookup failed: ENOTSOCK`; current synthetic `/etc/group` lacks `users` and `daemon`. | Run those cases on RV+LA, musl+glibc, and confirm libc is reading userdb/NSS paths rather than failing on unrelated sockets. | `Group ID lookup failed`, `getgrnam(users)`, `daemon`, `ENOTSOCK`, or attempted `/etc/nsswitch.conf`/nscd socket access. | `linux_abi.rs`, `synthetic_fs.rs`, possibly socket path handling if ENOTSOCK persists. |
| P1 | Honest synthetic kernel config for config-gated cases | Prior full-LTP repeatedly hit `Cannot parse kernel .config`; this blocks clean `TCONF` classification. | Run one or two known config-gated cases and confirm current source still lacks parseable `/proc/config.gz` or equivalent. | `tst_kconfig.c:* TBROK: Cannot parse kernel .config`. | `synthetic_fs.rs`, `fd_table.rs`, `metadata.rs`. |
| P2 | `rt_sigsuspend` minimal signal-wait semantics | Prior broad logs saw `sigsuspend(): Function not implemented`; current source lists `sigsuspend01` in a batch but does not dispatch `__NR_rt_sigsuspend`. | Run `LTP_CASES='sigsuspend01'` or `LTP_CASES='time-signal-basic'` on RV+LA and confirm ENOSYS or semantic failure. | `sigsuspend(): Function not implemented`, `__NR_rt_sigsuspend`, `ENOSYS`, timeout while waiting for a signal. | `syscall_dispatch.rs`, `signal_abi.rs`, perhaps `time_abi.rs` for interruptible sleep/wakeup behavior. |
| P2 | `chroot` errno/namespace semantics | Prior full-LTP chroot cases got ENOSYS; real semantics touch path resolution and current root/cwd. | Reproduce `chroot01-04` and separate expected `EPERM`, path errors, and actual root-change requirements. | `chroot()` got `ENOSYS`, expected `EPERM`/`EACCES`, or post-chroot path lookup mismatch. | `syscall_dispatch.rs`, `fd_table.rs`, `runtime_paths.rs`, `process_lifecycle.rs`. |
| P2 | `copy_file_range` for regular files only | Prior `copy_file_range03` saw ENOSYS; regular-file copy can be bounded but offset semantics are easy to corrupt. | Reproduce `copy_file_range01-03` and confirm device-acquisition blockers are not the first failure. | `copy_file_range unexpectedly failed: ENOSYS`, wrong offset/file-size behavior, or test-device TBROK. | `syscall_dispatch.rs`, file/fd table helpers. |
| P3 hard blocker | RV CVE/OOM and LA `crash01` | These are real hard blockers but outside low-risk syscall/ABI promotion. | Isolated current-source runs only: RV `cve-2017-17052,cve-2017-17053`; LA `crash01`. | RV `free_frames=0`, COW/fresh allocation failures, ELF map failures; LA `InstructionNotExist`/Unhandled trap. | memory/fork/trap/process lifecycle files, not this low-risk ABI lane. |

## Recommended targeted commands

Run these before any follow-up implementation worker edits source:

```sh
# User/group database candidates
LTP_CASES='chmod05,chmod07,creat08' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh
python3 scripts/ltp_summary.py output_rv.md
LTP_CASES='chmod05,chmod07,creat08' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh la
python3 scripts/ltp_summary.py output_la.md

# Signal syscall candidate
LTP_CASES='sigsuspend01' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh
LTP_CASES='sigsuspend01' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh la

# Config-gated classification probe: choose the smallest cases from the discovery list that used tst_kconfig.
# Patch only after the log still says Cannot parse kernel .config.

# Higher-risk candidates should stay isolated from stable promotion.
LTP_CASES='chroot01,chroot02,chroot03,chroot04' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh
LTP_CASES='copy_file_range01,copy_file_range02,copy_file_range03' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh
```

## Blockers and expected signatures

- There is no fresh failing targeted log for the P1/P2 candidates in this worktree. The current visible logs are green stable outputs.
- A code change now would be speculative because it would rely on old broad full-LTP signatures rather than current isolated failures.
- The first safe next task is targeted validation, not mutation: collect RV+LA and musl+glibc evidence for the candidate cases, then assign a narrow implementation lane with one file-owner surface.

## Verification performed for this report

```sh
python3 scripts/ltp_summary.py output_rv.md | sed -n '1,120p'
python3 scripts/ltp_summary.py output_la.md | sed -n '1,120p'
rg --text -n 'ENOSYS|not implemented|sigsuspend|rt_sigsuspend|Cannot parse kernel \.config|Group ID lookup|getgrnam|symlink\(|readlink|chroot\(|copy_file_range|FAIL LTP CASE|TBROK|TFAIL|TCONF|InstructionNotExist|free_frames=0' output_rv.md output_la.md docs/ltp-score-improvement-2026-05-22/*.md docs/ltp-score-improvement-2026-05-21/syscall-hardblocker-triage.md docs/ltp-score-improvement-2026-05-21/discovery-candidates.md
rg -n 'rt_sigsuspend|sigsuspend|copy_file_range|chroot|symlinkat|readlinkat|path_symlinks|nsswitch|/etc/group|/etc/passwd|proc/config|config.gz|__NR_|ENOSYS|not implemented' examples/shell/src/uspace examples/shell/src/cmd.rs
```

## Stop condition

This lane is complete when the team has a current-source syscall/ABI triage report that names 3-6 plausible fixes, specifies the exact log signatures required before attempting them, and avoids both fake PASS behavior and speculative source edits. This document satisfies that stop condition.
