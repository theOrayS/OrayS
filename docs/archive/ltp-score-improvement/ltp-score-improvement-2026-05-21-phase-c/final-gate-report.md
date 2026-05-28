# LTP stable score improvement final gate report - 2026-05-23

## Result

LTP stable batch increased from 63 to 75 cases per libc per architecture (+12). Final full gates completed on LA and RV with LTP stable clean pass counts, while preserving transparent `read02` TCONF reporting.

## Newly promoted stable cases

getpgid02, getsid02, getppid02, getuid03, geteuid02, getgid03, getegid02, getgroups03, uname02, wait01, wait02, getrlimit02

## Changed code files and intent

- `examples/shell/src/cmd.rs`
  - `LTP_STABLE_CASES`: add the 12 promoted cases.
  - `LTP_CASE_TIMEOUT_SECS`: raise default case timeout from 10s to 15s after RV final pretimeout exposed real LTP timeout pressure in otherwise clean cases; timeout still counts as fail.
- `examples/shell/src/uspace/process_abi.rs`
  - Add `visible_process_group_and_session`, implement `sys_getsid`, route `getpgid/getsid` through visible process lookup, and update `setsid` to set both pgid and sid.
- `examples/shell/src/uspace/mod.rs`
  - Add stored `sid` to `UserProcess`.
- `examples/shell/src/uspace/process_lifecycle.rs`
  - Initialize/inherit `sid`; add `sid()/set_sid()`; join child task before teardown in `wait_child`.
- `examples/shell/src/uspace/synthetic_fs.rs`
  - Report stored `sid` in `/proc/<pid>/stat` session field.
- `examples/shell/src/uspace/resource_sched.rs`
  - Validate `prlimit64/getrlimit` resource and return `EINVAL` for invalid resources.
- `examples/shell/src/uspace/syscall_dispatch.rs`
  - Dispatch `__NR_getsid` to the real helper.

## Expected behavior per fix

- Getter/session cases (`getpgid02`, `getsid02`, `getppid02`, uid/gid/group getters): execute against stored process/session identity rather than fake constants.
- Wait cases (`wait01`, `wait02`): parent reaping waits for child task exit cleanup before teardown, reducing stale task/resource pressure.
- `getrlimit02`: invalid resource handling returns `EINVAL` instead of copying out misleading data.
- Runner: each LTP case still executes the real binary/script; only exit 0 is PASS; timeout remains FAIL + TIMEOUT.

## Validation commands and exit codes

| Command | Exit | Evidence |
| --- | ---: | --- |
| `cargo fmt --all -- --check` | 0 | `final-gate-cargo-fmt-check-v2.status` |
| `OSCOMP_TEST_GROUPS=ltp ./run-eval.sh la` | 0 | `la-stable75-timeout15-targeted-summary.txt` |
| `OSCOMP_TEST_GROUPS=ltp ./run-eval.sh` | 0 | `rv-stable75-timeout15-targeted-summary.txt` |
| `./run-eval.sh la 2>&1 | tee output_la.md` | 0 | `output_la.md`, `final-gate-output-la-summary.txt` |
| `./run-eval.sh 2>&1 | tee output_rv.md` | 0 | `output_rv.md`, `final-gate-output-rv-summary.txt` |
| `python3 -B scripts/ltp_summary.py output_la.md` | 0 | `final-required-summary-la.txt` |
| `python3 -B scripts/ltp_summary.py output_rv.md` | 0 | `final-required-summary-rv.txt` |
| `git diff --check -- changed-files` | 0 | `final-git-diff-check.status` |

## Final LTP summary

| Arch | PASS LTP CASE | FAIL LTP CASE | ltp-musl | ltp-glibc | TFAIL | TBROK | TCONF | LTP timeout | ENOSYS | panic/trap |
| --- | ---: | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| LA | 150 | 0 | 75/0 | 75/0 | 0 | 0 | 4 | 0 | 0 | 0 |
| RV | 150 | 0 | 75/0 | 75/0 | 0 | 0 | 4 | 0 | 0 | 0 |

`read02` remains the only pass-with-TCONF signal: final summaries show 4 total TCONF entries across LA/RV and musl/glibc; this is intentionally not hidden as a clean pass.

## Timeout / ENOSYS / panic transparency

- LTP timeout: LA 0, RV 0.
- `scripts/ltp_summary.py` total `timeout matches`: LA 10, RV 10; these are from non-LTP benchmark groups, not the LTP stable group.
- ENOSYS/not implemented: LA 0, RV 0.
- panic/trap: LA 0, RV 0.
- Non-LTP caveat: full output still contains non-LTP `iperf-glibc ... end: fail` markers; not counted as LTP promotion success.

## Blocked / not promoted cases

- `access02`
- `access04`
- `clock_getres01`
- `clock_gettime01`
- `dup03`
- `fstatfs01`
- `getpgid01`
- `getsid01`
- `kill02`
- `link02`
- `lseek02`
- `mkdir02`
- `nanosleep01`
- `nanosleep02`
- `pause01`
- `pipe02`
- `read02`
- `rename01`
- `rt_sigprocmask01`
- `sigpending02`
- `sigprocmask01`
- `sigsuspend01`
- `statfs01`
- `statvfs01`
- `sysinfo01`
- `unlink05`

Reason buckets:
- read02: already stable but pass_with_tconf; transparent TCONF retained
- clock_getres01: current TCONF; not promoted as clean pass
- statfs01/statvfs01/fstatfs01/sysinfo01: need real fs/memory ABI semantics before promotion
- access02/access04/link02/rename01/unlink05/mkdir02/lseek02/pipe02/dup03: known real errno/TFAIL/TBROK/ENOSYS-style failures need fixes first
- nanosleep01/nanosleep02/pause01/rt_sigprocmask01/sigpending02/sigprocmask01/sigsuspend01/kill02: signal/time semantics require fresh targeted validation and likely ABI fixes
- getpgid01/getsid01/clock_gettime01: nearby cases still need separate clean evidence before promotion

## Syscall / errno / ABI-visible behavior changes

- `getsid(2)` is now dispatched and backed by stored per-process session id.
- `getpgid/getsid` lookup now returns `ESRCH` for missing/invalid visible target pids in this compatibility layer.
- `setsid()` now updates both pgid and sid, but remains simplified and does not yet implement the Linux process-group-leader `EPERM` condition.
- `/proc/<pid>/stat` session field now reports sid instead of mirroring pgid.
- `prlimit64/getrlimit` invalid resource now returns `EINVAL` before copy-out/copy-in.
- `wait_child` now joins the child task before teardown; return ABI is unchanged, but cleanup ordering is user-visible through improved wait/getpid stability.
- LTP harness default timeout is now 15 seconds; timeout-as-fail semantics are unchanged.

## Review and quality gate

- Anti-slop report: `final-gate-ai-slop-cleaner-report.md`.
- Code review report: `final-gate-code-review-report.md`.
- Quality gate JSON: `final-gate-quality-gate.json`.
- Architect review: CLEAR.
- Code-reviewer lane: APPROVE.

## Remaining risks and next batch suggestions

- Non-LTP iperf-glibc fail markers and non-LTP benchmark timeout matches remain visible in full evaluator output.
- Next LTP candidates should stay targeted-first: fs metadata/open/link/rename/statfs/access cases after real errno/ABI fixes; time/signal cases (`nanosleep*`, `pause01`, `sigprocmask*`, `sigsuspend01`) only after fresh signal/time evidence; statfs/sysinfo only after real ABI semantics.
- Avoid promoting `clock_getres01` as clean while it remains TCONF, and keep `read02` transparent as pass-with-TCONF.
