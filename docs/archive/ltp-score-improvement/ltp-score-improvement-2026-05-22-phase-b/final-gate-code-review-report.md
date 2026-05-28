# Final gate code review report — phase-b

## Reviewed files

- `examples/shell/src/cmd.rs`
- `examples/shell/src/uspace/credentials.rs`
- `examples/shell/src/uspace/resource_sched.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`

## Findings

No blocking findings.

### Notes

- `cmd.rs`: adds 14 cases to `LTP_STABLE_CASES` after LA/RV x musl/glibc evidence and stable115 targeted gates.
- `resource_sched.rs`: implements real scheduler priority bounds and `sched_rr_get_interval`; invalid policies return `EINVAL`, RR/FIFO bounds are 1..99, normal/batch/idle/deadline bounds are 0..0.
- `syscall_dispatch.rs`: dispatches `faccessat` with four-argument legacy semantics (`flags=0`) and `faccessat2` with explicit flags, preventing garbage arg3 from affecting libc `access()`/`faccessat()` wrappers; dispatches scheduler priority/interval syscalls.
- `credentials.rs`: formatting/import ordering only in the final diff.

## ABI / errno / syscall-visible changes

- `sched_get_priority_max(2)` and `sched_get_priority_min(2)` now return Linux-compatible bounds or `EINVAL` for invalid policies.
- `sched_rr_get_interval(2)` now validates target pid and copy-out pointer, then returns a finite 10ms RR interval.
- `faccessat(2)` legacy syscall now ignores a nonexistent flags argument; `faccessat2(2)` carries flags explicitly. This fixes observable access/faccessat behavior without masking failures.
- Stable runner visible behavior changes: `LTP_STABLE_CASES` grows from 101 to 115.

## Verification evidence

- `cargo fmt --all -- --check`: first run exit 1 due import ordering; rerun after cargo fmt exit 0 (final-cargo-fmt-check.status, final-cargo-fmt-check-rerun.status)
- `make A=examples/shell ARCH=riscv64`: exit 0 (run before and after formatting) (final-shell-riscv64-build.status, final-shell-riscv64-build-rerun.status)
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh la`: exit 0, stable115 targeted gate (stable115-targeted-la.status / summary)
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh`: exit 0, stable115 targeted gate (stable115-targeted-rv.status / summary)
- `./run-eval.sh la 2>&1 | tee output_la.md`: exit 0, final full LA gate (final-full-la.status, final-gate-output-la-summary.txt)
- `./run-eval.sh 2>&1 | tee output_rv.md`: exit 0, final full RV gate (final-full-rv.status, final-gate-output-rv-summary.txt)
- `python3 -B scripts/ltp_summary.py output_la.md / output_rv.md`: exit 0 via pipeline shell status; summaries generated (final-gate-output-*-summary.txt)
- `cargo fmt --all -- --check in /root/oskernel2026-orays-remote`: exit 0 after source sync (remote fmt check shell output)

## Verdict

APPROVE / CLEAR. Stable120 remains blocked by real candidate failures, not by review concerns.
