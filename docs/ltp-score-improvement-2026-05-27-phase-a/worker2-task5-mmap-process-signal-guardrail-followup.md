# Worker 2 follow-up for task 5 mmap/process/signal guardrails

Date: 2026-05-27
Team: `ltp-stable413-to-stab-d9f99e59`
Task: 5 — `Worker 5 mmap/process/signal + guardrails lane`, reassigned/continued by `worker-2`
Status: completed as a follow-up guardrail/test lane. No `.omx/ultragoal` edit and no final `examples/shell/src/cmd.rs::LTP_STABLE_CASES` edit were made.

## Report paths

- Prior lane report reused as the main candidate table: `docs/ltp-score-improvement-2026-05-27-phase-a/worker5-mmap-process-signal-guardrail-report.md`
- Prior raw evidence reused: `docs/ltp-score-improvement-2026-05-27-phase-a/raw/worker5-mmap-process-signal-evidence.txt`
- This continuation report: `docs/ltp-score-improvement-2026-05-27-phase-a/worker2-task5-mmap-process-signal-guardrail-followup.md`
- Parser guardrail tests updated: `scripts/test_ltp_summary.py`

## Candidate status table

| Case/group | Worker-2 follow-up status | Evidence / next action |
| --- | --- | --- |
| `mmap06`, `mmap10` | Already stable413; not new stable460 candidates. | Live stable scan: `total=413 unique=413 duplicates=0`; both cases are already in `LTP_STABLE_CASES`. |
| `mmap04`, `mmap05` | Blocked. | Historical targeted rows show RV glibc+musl `FAIL code=2`, `TBROK=1`; no direct fresh row in current smoke logs. Repair `/proc/self/maps` / VMA visibility or VM fault semantics before LA spend. |
| `munmap01` | Blocked. | Historical targeted rows show RV glibc+musl `FAIL code=139`; treat as post-unmap fault/signal/boundary repair, not a promotion-list edit. |
| `mprotect01`, `mprotect02` | Blocked. | Historical targeted rows show `mprotect01` `TFAIL=2` and `mprotect02` `TBROK=2`; needs semantic repair and regression. |
| `mmap10_1` | Inventory/staging blocker. | Historical fallback row has wrapper `FAIL code=-1`; refresh sdcard/runtest inventory before runtime debugging. |
| `vma01`, `vma02` | Blocked. | Historical fallback rows show `vma01` `TBROK=4` and `vma02` `TCONF=2`; no promotion without real clean PASS rows. |
| `waitid07`, `waitid08` | Blocked. | Existing evidence ties failures to missing stopped/continued event accounting. Needs real child state model (`WSTOPPED`/`WCONTINUED`/`WNOWAIT`), not a guardrail/parser change. |
| `waitid10` | Blocked/shared-scope. | Existing evidence points first at synthetic proc/VFS setup such as `/proc/sys/kernel/core_pattern`; shared with metadata/VFS surfaces. |
| `kill02` | High-risk aggregate blocker. | Targeted clean rows exist, but aggregate LA history has `TBROK=4`; do not promote from targeted-only rows. |
| `poll02`, `gethostid01`, `getcpu01`, `gethostname02` | Blocker/diagnosis only. | Timer/libc/setup/ENOSYS/TCONF issues remain; not a four-way clean promotion subset. |

## Guardrail change made

Added two regression tests to `scripts/test_ltp_summary.py` so future stable460 promotion tooling cannot silently accept misleading clean-looking rows:

1. `test_promotion_candidate_blocks_timeout_even_after_wrapper_pass` — proves `TIMEOUT LTP CASE` after a wrapper pass is still blocked with `timeout=1` and `status=TIMEOUT`.
2. `test_promotion_candidate_blocks_enosys_and_panic_trap_markers` — proves `ENOSYS/not implemented` and panic/trap markers block promotion even when wrapper status is `PASS : 0`.

This extends the existing guardrails for numeric wrapper status, internal `TCONF`, and prior fail-event masking without changing `scripts/ltp_summary.py` behavior.

## Delegation evidence

Subagents spawned: 2, both with model `gpt-5.4-mini`.

- `019e68c9-667a-7743-a3bf-e01394fa007d` (`James`) — mmap/VMA scout. Integrated findings: current `output_rv.md`/`output_la.md` only contain `mmap01` in the smoke logs, not the target mmap/VMA cases; `mmap06`/`mmap10` are already stable; `mmap04/05`, `munmap01`, `mprotect01/02`, `mmap10_1`, and `vma01/02` remain blocked or inventory-gapped.
- `019e68c9-932e-7ec3-a41f-038787a991a3` (`Socrates`) — process/signal/guardrail scout. Integrated findings: `read02` remains the transparent `pass_with_tconf` baseline exception; `waitid07/08/10` and `kill02` remain blocker/high-risk items; parser summary code already separates wrapper status, timeout, `TCONF`, `ENOSYS`, and panic/trap markers.

Serial repo-search/read commands before spawn: 0 after taking task 5; subagents were spawned before additional task-5 serial exploration.

## Verification

| Check | Command | Result |
| --- | --- | --- |
| Stable-list invariant | Python scan of `examples/shell/src/cmd.rs::LTP_STABLE_CASES` | PASS: `total=413 unique=413 duplicates=0`; `mmap06`/`mmap10` stable, target blocked cases not stable. |
| Parser regression tests | `python3 -B scripts/test_ltp_summary.py` | PASS: `Ran 12 tests in 0.501s` / `OK`. |
| Script test discovery | `python3 -B -m unittest discover -s scripts -p 'test_*.py'` | PASS: `Ran 12 tests in 0.489s` / `OK`. |
| Python syntax/type-smoke | `python3 -m py_compile scripts/ltp_summary.py scripts/test_ltp_summary.py` | PASS. |
| One-log summary sanity | `python3 -B scripts/ltp_summary.py output_rv.md`; `python3 -B scripts/ltp_summary.py output_la.md` | PASS: each log reports `PASS LTP CASE: 126`, `FAIL LTP CASE: 0`, internal `TCONF=4`, timeout string matches `10`, ENOSYS `0`, panic/trap `0`. These are smoke/stable logs, not non-stable promotion evidence. |
| Promotion smoke | `python3 -B scripts/ltp_summary.py --promotion-candidates --json output_rv.md output_la.md` | PASS: `candidate_count=62`, `blocked_count=1`, first blocked `read02`, first candidates `access01,access03,alarm02,alarm03,brk01`. |
| Guardrail grep | Literal Python grep over phase docs, outputs, `cmd.rs`, `process_lifecycle.rs`, `signal_abi.rs` | PASS: found expected `waitid*`, `kill02`, `read02`, `TCONF`, timeout, marker-prefix guardrail references without shell interpolation. |
| Linter/static whitespace | `git diff --check` | PASS. |
| QEMU/evaluator policy | `ps -ef | grep -E 'qemu-system|run-eval\.sh'` | PASS for worker-2 policy: no intentional QEMU/evaluator scout was started by this follow-up. Existing leader-owned `run-eval.sh rv` / `qemu-system-riscv64` was observed, so no serialized QEMU window was available. Note: one mistaken shell grep used double quotes around a backtick-containing regex and invoked `./run-eval.sh` via command substitution; it exited immediately before QEMU with missing `sdcard-rv.img`, and the literal grep was rerun safely. |

## Summary

No honest mmap/process/signal promotion subset is ready from this lane. The useful worker-2 follow-up was to harden promotion guardrail tests around timeout, `ENOSYS`, and panic/trap markers, and to preserve the blocker-first candidate table for leader-owned serialized repair/scout work.
