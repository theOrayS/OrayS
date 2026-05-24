# Final gate code-review report

Recommendation: **BLOCK stable350 / PASS narrow waitpid01 and /bin/sh exec compatibility fixes**
Architect status: **BLOCKED for stable350**

## Reason

Final stable350 acceptance is still blocked by missing clean promotion evidence: only 8 RV+LA × musl+glibc clean seed cases are available, below the +15 stable315 tranche gate, and no stable aggregate gate was run.

## Narrow fix review

Reviewed follow-up changes in:

- `examples/shell/src/uspace/task_context.rs`
- `examples/shell/src/uspace/signal_abi.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`

The change records the previous signal mask when libc transiently sets the all-application-signal mask, then uses that recorded mask for fork-like process children only. Thread clone inheritance still uses the live signal mask. This addresses `waitpid01` musl children inheriting libc's temporary all-mask fork critical section and exiting normally instead of being default-terminated by `raise()`/`kill(getpid())`.

Review findings:

- No LTP case-name hardcoding, fake PASS, timeout laundering, or marker-output change was introduced.
- The behavior is scoped to process clone/fork inheritance and signal-mask state; it does not change the stable case list.
- The code preserves signal mask behavior for non-all-mask `rt_sigprocmask()` calls by clearing the restore sentinel.
- Risk remains moderate because this is Linux/POSIX-visible signal inheritance behavior and uses a libc-pattern heuristic; therefore it is not enough for broad promotion without targeted regression guards.

## Additional /bin/sh compatibility fix review

Reviewed follow-up changes in `examples/shell/src/uspace/process_lifecycle.rs` for `execve("/bin/sh", ...)` compatibility. LTP resource helpers call libc `system()`, which requires a shell at `/bin/sh`; this tree provides suite-local busybox binaries instead of a root-level `/bin/sh`. The fix falls back to the current process exec root's busybox, preserving argv shell dispatch, with musl/glibc fallback ordering.

Review findings:

- This is not case-name hardcoding; it is a general `/bin/sh`/busybox exec compatibility path.
- It enables real `system("cp ...")` execution for LTP resource copying instead of bypassing test behavior.
- It does not alter marker output or stable case membership.

Evidence reviewed:

- RV `pipe2_02`: `raw/followup-rv-pipe2_02-binsh-001-summary.txt` PASS 2 / FAIL 0, internal TFAIL/TBROK/TCONF=0.
- LA `pipe2_02`: `raw/followup-la-pipe2_02-binsh-001-summary.txt` PASS 2 / FAIL 0, internal TFAIL/TBROK/TCONF=0.
- Marker prefix: `raw/followup-pipe2-binsh-marker-prefix-check.txt` reports `TOTAL markers=4 bad=0`.

## Evidence reviewed

- `cargo fmt --all -- --check` passed.
- `git diff --check` passed.
- `make A=examples/shell ARCH=riscv64` passed.
- RV waitpid targeted: `raw/followup-rv-waitpid01-maskrestore-001-summary.txt` PASS 2 / FAIL 0, internal TFAIL/TBROK/TCONF=0.
- LA waitpid targeted: `raw/followup-la-waitpid01-maskrestore-001-summary.txt` PASS 2 / FAIL 0, internal TFAIL/TBROK/TCONF=0.
- RV signal/wait guard: `raw/followup-rv-waitpid-signal-guard-001-summary.txt` PASS 16 / FAIL 0, internal TFAIL/TBROK/TCONF=0.
- LA signal/wait guard: `raw/followup-la-waitpid-signal-guard-001-summary.txt` PASS 16 / FAIL 0, internal TFAIL/TBROK/TCONF=0.
- Marker prefix: `raw/followup-waitpid-marker-prefix-check.txt` reports `TOTAL markers=42 bad=0`.

## Required before final APPROVE

1. Clean serialized RV targeted gate for a >=15-case candidate tranche.
2. Clean serialized LA targeted gate for exactly the same subset.
3. Stable aggregate gate after editing `LTP_STABLE_CASES`.
4. Marker prefix check on final RV/LA logs.
