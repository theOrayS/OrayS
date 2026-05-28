# Final gate report — LTP stable101 to stable115 phase-b

Date: 2026-05-22

## Outcome

- Final promoted stable score: **115 cases / libc / arch**.
- Delta from phase-a baseline stable101: **+14**.
- Minimum success line stable115: **met**.
- Main target stable120 and stretch stable125+: **blocked by real evidence** after aggressive Wave A/A2/B/C targeted runs.

## Files changed

- `examples/shell/src/cmd.rs`
  - `LTP_STABLE_CASES`: added 14 verified stable cases.
- `examples/shell/src/uspace/resource_sched.rs`
  - Added `sched_priority_bounds`.
  - Added `sys_sched_get_priority_max`.
  - Added `sys_sched_get_priority_min`.
  - Added `sys_sched_rr_get_interval`.
- `examples/shell/src/uspace/syscall_dispatch.rs`
  - Added dispatch for `sched_get_priority_max`, `sched_get_priority_min`, `sched_rr_get_interval`.
  - Split legacy `faccessat` (`flags=0`) from `faccessat2` (explicit flags).
- `examples/shell/src/uspace/credentials.rs`
  - Formatting/import order only in final diff.

## New stable cases

- `dup202`
- `mkdirat01`
- `openat01`
- `pipe04`
- `pipe05`
- `pread01`
- `pwrite01`
- `sysinfo01`
- `faccessat01`
- `getgroups01`
- `setrlimit02`
- `sched_get_priority_max01`
- `sched_get_priority_min01`
- `sched_rr_get_interval01`

## Final LA/RV LTP summary

| Gate | LA | RV |
| --- | --- | --- |
| PASS LTP CASE | 230 | 230 |
| FAIL LTP CASE | 0 | 0 |
| ltp-musl | 115/0 | 115/0 |
| ltp-glibc | 115/0 | 115/0 |
| internal TFAIL/TBROK/TCONF | TFAIL=0, TBROK=0, TCONF=4 | TFAIL=0, TBROK=0, TCONF=4 |
| timeout | 0 | 0 |
| ENOSYS/not implemented | 0 | 0 |
| panic/trap | 0 | 0 |


`read02` remains pass_with_tconf, yielding total `TCONF=4`; this is transparent and not claimed as a clean pass.

## Validation commands and exit codes

- `cargo fmt --all -- --check`: first run exit 1 due import ordering; rerun after cargo fmt exit 0; evidence `final-cargo-fmt-check.status, final-cargo-fmt-check-rerun.status`.
- `make A=examples/shell ARCH=riscv64`: exit 0 (run before and after formatting); evidence `final-shell-riscv64-build.status, final-shell-riscv64-build-rerun.status`.
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh la`: exit 0, stable115 targeted gate; evidence `stable115-targeted-la.status / summary`.
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh`: exit 0, stable115 targeted gate; evidence `stable115-targeted-rv.status / summary`.
- `./run-eval.sh la 2>&1 | tee output_la.md`: exit 0, final full LA gate; evidence `final-full-la.status, final-gate-output-la-summary.txt`.
- `./run-eval.sh 2>&1 | tee output_rv.md`: exit 0, final full RV gate; evidence `final-full-rv.status, final-gate-output-rv-summary.txt`.
- `python3 -B scripts/ltp_summary.py output_la.md / output_rv.md`: exit 0 via pipeline shell status; summaries generated; evidence `final-gate-output-*-summary.txt`.
- `cargo fmt --all -- --check in /root/oskernel2026-orays-remote`: exit 0 after source sync; evidence `remote fmt check shell output`.

## Non-LTP full-gate markers

These remain present in final full logs and are outside stable LTP promotion criteria:

- LA and RV: busybox `which ls fail` appears twice per full log.
- LA: libcbench-glibc futex unexpected error code appears 4 times; RV appears 5 times.
- LA and RV: iperf-glibc `end: fail` appears for BASIC_UDP/BASIC_TCP/PARALLEL_UDP/PARALLEL_TCP/REVERSE_UDP/REVERSE_TCP.

## Blocked cases / why stable120 was not promoted

- `getrlimit03`: RV clean but LA musl/glibc ENOSYS/missing legacy getrlimit syscall wrapper; not promoted.
- `unlinkat01`: RV clean but LA glibc wrapper/TBROK/order pollution; not promoted.
- `sched_getscheduler02`: RV clean but prior/current LA evidence not clean enough; historical LA musl TFAIL.
- `access02`: TFAIL on execute-file setup/ENOENT semantics.
- `access04`: TBROK, tmpfs mount returns EINVAL in harness environment.
- `dup03`: TFAIL, dup unexpectedly succeeded in negative case.
- `pipe02`: TFAIL, child signal/pipe kill semantics not correct.
- `lseek02`: mkfifo/fixture path hits ENOSYS.
- `readlinkat01/readlinkat02`: invalid input / ENOTDIR semantics still failing.
- `setrlimit01`: RV TFAIL/timeout under 20s targeted gate; not clean.
- `waitpid04/kill02/kill05/pause01/sigpending02/sigwait01/sigtimedwait01/setitimer01/getitimer01/nanosleep02`: Wave C had real TFAIL/TBROK/timeout/ENOSYS; not promoted.

The gate was intentionally stopped at stable115 because adding five more cases would require accepting real TFAIL/TBROK/timeout/ENOSYS evidence, which violates the promotion rules.

## Remote sync

Synced source files to `/root/oskernel2026-orays-remote`:

- `examples/shell/src/cmd.rs`
- `examples/shell/src/uspace/credentials.rs`
- `examples/shell/src/uspace/resource_sched.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`

Preserved remote-only address-mapping differences by not touching architecture/platform mapping files. Remote had pre-existing unrelated dirty files (`LoongArch输出.txt`, `Riscv输出.txt`, `docs/merge-moss-kernel-like-remote-2026-05-22/`, `docs/remote-eval-scores.md`). Remote `cargo fmt --all -- --check` exited 0 after sync.

## Syscall / errno / ABI-visible behavior changes

- `sched_get_priority_max(2)` / `sched_get_priority_min(2)` now return Linux-compatible scheduling priority bounds, including `EINVAL` for invalid policies.
- `sched_rr_get_interval(2)` now validates pid/copy-out and reports a finite 10ms interval.
- Legacy `faccessat(2)` now uses `flags=0`; `faccessat2(2)` handles explicit flags. This is ABI-visible through access/faccessat wrappers and fixes prior false TFAILs.
- Stable runner output now includes 115 cases per libc/arch instead of 101.

## Quality gate

- ai-slop-cleaner: APPROVE / CLEAR (`final-gate-ai-slop-cleaner-report.md`).
- code-review: APPROVE / CLEAR (`final-gate-code-review-report.md`).
- machine-readable quality gate: `final-gate-quality-gate.json`.
