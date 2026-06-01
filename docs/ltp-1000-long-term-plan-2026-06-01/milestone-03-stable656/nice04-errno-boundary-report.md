# Milestone 03 `nice04` errno-boundary report

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Live stable baseline: `606 total / 606 unique / 0 duplicate`

## Question

Can `nice04` be converted into a safe post-stable606 candidate by changing the priority errno path?

## Current evidence

Current RV scout artifact:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.summary.txt`

Relevant raw line:

```text
nice04.c:32: TFAIL: nice(-10) should fail with EPERM: EACCES (13)
```

The failure is RV musl only in this shard; RV glibc was clean. Closed arch-sweep matrices show LA musl and LA glibc were clean for the same case, so the blocker is an arch/libc boundary rather than a simple all-architecture priority syscall failure.

## LTP source requirement

Source inspected:

- `/root/oskernel2026-orays-clean-stable520/docs/ltp-score-improvement-2026-05-28-phase-b/raw/ltp-source/nice_nice04.c`

The test switches to `nobody`, calls `nice(-10)`, and requires `TST_ERR == EPERM`.

## Kernel boundary

Relevant implementation:

- `examples/shell/src/uspace/resource_sched.rs::sys_setpriority`
- `examples/shell/src/uspace/syscall_dispatch.rs` dispatches `general::__NR_setpriority` and `general::__NR_getpriority`; neither current RV nor LA syscall constant set exposes a separate `__NR_nice` entry.

Current `sys_setpriority` deliberately returns:

- `EPERM` when an unprivileged caller targets a process with a different uid;
- `EACCES` when an unprivileged caller tries to lower its own/process-group priority.

This matches the stable `setpriority02` LTP source inspected at:

- `/root/oskernel2026-orays-clean-stable520/docs/ltp-score-improvement-2026-05-28-phase-b/raw/ltp-source/setpriority_setpriority02.c`

That source explicitly expects `setpriority(PRIO_PROCESS, 0, -2)` and `setpriority(PRIO_PGRP, 0, -2)` as an unprivileged user to fail with `EACCES`, while `setpriority(PRIO_PROCESS, 1, -2)` fails with `EPERM`.

## Decision

Do **not** change `sys_setpriority` from `EACCES` to `EPERM` just to satisfy RV musl `nice04`. That would risk regressing the stable `setpriority02` contract and would be a libc/kernel wrapper-boundary workaround, not a generic POSIX/Linux semantic fix.

`nice04` remains outside the stable656 candidate pool until one of these is true:

1. a legitimate `nice()`-specific wrapper/syscall boundary is identified that does not affect direct `setpriority(2)` errno behavior; or
2. an upstream-compatible libc/wrapper explanation justifies the arch/libc split without changing direct `setpriority` semantics.

## Regression requirements if revisited

Any future fix must run at minimum:

- targeted RV/LA `nice04`;
- stable `setpriority02` on RV/LA;
- adjacent `setpriority01`, `sched_setaffinity01`, and scheduler permission subset if priority code changes;
- parser check for zero new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`.
