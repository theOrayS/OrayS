# stable350 delivery report

Date: 2026-05-25
Requested target: stable350
Delivered state: **stable300 retained**

## Final status

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: **300 total / 300 unique / 0 duplicates**.
- No duplicate stable entries.
- No new stable cases were added.
- Stable350 final gate was **not run**, because no stable315/stable330 promotion gate passed.
- Codex goal remains active; final `update_goal({status: "complete"})` was not called.

## What was completed

- Team mode launched and shut down; terminal team state reached `pending=0`, `in_progress=0`, `failed=0` before shutdown.
- Discovery/report lanes produced candidate matrices for permissions/VFS, fd/pipe/iovec, process/wait/signal/rlimit, and mmap/mprotect/munmap.
- Two real implementation changes were integrated by Team checkpoints:
  - LTP runner environment/cwd handling for resource-helper cases (`LTPROOT`, `PATH`, per-case `/tmp/ltp-work/<case>-run`).
  - Linux-visible `prlimit64(current pid)` acceptance plus default-fatal signal delivery/exit handling.
- Validation completed before the aborted targeted rerun attempts:
  - `cargo fmt --all -- --check` passed.
  - `python3 -B scripts/test_ltp_summary.py` passed.
  - `git diff --check` passed.
  - `make A=examples/shell ARCH=riscv64` passed and regenerated remote-submission `kernel-rv`/`kernel-la` outputs through the Makefile path.

## Why stable350 was not delivered

Fresh candidate evidence did not produce a clean tranche. The strongest blocker evidence remains:

- `raw/batch-a-rv-summary.txt`: one-side libc failures across most near-clean candidates.
- `raw/blocker-batch-rv-summary.txt`: user-priority cases still show real TFAIL/TBROK/TCONF/ENOSYS or crash-style wrapper codes.
- Post-Team targeted reruns were aborted/untrusted due duplicate/aborted evaluator launches and are not used as evidence.

## Promotion policy preserved

No timeout, ENOSYS, panic/trap, TFAIL, TBROK, or TCONF was converted to PASS. No case-name hardcoding was introduced. `read02` remains transparently documented as `pass_with_tconf` in the stable300 baseline.

## User-visible / ABI-visible behavior changes integrated this round

- LTP runner behavior: cases with `<case>_*` resource helpers may run from a per-case work directory with `LTPROOT` and adjusted `PATH`; this is harness/environment behavior for executing real LTP binaries, not a PASS shim.
- POSIX/Linux-visible behavior: `prlimit64` now accepts the current process pid as a valid current target; default-fatal self/pending signals are handled more synchronously when unblocked or delivered.
- `LTP_STABLE_CASES` and visible stable score did not change.
