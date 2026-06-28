# Self-check compliance re-audit — 2026-06-19

## Verdict

- **Confirmed violation found and fixed:** the official-test runner used a score-aware shortcut that skipped `/glibc/libctest` because official libctest scoring is musl-only. That branch hid real glibc libctest results and violated `self-check.md`'s bans on score-only/test-environment shortcuts and hidden failures.
- **Fresh per-testcase audit result:** regenerated the audit matrix from the live source and current runner inventory: 1044 rows total, including 1000 unique stable LTP cases and 44 non-LTP/runner entries. The scan found **0 runtime case-name special-casing risks** outside selection lists, docs/comments, tests, or static guards.
- **Behavior after fix:** `/glibc/libctest` now runs and exposes real outcomes instead of being skipped. In the local RV official entrypoint partial run, `libctest-glibc` reported `177 passed, 40 failed, 2 timed out`, making failures visible rather than converting them into a score-aware skip.
- **Regression status:** official `make all` completed successfully for RV and LA artifacts. Full official score preservation is **not fully proven** in this run because Docker is unavailable and a complete local stable-LTP RV run would take hours; the partial RV run completed all non-LTP groups and 18 LTP cases before intentional QEMU termination.

## Inputs and scope

Primary rule source: `self-check.md`.

Reviewed implementation surfaces:

- `examples/shell/src/cmd.rs`: official-suite discovery, group ordering, LTP stable/default selection, libctest/LTP runners, parser/marker behavior.
- `api/arceos_posix_api/`, `kernel/`, `ulib/`, `examples/shell/src/uspace/`: syscall/user-memory/process/fs/sync surfaces through existing G002-G013 compliance guards and targeted watch-risk inspection.
- `scripts/check_g*.py` and `scripts/test_g*.py`: static regression guards that enforce no fake success, no hidden runner skips, and no scattered raw user-copy primitives.
- `/root/autotest-for-oskernel`: official remote-evaluator entrypoints and libctest judge scripts used only as reference evidence for runner behavior.

Non-scope / not modified:

- No evaluator/testsuite scripts were modified.
- No syscall semantics were loosened.
- No LTP case list was promoted or reduced.
- Pre-existing unrelated source diffs in `examples/shell/src/uspace/memory_map.rs` and `examples/shell/src/uspace/task_context.rs` were inspected as import-order-only changes and left untouched.

## `self-check.md` rule mapping

The live rule extraction is stored at `raw/g001-self-check-rules.json`. The audit treated the following as blocking violations:

1. program-name or binary-feature special casing;
2. hardcoded syscall parameter combinations, input data, directory layout, fixed path/time/order, or testcase-derived branches;
3. high-score-only partial semantics that intentionally ignore general Linux/POSIX behavior;
4. evaluator-environment exploitation;
5. broken Linux syscall compatibility or security boundaries to pass tests;
6. bypassing real process, memory, filesystem, or synchronization mechanisms through non-general shortcuts;
7. hidden `TCONF`, timeout, `ENOSYS`, panic/trap, `TFAIL`/`TBROK`, fake `TPASS`, or wrapper PASS fabrication;
8. speed/score optimizations that sacrifice compatibility, permission/resource checks, error handling, or hidden-condition safety.

## Per-testcase audit matrix

Fresh matrix: `fresh-testcase-audit-matrix.csv`.

Summary from `raw/g002-matrix-summary.json`:

| Metric | Value |
| --- | ---: |
| Stable LTP cases parsed live from `LTP_STABLE_CASES` | 1000 |
| Unique stable LTP cases | 1000 |
| Matrix rows | 1044 |
| Non-LTP / runner rows | 44 |
| Runtime case-name risk findings | 0 |
| Case-name occurrences outside stable list | 87 |

Risk classification counts:

