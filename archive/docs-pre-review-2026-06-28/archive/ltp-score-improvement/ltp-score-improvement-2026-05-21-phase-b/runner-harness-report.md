# Runner/Harness Audit: reproducible LTP batch runner

Worker: `worker-3`  
Task: `8` / Runner-Harness audit  
Date: 2026-05-21

## Scope

Audit target: `examples/shell/src/cmd.rs` LTP runner behavior for:

- stable / batch / file / inline LTP case selection
- reproducible case-list and timeout configuration
- timeout counted as `FAIL` plus explicit `TIMEOUT`, not `PASS`
- no fake PASS and no silent SKIP for requested cases
- minimal patch only if a real runner gap is found

This task did not edit `.omx/ultragoal`.

## Verdict

No runner code patch is needed for the audited requirements. The current runner already has a reproducible stable set, named exploratory batches, file and inline case-list configuration, and per-case timeout accounting that increments failure and timeout counters separately.

The deliverable is this report rather than a runner patch because the audited code path already satisfies the task constraints and a speculative patch would increase integration risk without evidence of a concrete failure.

## Evidence from `examples/shell/src/cmd.rs`

### Case-set configuration

`cmd.rs` defines explicit case sets:

| Case set | Location | Count | Purpose |
| --- | ---: | ---: | --- |
| `LTP_CORE_CASES` | `examples/shell/src/cmd.rs:44` | 16 | original core baseline |
| `LTP_STABLE_CASES` | `examples/shell/src/cmd.rs:49` | 44 | current default stable scoring batch |
| `LTP_SYSCALLS_BASIC_PLUS_CASES` | `examples/shell/src/cmd.rs:96` | 20 | exploratory syscall-adjacent batch |
| `LTP_FS_BASIC_CASES` | `examples/shell/src/cmd.rs:119` | 17 | exploratory filesystem batch |
| `LTP_PROC_BASIC_CASES` | `examples/shell/src/cmd.rs:139` | 1 | proc probe batch |
| `LTP_TIME_SIGNAL_BASIC_CASES` | `examples/shell/src/cmd.rs:141` | 19 | exploratory time/signal batch |

`LTP_CASE_BATCHES` maps stable names to those lists at `examples/shell/src/cmd.rs:163`.

Static parse check found no duplicate case names in these lists.

### Selection semantics

`selected_ltp_cases()` (`examples/shell/src/cmd.rs:544`) selects cases in this order:

1. Runtime file `/ltp_cases.txt` or `/tmp/ltp_cases.txt`, reported as `file:<path>`.
2. Build-time `LTP_CASES` option.
3. Default `stable` batch when unset or empty.
4. Explicit `core` compatibility mode.
5. Named `batch:<name>` from `LTP_CASE_BATCHES`.
6. Explicit `file:<path>` case list.
7. Inline comma/whitespace case list.

`split_ltp_case_list()` strips `#` comments and accepts comma or whitespace delimiters (`examples/shell/src/cmd.rs:509`). `push_ltp_case()` deduplicates cases and rejects invalid names before insertion (`examples/shell/src/cmd.rs:500`).

This is reproducible because the runner prints the resolved case-list name and case count before running: `ltp case list: {case_list_name} (...)` at `examples/shell/src/cmd.rs:1382`.

### Timeout semantics

`ltp_case_timeout_secs()` reads runtime `/ltp_case_timeout_secs` first, then build-time `LTP_CASE_TIMEOUT_SECS`, then defaults to `10` seconds (`examples/shell/src/cmd.rs:597`; default at `examples/shell/src/cmd.rs:487`).

`run_ltp_suite()` executes each case with `run_user_program_argv_in_timeout(..., timeout_secs)` (`examples/shell/src/cmd.rs:1414` and `examples/shell/src/cmd.rs:1425`). Timeout-like results are treated as failures:

- status `137` / `143`: prints `FAIL LTP CASE ...`, then `TIMEOUT LTP CASE ...`, increments both `failed` and `timed_out` (`examples/shell/src/cmd.rs:1433`).
- timeout error string: prints `FAIL LTP CASE ... : -1`, prints `TIMEOUT LTP CASE ...`, increments both `failed` and `timed_out` (`examples/shell/src/cmd.rs:1443`).
- final suite summary includes timed-out count: `ltp cases: {passed} passed, {failed} failed, {timed_out} timed out` (`examples/shell/src/cmd.rs:1464`).

Therefore timeout is not counted as PASS.

### PASS/FAIL semantics and no silent SKIP

For every selected case, `run_ltp_suite()` prints `RUN LTP CASE {case}` before execution (`examples/shell/src/cmd.rs:1395`). Outcomes are explicit:

