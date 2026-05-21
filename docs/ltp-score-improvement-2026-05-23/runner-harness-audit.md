# Runner/harness audit: LTP stable, batch, file, and summary semantics

## Scope

Worker-3 audited `examples/shell/src/cmd.rs` and `scripts/ltp_summary.py` for:

- stable/batch/file LTP case selection semantics;
- timeout-as-fail behavior;
- PASS/FAIL emission and parser behavior;
- hardcoded PASS or silent-SKIP risk;
- `.omx/ultragoal` mutation avoidance.

## Verdict

PASS: the runner emits wrapper-level PASS/FAIL records from actual per-case process results, treats timeout as failure, and preserves internal LTP signals for post-run classification. The summary tool does not rely on `run-eval` exit status; it counts wrapper PASS/FAIL plus TFAIL/TBROK/TCONF, timeout, ENOSYS, and panic/trap markers.

No `.omx/ultragoal` files were created or modified by this worker lane.

## Runner findings (`examples/shell/src/cmd.rs`)

### Stable and batch lists

- `LTP_STABLE_CASES` is an explicit stable allowlist at `examples/shell/src/cmd.rs:49-180`; current size is 63 cases and includes the known transparent `read02` case.
- Named batches are separate arrays for `syscalls-basic-plus`, `fs-basic`, `proc-basic`, and `time-signal-basic`, registered in `LTP_CASE_BATCHES` at `examples/shell/src/cmd.rs:183-190`.
- Audit note: explicit stable promotion is intentional, but every addition to `LTP_STABLE_CASES` must be backed by fresh LA/RV x musl/glibc evidence because the list directly controls the default evaluator run.

### Case selection and file/inline semantics

- `selected_ltp_cases()` first checks `/ltp_cases.txt` or `/tmp/ltp_cases.txt`, then compile-time `LTP_CASES`, then defaults to `stable` at `examples/shell/src/cmd.rs:563-612`.
- Supported forms are:
  - empty or `stable` -> `LTP_STABLE_CASES`;
  - `core` -> `LTP_CORE_CASES`;
  - `batch:<name>` -> a named static batch;
  - `file:<path>` -> newline/comma/whitespace separated cases from a file;
  - inline comma/whitespace list -> direct targeted cases.
- `push_ltp_case()` validates names and deduplicates them at `examples/shell/src/cmd.rs:517-525`; invalid names abort selection rather than becoming shell text.
- Silent fallback risk: if inline parsing yields no cases after earlier selectors do not match, `selected_ltp_cases()` falls back to `core` at `examples/shell/src/cmd.rs:612`. Targeted runs should log the printed `ltp case list:` line and fail review if it does not match the intended list.
- `selected_official_test_groups()` returns `None` on parse failure at `examples/shell/src/cmd.rs:630-636`, which means all groups may run implicitly; this is not a false PASS, but it can hide a group-filter typo in long logs.
- Audit note: file-based selection enables targeted batches without editing stable lists. This is good for discovery, but selected files must be logged in run artifacts.

### Timeout-as-fail and PASS/FAIL emission

- Default per-case timeout is 10s via `LTP_CASE_TIMEOUT_SECS`, overrideable by `/ltp_case_timeout_secs` or compile-time `LTP_CASE_TIMEOUT_SECS` at `examples/shell/src/cmd.rs:506` and `examples/shell/src/cmd.rs:616-624`.
- `rewrite_ltp_case_line()` wraps LTP shell case execution in a `setsid` process group watchdog and emits `TIMEOUT LTP SCRIPT` before killing the group at `examples/shell/src/cmd.rs:971-985`.
- `run_ltp_suite()` emits `RUN LTP CASE`, `PASS LTP CASE <case> : 0`, or `FAIL LTP CASE <case> : <status>` from the actual process result at `examples/shell/src/cmd.rs:1413-1466`.
- Exit statuses 137 and 143 are counted as failed and timed out, with `TIMEOUT LTP CASE` printed at `examples/shell/src/cmd.rs:1451-1456`; timeout errors also emit `FAIL LTP CASE <case> : -1` plus `TIMEOUT LTP CASE` at `examples/shell/src/cmd.rs:1461-1466`.
- Missing testcase files emit `FAIL LTP CASE <case> : -1`, not skip/pass, at `examples/shell/src/cmd.rs:1418-1425`.
- The suite summary prints passed/failed/timed-out totals at `examples/shell/src/cmd.rs:1483`.

