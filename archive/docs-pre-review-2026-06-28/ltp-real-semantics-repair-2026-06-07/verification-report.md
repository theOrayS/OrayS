# G001 docs baseline verification report

Date: 2026-06-07
Worker/task: `worker-1` / Task 2
Scope: docs-only verification for `G001-g001-phase-0-quarantine`. No source code, runner, parser, `LTP_STABLE_CASES`, or `.omx/ultragoal` state is changed by this report.

## Verification summary

| Check | Result | Evidence |
| --- | --- | --- |
| Required docs present | PASS | `README.md`, `fake-implementation-inventory.md`, `promotion-quarantine.md`, and `ltp-stable-cases-evidence.md` exist under this directory. |
| README integration | PASS | README links inventory, quarantine, and stable-count evidence files. |
| Live stable count | PASS | Re-read `examples/shell/src/cmd.rs::LTP_STABLE_CASES` as `1000 total / 1000 unique / 0 duplicate entries / 0 duplicate case names`, range `examples/shell/src/cmd.rs:50-619`. |
| Parser smoke | PASS | `python3 scripts/ltp_summary.py --help` reports numeric wrapper status as source of truth and exposes promotion matrix options. |
| Parser syntax | PASS | `python3 -m py_compile scripts/ltp_summary.py` succeeded. |
| Parser tests | PASS | `python3 scripts/test_ltp_summary.py` ran 10 tests, all OK. |
| Docs sanity/lint | PASS | Python docs sanity check verified required files, README links, quarantine keywords, and stable-count evidence; `git diff --check` passed. |
| Typecheck/build | PASS | `cargo check --workspace --locked` completed successfully; only pre-existing warnings were reported. |
| Source edit boundary | PASS | `git status --short --branch --untracked-files=all` showed only docs changes before this verification commit; no source files were edited. |

## Commands

```bash
python3 scripts/ltp_summary.py --help
python3 -m py_compile scripts/ltp_summary.py
python3 scripts/test_ltp_summary.py
python3 - <<'PY'  # docs/stable-count sanity verifier
# Re-read examples/shell/src/cmd.rs::LTP_STABLE_CASES and assert total=unique=1000, duplicates=0.
PY
git diff --check
cargo check --workspace --locked
git status --short --branch --untracked-files=all
```

## Caveats

- QEMU/LTP runtime was not run because G001 is documentation/baseline-only.
- `blacklist`, `score-blacklist`, `stable-plus-blacklist`, status0-only, full-sweep partial `TPASS`, synthetic probe-only evidence, and case-specific runner behavior remain quarantined for promotion.
- `TCONF`, `TBROK`, `TFAIL`, `ENOSYS`, timeout, panic/trap, marker-prefix, and truncation caveats must remain visible in future gates.