- `Ok(0)` only path to `PASS LTP CASE` (`examples/shell/src/cmd.rs:1428`).
- nonzero process status prints `FAIL LTP CASE` (`examples/shell/src/cmd.rs:1433` and `examples/shell/src/cmd.rs:1439`).
- missing testcase binary prints `FAIL LTP CASE ... : -1` plus `missing ltp testcase` (`examples/shell/src/cmd.rs:1398`).
- execution errors print `FAIL LTP CASE ... : -1` plus the error (`examples/shell/src/cmd.rs:1443`).

No audited branch converts a requested case into PASS by name, and missing or failing selected cases are not silently skipped.

### Script wrapper compatibility

The older script-driven LTP path also has a watchdog wrapper in `rewrite_ltp_case_line()` (`examples/shell/src/cmd.rs:955`). It runs script cases in a process group, emits `TIMEOUT LTP SCRIPT`, and kills the process group on timeout instead of blocking the whole evaluation. The main `run_ltp_suite()` direct binary path is now the relevant scoring path, but this wrapper is consistent with the same bounded-timeout policy.

## Current evidence artifacts

Existing raw evidence under `docs/ltp-score-improvement-2026-05-22/raw/` was inspected:

- `cmd-rs-ltp-batches.txt` records the parsed case-set names and counts.
- `output_la-current-summary.txt` and `output_rv-current-summary.txt` show the current summaries still parse timeout separately and preserve internal marker categories.

The current LA/RV summaries show the old full-output group timeout noise outside the LTP case matrix, but per-case LTP rows report timeout `0` for the current stable/core pass set. Promotion logic should continue to use per-case rows, not global timeout counters.

## Risks / follow-up recommendations

1. Multi-log promotion should be owned by stats/report tooling, not runner code. The runner emits enough per-case evidence; cross-arch promotion should require LA/RV x musl/glibc summaries.
2. Unknown `batch:<name>` fails visibly through `autorun: ltp suite failed: ...`; this is acceptable for misconfiguration, but a future UX patch could print known batches in the final report artifact.
3. `proc-basic` contains `proc01` only. More `/proc` cases require runtest command-line support or manual argv probes; adding labels without argv support would be premature.
4. Keep stable promotion gated by real LA/RV evidence. Do not promote `syscalls-basic-plus`, `fs-basic`, or `time-signal-basic` cases solely because they are listed in candidate batches.

## Verification commands

The following checks were run for this audit:

```sh
# Inspect runner anchors and relevant implementation blocks
grep -n "LTP_CORE_CASES\|LTP_.*CASE\|run_ltp\|timeout\|RUN LTP CASE\|PASS LTP CASE\|FAIL LTP CASE\|TIMEOUT LTP CASE\|case-list\|batch\|runtest\|RUN_EVAL" examples/shell/src/cmd.rs
nl -ba examples/shell/src/cmd.rs | sed -n '44,176p;500,606p;1374,1458p;1560,1600p'

# Parse LTP case-list constants and check duplicate entries
python3 - <<'PY'
from pathlib import Path
import re
s=Path('examples/shell/src/cmd.rs').read_text()
consts=re.findall(r'const (LTP_[A-Z0-9_]+_CASES): &\\[&str\\] = &\\[(.*?)\\];', s, re.S)
for name, body in consts:
    cases=re.findall(r'"([^"]+)"', body)
    print(name, len(cases), 'dups=', len(cases)-len(set(cases)))
PY
```

Results: all audited LTP case-list constants parsed successfully with zero duplicates; static code inspection confirms timeout is failure-counted and no selected case has a fake-PASS/silent-SKIP branch.

## Verification results from this worker

- PASS: static runner inspection found explicit `RUN LTP CASE`, `PASS LTP CASE`, `FAIL LTP CASE`, `TIMEOUT LTP CASE`, and final timed-out suite summary print paths in `examples/shell/src/cmd.rs`.
- PASS: Python constant parser found all six LTP case-list constants and zero duplicate cases in each list.
- PASS: report content check found the required verdict, timeout semantics, PASS/FAIL semantics, and verification sections.
- PASS: `rustfmt --edition 2021 --check examples/shell/src/cmd.rs` completed successfully.
- FAIL (environment/worktree metadata, not code): `cargo fmt --all -- --check` from this OMX nested worktree failed before formatting because Cargo resolved `vendor/rust-fatfs` as belonging to the leader workspace at `/root/oskernel2026-orays/Cargo.toml`. No source formatting diagnostics were produced.
- NOT RUN: full `./run-eval.sh` / full LTP, by task constraint to use static checks or targeted commands and avoid full eval for this audit.
