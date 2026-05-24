# Final gate code-review report

Recommendation: **BLOCK stable350 / PASS narrow sched_getscheduler02 fix**
Architect status: **BLOCKED for stable350**

## Reason

Final stable350 acceptance is still blocked by missing clean promotion evidence: only 6 RV+LA × musl+glibc clean seed cases are available, below the +15 stable315 tranche gate, and no stable aggregate gate was run.

## Narrow fix review

Reviewed `examples/shell/src/uspace/program_loader.rs` after the follow-up fix:

- The LoongArch musl scheduler patch remains architecture- and interpreter-scoped (`/musl` + `ld-musl`).
- It patches exported scheduler libc wrappers by issuing the real syscall and tail-branching to the wrapper's existing `__syscall_ret` target, preserving libc `errno`/`-1` semantics.
- Raw syscall tests still bypass the libc symbol and keep raw `-errno` behavior through `syscall_dispatch`.
- No LTP case name hardcoding, fake PASS, timeout laundering, or marker-output change was introduced.

## Evidence reviewed

- `cargo fmt --all -- --check` passed.
- `git diff --check` passed.
- `python3 -B scripts/test_ltp_summary.py` passed.
- `make A=examples/shell ARCH=loongarch64 build` passed.
- `OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_getscheduler02 LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la` passed and was parsed by `scripts/ltp_summary.py` as `PASS LTP CASE 2`, `FAIL 0`, with internal TFAIL/TBROK/TCONF=0 and timeout/ENOSYS/panic/trap=0.

## Required before final APPROVE

1. Clean serialized RV targeted gate for a >=15-case candidate tranche.
2. Clean serialized LA targeted gate for exactly the same subset.
3. Stable aggregate gate after editing `LTP_STABLE_CASES`.
4. Marker prefix check on final RV/LA logs.
