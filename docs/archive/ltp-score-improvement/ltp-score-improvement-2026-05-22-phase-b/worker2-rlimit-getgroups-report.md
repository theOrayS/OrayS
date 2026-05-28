# Worker 2 Task 8: getgroups + rlimit source-fix report

## Scope

Task 8 was limited to real source-level ABI semantics for `getgroups01`, `getrlimit03`, `setrlimit01`, and `setrlimit02` in:

- `examples/shell/src/uspace/credentials.rs`
- `examples/shell/src/uspace/resource_sched.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`
- this report under `docs/ltp-score-improvement-2026-05-22-phase-b/`

No `LTP_STABLE_CASES`, runner PASS/SKIP logic, or `.omx/ultragoal` state was changed.

## Source changes

- `credentials.rs` now uses an explicit `NGROUPS_MAX = 65536`; `getgroups(size > NGROUPS_MAX, ...)` and `setgroups(size > NGROUPS_MAX, ...)` return `EINVAL`. This matches the `getgroups01` oversized-size expectation without hardcoding an LTP case name.
- `resource_sched.rs` now rejects unprivileged hard-limit raises in `prlimit64`/`setrlimit` with `EPERM` after validating `rlim_cur <= rlim_max`. This addresses `setrlimit02`'s expected `EPERM` path while preserving the existing `EINVAL` path for invalid soft/hard ordering.
- `resource_sched.rs` adds RV-only `sys_getrlimit`/`sys_setrlimit` wrappers that reuse the `prlimit64` implementation for the current process.
- `syscall_dispatch.rs` wires RV `__NR_getrlimit`/`__NR_setrlimit` to those wrappers. The dispatch is `#[cfg(target_arch = "riscv64")]` because the checked `linux-raw-sys` loongarch64 table does not expose old `getrlimit`/`setrlimit` syscall constants.

## Verification evidence

### Formatting / build

| Command | Exit | Evidence |
| --- | ---: | --- |
| `rustfmt --check examples/shell/src/uspace/credentials.rs examples/shell/src/uspace/resource_sched.rs examples/shell/src/uspace/syscall_dispatch.rs` | 0 | `worker2-rustfmt-check.txt` |
| `cargo fmt --all -- --check` | 1 | `worker2-cargo-fmt-all-check.txt`; fails before formatting because vendored `rust-fatfs` still points at `/root/oskernel2026-orays/Cargo.toml` workspace from inside this worker worktree. |
| `make A=examples/shell ARCH=riscv64` | 0 | RV and LA release kernels rebuilt successfully in this worktree; only pre-existing vendored `smoltcp`/`axnet` warnings remained. |

### Targeted RV LTP before RV `getrlimit` dispatch

Command:

```bash
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=getgroups01,getrlimit03,setrlimit01,setrlimit02 \
LTP_CASE_TIMEOUT_SECS=20 \
./run-eval.sh
```

Evidence:

- Status: `worker2-rlimit-getgroups-rv.status` (`run_eval_exit: 0`).
- Parser output: `worker2-rlimit-getgroups-rv.summary.txt`.
- Matrix: 4 PASS / 4 FAIL.
  - PASS: `getgroups01` RV musl+glibc, `setrlimit02` RV musl+glibc.
  - FAIL: `getrlimit03` RV musl+glibc had 16 `ENOSYS` hits per libc because old `__NR_getrlimit` was missing; `setrlimit01` RV musl+glibc still failed.

### Targeted RV LTP after RV `getrlimit` dispatch

Same command as above, saved to:

- Raw log: `worker2-rlimit-getgroups-rv-after-getrlimit.log`
- Status: `worker2-rlimit-getgroups-rv-after-getrlimit.status` (`run_eval_exit: 0`)
- Parsed summary: `worker2-rlimit-getgroups-rv-after-getrlimit.summary.txt`

`ltp_summary.py` result:

| Case | RV musl | RV glibc | Notes |
| --- | --- | --- | --- |
| `getgroups01` | PASS | PASS | oversized sizes now return `EINVAL`; gidset preservation and returned gid checks pass. |
| `getrlimit03` | PASS | PASS | `__NR_prlimit64` and `__NR_getrlimit` agree for resources 0..15; ENOSYS count is now 0. |
| `setrlimit02` | PASS | PASS | invalid ordering returns `EINVAL`; unprivileged hard-limit raise returns `EPERM`. |
| `setrlimit01` | FAIL | FAIL | still real failure: `setrlimit failed, expected 10 got 26`; RV musl also hit the 20s case timeout. |

Parser totals after fix:

- PASS LTP CASE: 6
- FAIL LTP CASE: 2
- Internal TFAIL/TBROK/TCONF: 2 (`TFAIL`: 2)
- timeout matches: 1 (`rv:musl:setrlimit01`)
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0

## Remaining blocker

`setrlimit01` still fails in both RV libc variants. The failing subtest is the `RLIMIT_FSIZE` behavior: LTP expects the file-size-limit path to report code 10, but the current kernel reports code 26 (`setrlimit failed, expected 10 got 26`). The relevant filesystem/write enforcement path is outside Task 8's allowed write scope, so I left it visible as a real blocker instead of converting it to PASS/SKIP or editing unrelated files.

