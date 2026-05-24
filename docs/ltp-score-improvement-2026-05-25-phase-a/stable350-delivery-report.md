# stable350 delivery report

Date: 2026-05-25
Requested target: stable350
Delivered state: **stable300 retained**

## Final status

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: **300 total / 300 unique / 0 duplicates**.
- No duplicate stable entries.
- No new stable cases were added.
- Stable350 final gate was **not run**, because stable315 promotion did not pass.
- Codex goal remains active; final `update_goal({status: "complete"})` was not called.

## What was completed

- Team mode launched and shut down; terminal team state reached `pending=0`, `in_progress=0`, `failed=0` before shutdown.
- Discovery/report lanes produced candidate matrices for permissions/VFS, fd/pipe/iovec, process/wait/signal/rlimit, and mmap/mprotect/munmap.
- Two real implementation changes were integrated by Team checkpoints:
  - LTP runner environment/cwd handling for resource-helper cases (`LTPROOT`, `PATH`, per-case `/tmp/ltp-work/<case>-run`).
  - Linux-visible `prlimit64(current pid)` acceptance plus default-fatal signal delivery/exit handling.
- Validation completed before the follow-up targeted gates:
  - `cargo fmt --all -- --check` passed.
  - `python3 -B scripts/test_ltp_summary.py` passed.
  - `git diff --check` passed.
  - `make A=examples/shell ARCH=riscv64` passed and regenerated remote-submission `kernel-rv`/`kernel-la` outputs through the Makefile path.
- Fresh follow-up targeted evidence was collected:
  - RV `followup-rv-targeted-001`: `PASS LTP CASE 13`, `FAIL LTP CASE 3`; `pipe2_02` TBROK on both libc and `waitpid01` musl TFAIL=40.
  - LA `followup-la-targeted-004`: `PASS LTP CASE 11`, `FAIL LTP CASE 1`; `sched_getscheduler02` LA/musl TFAIL=1.
  - Follow-up marker-prefix scan: `TOTAL markers=28 bad=0`.

## Why stable350 was not delivered

Fresh candidate evidence produced only five four-way clean cases: `prctl05,sethostname01,setrlimit01,signal03,signal04`. This is useful but below stable315's +15 gate, so no stable315/stable330/stable350 aggregate gate was justified.

Current high-value blockers:

- `sched_getscheduler02`: RV clean and LA/glibc clean, but LA/musl has internal `TFAIL=1` (`sched_getscheduler(4194304)` libc variant expected ESRCH).
- `pipe2_02`: fresh RV targeted still TBROK on both libc from helper/resource setup.
- `waitpid01`: fresh RV targeted still musl TFAIL=40 in wait-status/signal semantics.

Post-Team LA attempts `followup-la-targeted-001/002/003` were aborted/untrusted due duplicated starts and are not used as evidence.

## Promotion policy preserved

No timeout, ENOSYS, panic/trap, TFAIL, TBROK, or TCONF was converted to PASS. No case-name hardcoding was introduced. `read02` remains transparently documented as `pass_with_tconf` in the stable300 baseline.

## User-visible / ABI-visible behavior changes integrated this round

- LTP runner behavior: cases with `<case>_*` resource helpers may run from a per-case work directory with `LTPROOT` and adjusted `PATH`; this is harness/environment behavior for executing real LTP binaries, not a PASS shim.
- POSIX/Linux-visible behavior: `prlimit64` now accepts the current process pid as a valid current target; default-fatal self/pending signals are handled more synchronously when unblocked or delivered.
- `LTP_STABLE_CASES` and visible stable score did not change.