| Classification | Count | Interpretation |
| --- | ---: | --- |
| `OK_NO_CASE_SPECIFIC_CODE` | 913 | No matched case-specific runtime code. |
| `OK_SELECTION_LIST_ONLY` | 71 | Appears only in explicit selection/list policy. |
| `OK_SELECTION_GUARD_COMMENT_ONLY` | 5 | Guard/comment/documentation occurrence only. |
| `OK_GUARD_TEST_ONLY` | 8 | Static guard or test fixture only. |
| `WATCH_COMMENT_ONLY` | 3 | Comment-only watch item, not runtime behavior. |
| `REVIEW_RUNNER_SEMANTICS` | 44 | Non-LTP/runner behavior reviewed manually. |

Family coverage:

| Family | Rows |
| --- | ---: |
| fd | 219 |
| misc | 263 |
| process | 150 |
| fs_path_metadata | 132 |
| memory | 96 |
| signal | 44 |
| net_socket | 37 |
| ipc | 32 |
| time_clock | 18 |
| thread_futex | 8 |
| namespace | 1 |
| official_non_ltp_or_runner | 44 |

Important distinction: LTP arrays and `REMOTE_LTP_CASES=stable` are selection policy and reporting inventory, not runtime testcase-name shims. They remain visible and are guarded by the matrix and parser checks.

## Confirmed violation and repair

### Violation

`examples/shell/src/cmd.rs` contained this runner branch:

```rust
if group == "libctest" && suite_dir != "/musl" {
    println!(
        "autorun: skip unscored test group {suite_dir}/{script}: official libctest score is musl-only"
    );
    continue;
}
```

Why this is a `self-check.md` violation:

- It is explicitly score-aware (`unscored`, `musl-only`) rather than Linux/POSIX-semantics-aware.
- It branches on fixed evaluator suite layout (`/musl` vs `/glibc`) to hide a whole official group.
- It hides real glibc libctest failures/timeouts instead of exposing them.
- It can preserve apparent score cleanliness while failing general libc compatibility expectations.

### Fix

Changed files:

- `examples/shell/src/cmd.rs`: deleted only the score-aware `/glibc/libctest` skip.
- `scripts/check_g005_runner_parser.py`: added a static/structural guard that rejects `skip unscored test group`, `musl-only`, the exact removed predicate, and libctest dispatch blocks that conditionally skip discovered suites or branch on fixed `/musl`/`/glibc` suite policy.
- `scripts/test_g005_runner_parser.py`: added negative regression fixtures proving the guard fails if either the exact score-aware skip or a semantically equivalent libctest suite-dir skip is reintroduced.

This keeps the kernel's scoring-capable behavior honest: successful cases still run and pass, while unsupported/failing glibc libctest cases are reported as failures or timeouts.

## Runner behavior evidence after fix

From the committed summaries `raw/g005-run-eval-rv-summary.txt` and `raw/g005-run-eval-rv-ltp-summary.json` (full local `.log` remains an ignored workspace evidence file):

- `libctest-musl` ran and reported `215 passed, 2 failed, 1 timed out`.
- `libctest-glibc` ran immediately after and reported `177 passed, 40 failed, 2 timed out`.
- All non-LTP groups through `iozone-glibc` reached their `OS COMP TEST GROUP END` marker.
- LTP then started as `ltp case list: stable (1000 cases, timeout 180s)`.
- 18 LTP cases completed before intentional QEMU termination: `access01 brk01 chdir01 clone01 close01 dup01 fcntl01 fcntl02 fork01 getpid01 mmap01 open01 pipe01 read01 stat01 wait401 write01 access03`.
- `close02` was started but not completed because QEMU was intentionally terminated to avoid spending hours in a local full-stable run.

The partial RV evidence is compliance-relevant because it demonstrates the fixed runner now exposes failures/TCONF/timeouts instead of skipping unscored groups. It is not claimed as a full-score pass.

## Watch-list items not changed in this pass

These are not confirmed violations in this diff, but remain documented review watch points:

