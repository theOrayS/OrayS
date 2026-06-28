AI SLOP CLEANUP REPORT
======================

Scope: `examples/shell/src/cmd.rs`, `scripts/check_g005_runner_parser.py`, `scripts/test_g005_runner_parser.py`, and the new audit docs under `docs/self-check-recheck-2026-06-19/`.

Behavior Lock:
- `rustfmt --check examples/shell/src/cmd.rs`
- `python3 scripts/check_g005_runner_parser.py --root .`
- `python3 -m unittest scripts/test_g005_runner_parser.py` (10 tests after structural libctest skip guard)
- `for f in scripts/check_g*.py; do python3 "$f" --root .; done`
- `python3 -m unittest discover -s scripts -p 'test_g*.py'` (146 tests after structural guard reinforcement)
- `make all`
- partial local official RV run via `./run-eval.sh rv` (non-LTP groups completed; LTP stable manifest observed; 18 LTP cases completed before intentional stop)

Cleanup Plan:
1. Keep the code change minimal: delete only the score-aware runner skip.
2. Add a guard/regression test so the exact score-aware skip cannot return.
3. Do not refactor runner ordering, testcase lists, syscall implementations, or evaluator scripts in this pass.

Fallback Findings:
- `examples/shell/src/cmd.rs`: no newly introduced fallback/slop branch. Existing TPASS/TFAIL/TBROK/TCONF colorization comment is a parser-compatibility presentation detail that preserves real LTP output categories; it was not changed here.
- `scripts/check_g005_runner_parser.py`: new score-policy and structural libctest-dispatch scans are guards, not runtime branches.
- `scripts/test_g005_runner_parser.py`: contains forbidden snippets only as negative regression fixtures; classified as test-only evidence.

Passes Completed:
- Fallback-like code resolution gate: removed score-aware `/glibc/libctest` skip and exposed real glibc libctest outcomes.
- Dead code deletion: offending branch deleted.
- Duplicate removal: not applicable; no duplicate runtime logic introduced.
- Naming/error handling cleanup: guard messages are explicit and scoped.
- Test reinforcement: added `test_detects_score_aware_libctest_skip` and `test_detects_structural_libctest_suite_dir_skip`.

Quality Gates:
- Regression/static/unit/build: PASS; see `raw/g005-final-verification.txt`, `raw/g005-final-verification.txt`, and `raw/g005-make-all.status and raw/g005-make-all-postcheck.txt`.
- Static/security scan: PASS for G002-G013 guard suite.
- Full score run: NOT COMPLETE; see `raw/g005-run-eval-rv-summary.txt` for partial RV evidence and stop reason.

Remaining Risks:
- Full RV/LA official score preservation is not proven in this run because full stable LTP would take hours locally. `make all` does prove official RV/LA buildability after the fix.
