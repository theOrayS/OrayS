# Worker 4 process/wait/sched/rlimit/proc lane report

Date: 2026-05-25
Worker: `worker-4`
Task: `process/wait/sched/rlimit/proc lane`

## Scope and guardrails

- Lane scope: `waitpid01`, scheduler negative-pid cases, rlimit/RLIMIT_FSIZE, `prctl`, `sethostname`, and procfs-adjacent candidates.
- Did not edit `.omx/ultragoal`.
- Did not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; live list remains 300 cases.
- Did not run or claim a promotion gate. Worker QEMU evidence remains discovery-only unless the leader serializes promotion because shared `/tmp/arceos-sdcard-*.run.qcow2` names can invalidate concurrent runs.
- No source change was made: the current evidence points to real remaining blockers or already-clean adjacent cases, not to a safe narrow one-line fix.

## Evidence summary

| Area | Current evidence | Decision |
| --- | --- | --- |
| `waitpid01` | Fresh parser evidence from `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-24-phase-a/raw/fix-waitpid01-rv-20260524T102451+0800.log`: RV glibc PASS, RV musl FAIL with 40 `TFAIL`, no timeout/ENOSYS/panic. The later signal-focused rerun `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-24-phase-a/raw/fix-waitpid01-signal-rv-20260524T142856+0800.log` has the same RV musl 40-`TFAIL` shape. | Blocked for promotion. Existing `process_lifecycle.rs` has pid matching and signal wait status plumbing, but musl still reports signal children as normal exit for most signal cases. A source edit without an isolated musl raise/kill reproducer would risk papering over real wait-status semantics. |
| Scheduler negative-pid (`sched_getparam03`, `sched_getscheduler02`, `sched_setparam04`) | RV matrix `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-24-phase-a/raw/fix-sched-negative-pid-rv.log` is clean: 6 PASS, 0 FAIL. LA matrix `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-24-phase-a/raw/fix-sched-negative-pid-la.log` has glibc PASS but musl FAIL for all three cases, 8 total `TFAIL`, no timeout/ENOSYS/panic. Raw failure shape shows musl libc wrappers seeing raw negative return values while syscall variants/glibc pass. | Blocked for promotion on LA musl. Current `resource_sched.rs` validates negative pid with `EINVAL` and non-existing pid with `ESRCH`; the syscall path already behaves correctly. Do not change scheduler semantics until the LA musl libc-wrapper ABI/errno path is isolated. |
| RLIMIT_FSIZE / `setrlimit01` | Prior phase reports keep `setrlimit01` as a real blocker; current source enforces `RLIMIT_FSIZE` in `sys_ftruncate`, `sys_truncate`, `sys_write`, and `sys_writev`. Historical evidence also shows stable `getrlimit*` coverage but `setrlimit01` should not be inferred clean from getter success. | Keep as a targeted follow-up. Likely next investigation should focus on exact `setrlimit01` subtest expectations around file growth, `EFBIG`, and `SIGXFSZ`, not broad rlimit rewrites. |
| `prctl`/proc comm | `fix-prctl05-comm-rv.log` and `target-stable300-candidates-rv.log` show old RV `prctl05` `TBROK`, while `fix-prctl05-comm-rv2.log` later shows 2 PASS/0 FAIL and LA target candidates show `prctl05` clean. | Treat as previously fixed but require leader-owned final stable evidence. Do not touch source here. |
| `sethostname`/nice adjacent | `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-24-phase-a/raw/fix-prctl-sethostname-nice-la-20260524T165711+0800.log` is clean: 12 PASS/0 FAIL over LA musl+glibc for `nice01`, `nice02`, `prctl01`, `sethostname01`, `sethostname02`, `sethostname03`. | No source change needed. |
| procfs synthetic visibility | Current source exposes `/proc/self/maps`, `/proc/<pid>/stat`, `/proc/<pid>/status`, and proc comm paths through `synthetic_fs.rs`/`fd_table.rs`. No new failing procfs evidence was found in this lane beyond old `prctl05` proc-comm failures that have a later clean rerun. | No source change needed. |

## Source review anchors

- `examples/shell/src/uspace/process_lifecycle.rs`: `wait_child`, `wait_status`, `request_signal_exit_group`, `signal_wait_status` already carry process selection and signal status state.
- `examples/shell/src/uspace/signal_abi.rs`: default signal delivery calls `request_signal_exit_group`; musl `waitpid01` failure likely needs an isolated signal-delivery/re-entry reproducer before editing this path again.
- `examples/shell/src/uspace/resource_sched.rs`: scheduler target selection rejects negative pids and non-existing pids separately; syscall evidence is already clean on RV and glibc LA.
- `examples/shell/src/uspace/fd_table.rs`: `RLIMIT_FSIZE` checks are present in `ftruncate`, `truncate`, `write`, and `writev`; next work should validate exact Linux signal/error ordering.
- `examples/shell/src/uspace/synthetic_fs.rs` and `examples/shell/src/uspace/system_info.rs`: proc/prctl/sethostname adjacent code is already covered by later clean targeted evidence.

## Subagent integration

Subagent spawn evidence: 2, Hegel/019e5ac6-501b-7743-85c9-e21b992566c0, Aristotle/019e5ac6-6dce-74a1-9548-b8b59aba3d7b, findings integrated: Hegel confirmed `waitpid01`, LA musl scheduler negative-pid, and `setrlimit01` remain the risky blockers while prctl/sethostname/procfs should not be broadened; Aristotle supplied verification guidance and warned that worker QEMU runs are discovery-only unless serialized by the leader.

## Recommended next feasible task

1. Isolate LA musl scheduler libc wrapper behavior with a minimal `sched_getparam/getscheduler/setparam` negative-pid harness that logs raw return and `errno` before any `resource_sched.rs` edit.
2. Isolate RV musl `waitpid01` by comparing `raise(sig)`, `kill(getpid(), sig)`, and direct syscall signal delivery, then inspect whether musl exits through a path that bypasses pending-signal termination before `wait_status` is observed.
3. For `setrlimit01`, run a single-case RV targeted log and inspect the exact failing subtest before changing `RLIMIT_FSIZE` handling. Candidate narrow fix, if confirmed, should be limited to `EFBIG`/`SIGXFSZ` ordering in write/truncate paths.
4. Leave `prctl05`, `sethostname01-03`, and procfs synthetic paths untouched unless a new leader-serialized matrix regresses them.

## Verification performed

- Parsed `fix-waitpid01-rv-20260524T102451+0800.log`: 1 PASS, 1 FAIL; RV musl `waitpid01` has 40 `TFAIL`.
- Parsed `fix-waitpid01-signal-rv-20260524T142856+0800.log`: same 1 PASS, 1 FAIL; RV musl `waitpid01` has 40 `TFAIL`.
- Parsed `fix-sched-negative-pid-rv.log`: 6 PASS, 0 FAIL.
- Parsed `fix-sched-negative-pid-la.log`: 3 PASS, 3 FAIL; LA musl scheduler negative-pid cases have 8 total `TFAIL`.
- Parsed `fix-prctl-sethostname-nice-la-20260524T165711+0800.log`: 12 PASS, 0 FAIL.
- Parsed `fix-prctl05-comm-rv2.log`: 2 PASS, 0 FAIL.
- Live stable list count checked from `examples/shell/src/cmd.rs`: 300.
