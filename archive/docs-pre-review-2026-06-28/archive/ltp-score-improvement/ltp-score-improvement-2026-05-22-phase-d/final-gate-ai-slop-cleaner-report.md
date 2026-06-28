# Final gate ai-slop-cleaner report: stable250 + remote score-zero fix

Created: 2026-05-23T17:56:47Z

## Cleanup plan and behavior lock

1. Lock the delivered target with parsed full stable gates after the remote marker ANSI fix.
2. Audit the diff for masking patterns: fake PASS, case-name hardcoding, silent SKIP/TCONF conversion, timeout-as-PASS, and ungrounded fallback/workaround code.
3. Avoid further cleanup edits unless the audit finds a concrete behavior or maintainability defect; no dependency additions.
4. Keep raw logs uncommitted and record durable summaries plus the quality gate JSON.

## Result

No additional cleanup patch was needed. The task-owned changes remain behavior-grounded:

- stable250 delivery is backed by real POSIX credential, fsuid/fsgid, permission, metadata, and scheduler/process semantics rather than marker manipulation.
- remote score-zero follow-up is a console-log framing fix in `kernel/diagnostics/axlog/src/lib.rs`: the ANSI reset stays before the newline so scorer-visible `FAIL LTP CASE ... : 0` / `PASS LTP CASE ...` markers begin at column 1.
- `read02` remains explicitly visible as `pass_with_tconf`; it was not converted to a clean PASS.
- No timeout, ENOSYS, panic/trap, TFAIL, or TBROK was hidden in the final gates.

## Evidence

| Check | Evidence |
| --- | --- |
| RV full stable gate after ANSI fix | `stable250-post-ansi-rv-summary.txt`: PASS LTP CASE 500, FAIL 0; ltp-musl 250/0; ltp-glibc 250/0; TCONF 4 known `read02`; timeout/ENOSYS/panic-trap 0 |
| LA full stable gate after ANSI fix | `stable250-post-ansi-la-summary.txt`: PASS LTP CASE 500, FAIL 0; ltp-musl 250/0; ltp-glibc 250/0; TCONF 4 known `read02`; timeout/ENOSYS/panic-trap 0 |
| Remote marker shape | RV and LA post-ANSI logs each have 500 marker lines and 0 marker lines with a non-marker prefix |
| Static checks | `python3 -B scripts/test_ltp_summary.py`, `cargo fmt --all -- --check`, `make A=examples/shell ARCH=riscv64`, `make all`, offline `make all`, and diff whitespace checks passed |
| Disk monitor | `/` was 45% used; `/root/.codex` was 7.9G after the LA full gate, so no cleanup was necessary |

## Masking/fallback audit

PASS. I found no new task-owned code that:

- hardcodes case names to PASS;
- fabricates `PASS LTP CASE` / success markers;
- treats timeout as PASS;
- turns real TFAIL/TBROK into SKIP/TCONF;
- introduces an ungrounded fallback or workaround to bypass behavior.

The remote scorer fix is intentionally narrow and documented next to the code because it preserves existing log content while preventing ANSI reset leakage from corrupting line-start marker parsing.

## Remaining risk

The external remote evaluator score display cannot be proven locally. The local evidence proves that scorer-relevant marker lines now start at column 1 in RV and LA logs, and that stable250 still passes locally after that change.