1. **User-copy helpers** (`api/arceos_posix_api/src/utils.rs`): shared raw-pointer copy helpers centralize the current single-address-space ABI boundary. G013 prevents scattered raw pointer primitives from reappearing, but the implementation is not an MMU-backed fault-isolation proof.
2. **Evaluator staging paths** (`/tmp/ltp-work`, suite staging, busybox fallback): used by runner integration with the official test image. The audit found no per-case runtime hardcoding, but fixed path staging remains a high-visibility area for future changes.
3. **Selection lists and default stable mode**: `LTP_STABLE_CASES` and `REMOTE_LTP_CASES ?= stable` are explicit scope/selection policy. They must not be represented as broader Linux completeness.
4. **Futex/sync semantics**: static guards and prior watch notes cover honest unsupported/fallback behavior; no new futex branch was introduced here.
5. **Parser marker compatibility**: wrapper colorization and marker parsing preserve real TPASS/TFAIL/TBROK/TCONF categories. They must not be changed into synthetic PASS generation.

## Verification evidence

| Check | Result | Evidence |
| --- | --- | --- |
| Disk/tool preflight | PASS with caveat | `raw/g005-official-validation-preflight.txt`: free disk checked; QEMU tools present; Docker unavailable. |
| Official build entrypoint | PASS | `make all` returned `0`; see `raw/g005-make-all.status` and `raw/g005-make-all.status and raw/g005-make-all-postcheck.txt`. RV/LA artifacts were produced; see `raw/g005-make-all-postcheck.txt`. |
| Rust formatting | PASS | `rustfmt --check examples/shell/src/cmd.rs` in `raw/g005-final-verification.txt`. |
| G005 runner guard | PASS | `python3 scripts/check_g005_runner_parser.py --root .` in `raw/g005-final-verification.txt`. |
| G005 regression tests | PASS | `python3 -m unittest scripts/test_g005_runner_parser.py`: 10 tests OK after structural guard reinforcement in `raw/g005-final-verification.txt`. |
| Full G002-G013 static guard suite | PASS | `for f in scripts/check_g*.py; do python3 "$f" --root .; done`: all guards PASS in `raw/g005-final-verification.txt`. |
| Full G00x unittest discovery | PASS | `python3 -m unittest discover -s scripts -p 'test_g*.py'`: 146 tests OK after structural guard reinforcement in `raw/g005-final-verification.txt`. |
| Local official RV entrypoint | PARTIAL, compliance evidence only | `./run-eval.sh rv` completed all non-LTP groups and 18 LTP cases before intentional QEMU termination; see `raw/g005-run-eval-rv-summary.txt` and `raw/g005-run-eval-rv-ltp-summary.json`. |

## Score-preservation statement

- Proven: source still builds through the official `make all` RV/LA artifact path after the fix.
- Proven: musl libctest still runs; the runner no longer hides glibc libctest.
- Proven: G002-G013 anti-fake/static guard suite remains green, including the strengthened structural libctest skip guard.
- Not proven: complete official remote score equality, because Docker is unavailable locally and a complete stable-LTP RV/LA run was not completed in this session.
- Compliance priority: exposing real `/glibc/libctest` failures is required even if it makes previously hidden failures visible. This is not a scoring optimization; it removes a scoring shortcut.

## Residual risk boundary

This audit is intentionally strict, but static review cannot prove all Linux/POSIX semantic correctness. The current evidence supports:

- no detected runtime testcase-name shims across the audited stable/non-LTP matrix;
- one confirmed score-aware skip removed;
- regression guards preventing that skip from returning;
- official buildability and partial official-run behavior after the repair.

The evidence does **not** claim:

- all 1000 stable LTP cases pass in this run;
- LA runtime behavior was fully executed locally;
- Docker-based official remote scorer was available;
- every hidden semantic bug in process/memory/fs/sync is impossible.