### Hardcode/silent-skip risks

- No hardcoded PASS path was found in `run_ltp_suite()`; PASS is emitted only for `Ok(0)` from `run_user_program_argv_in_timeout()`.
- Known case-specific environment exists for `chdir01` at `examples/shell/src/cmd.rs:1168-1174`; this is a semantic scratch-device adaptation, not a PASS override. It should remain documented when interpreting `chdir01` evidence.
- `run_ltp_suite()` returns `Ok(())` after recording per-case failures, and the auto-run wrapper ultimately exits 0 after printing diagnostics. This means the top-level evaluator can exit successfully even with LTP failures; consumers must use `scripts/ltp_summary.py` instead of wrapper exit status alone.

## Summary-tool findings (`scripts/ltp_summary.py`)

### Parser coverage

- The module docstring explicitly says the tool counts wrapper PASS/FAIL plus internal LTP quality signals so `RUN_EVAL_DEFAULT_STATUS=0` is not mistaken for clean LTP at `scripts/ltp_summary.py:1-6`.
- Regexes parse group boundaries, `RUN LTP CASE`, wrapper `PASS/FAIL LTP CASE`, `TIMEOUT LTP CASE`, runtime, memory, `TFAIL/TBROK/TCONF`, ENOSYS/not-implemented text, and panic/trap markers at `scripts/ltp_summary.py:17-39`.
- `parse_log()` attributes internal markers and timeout/ENOSYS/panic-trap signals to the current case when possible at `scripts/ltp_summary.py:106-203`.

### Timeout and failure classification

- Timeout markers increment both global/group counts and the current case bucket at `scripts/ltp_summary.py:187-191`.
- `compact()` classifies each row into `pass_clean`, `pass_with_tconf`, `fail_wrapper`, `internal_tfail`, `internal_tbrok`, `timeout`, `enosys`, `panic_trap`, and `unknown` at `scripts/ltp_summary.py:256-288`.
- `row_problem_markers()` treats TFAIL, TBROK, TCONF, timeout, ENOSYS, panic/trap, event failures, and any non-PASS status as blockers at `scripts/ltp_summary.py:315-329`; this keeps timeout/TCONF from becoming silent promotion even when wrapper status is PASS.
- `promotion_report()` only promotes a case when all required arch/libc combinations are present and no blocker exists at `scripts/ltp_summary.py:333-401`. The default required matrix is `rv,la` x `musl,glibc`, so non-standard logs or new arches must pass explicit `--promotion-arches/--promotion-libcs` flags instead of relying on defaults.

### Output semantics

- Standard Markdown output includes PASS count, FAIL count, internal marker total, timeout count, ENOSYS count, panic/trap count, suite summaries, case matrix, categories, and per-group details at `scripts/ltp_summary.py:476-554`.
- Multi-log promotion mode requires `--promotion-candidates`; otherwise multiple logs are rejected at `scripts/ltp_summary.py:581-582`, preventing accidental mixed-log summaries.

## Recommendations for this LTP score campaign

1. Use `file:<path>` or `/ltp_cases.txt` for first-pass targeted batches; do not edit `LTP_STABLE_CASES` until evidence is clean.
2. Save raw logs plus `scripts/ltp_summary.py` Markdown/JSON for each LA/RV targeted run.
3. Treat any wrapper `FAIL LTP CASE`, timeout marker, TFAIL, TBROK, TCONF, ENOSYS, panic/trap, or missing arch/libc combo as a promotion blocker unless explicitly documented as an existing transparent exception like `read02`.
4. Keep `.omx/ultragoal` checkpointing leader-only; workers should report through task lifecycle results and docs under `docs/ltp-score-improvement-2026-05-23/`.
5. Reject targeted validation runs when the logged `ltp case list:` does not match the intended inline/file/batch selector; this catches the current `core` fallback risk.

## Verification commands

- `python3 -m py_compile scripts/ltp_summary.py`
- `python3 scripts/ltp_summary.py output_rv.md`
- `python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc output_la.md output_rv.md`
- `git diff --check -- docs/ltp-score-improvement-2026-05-23/runner-harness-audit.md`
- `git status --short -- .omx && git diff --name-only -- .omx/ultragoal`
