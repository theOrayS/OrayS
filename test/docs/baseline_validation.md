# Baseline and official validation evidence

This report records the fail-closed validation of the unified local test suite.
Generated stdout, stderr, JSON summaries, qcow2 overlays, and large images remain
under the ignored `test/output/` tree or outside the repository; none is committed.
A nonzero result below remains non-passing even when all planned cases completed.

All timestamps copied from runner summaries are UTC.

Status at this documentation revision: implementation commit
`b34881941843892d6e7907a976a7306c80f94f16` has complete clean-worktree quick,
cwd-independent quick, baseline, and RISC-V64 official evidence. Quick and
baseline remain honestly non-passing because of unchanged production failures
and one host capability error. The complete RISC-V64 run is an infrastructure
error because its raw stdout contains a NUL byte. A LoongArch64 run was
interrupted by an execution-environment transition, and its clean retry was
then stopped at the operator's request; neither incomplete directory is a
verdict or PASS. Older completed runs below are retained only as historical
blocker evidence and are not represented as current final evidence.

## Revision boundary

- Unchanged production baseline:
  `921171ac1ef5c85ab5a7cd1882dd40e1471b79f0` (`origin/main` and `HEAD` before
  the first tracked migration edit).
- Eight logical migration/framework and review-hardening commits precede this
  final evidence report. Their exact IDs and subjects are recorded below.
- Earlier local profile evidence used runner commit `d11586c9`; earlier long
  official runs used `07fce567`. Those runs predate the current identity-binding,
  lifecycle, UTF-8, isolation, and provenance hardening and therefore cannot be
  promoted to current final evidence.
- Current canonical evidence is allowed only from a clean worktree. The runner
  records and compares starting/final `HEAD` and porcelain status, uses an
  isolated no-site first interpreter with a non-repository bytecode-cache
  prefix, and requires every canonical unittest method to have an exact runtime
  binding. The closed case environment, unittest harness, and low-level official
  adapter propagate `PYTHONDONTWRITEBYTECODE=1` and
  `PYTHONPYCACHEPREFIX=/dev/null`, so their default nested Python interpreters do
  not read or write repository-local caches; every canonical explicitly isolated
  Python command also carries its own cache flags. The runner does not separately
  scan for ignored cache files.

No production semantic edit is used to change a result. The unchanged RR
scheduler finding, formatting drift, crate-test failure, and lint failures remain
visible.

## Host tool snapshot

Every version command below exited 0 on 2026-07-15:

| Command | Observed version |
| --- | --- |
| `python3 --version` | Python 3.10.12 |
| `cargo --version` | cargo 1.89.0-nightly (`47c911e9e`, 2025-05-14) |
| `rustc --version` | rustc 1.89.0-nightly (`60dabef95`, 2025-05-19) |
| `rustup --version` | rustup 1.29.0 (`28d1352db`, 2026-03-05) |
| `make --version` | GNU Make 4.3 |
| `qemu-system-riscv64 --version` | QEMU 6.2.0 (Debian `1:6.2+dfsg-2ubuntu6.30`) |
| `qemu-system-loongarch64 --version` | QEMU 9.2.4 |
| `qemu-img --version` | qemu-img 9.2.4 |
| `git --version` | Git 2.34.1 |

The two locally supplied raw ext4 official images were each exactly
4,294,967,296 bytes and readable. No image was downloaded or modified in place.
The completed RISC-V64 adapter run used a unique qcow2 overlay and removed it at
exit. The interrupted LoongArch64 attempts left two 196,672-byte ignored qcow2
overlays under their generated output directories; they are neither tracked nor
result evidence, and the base image remains untouched.

## Legacy pre-edit baseline

Before any tracked edit, every legacy asset and reference was inventoried with
`git ls-files` and `git grep`, and the existing checks were executed:

- 13 legacy static guards: 12 passed. The state-backed guard first exposed a
  missing ignored vendor source and then reported 33 findings once the existing
  dependency was available.
- 14 legacy Python unit scripts, 192 methods: 155 passed, 2 assertions failed,
  and 35 errored. These outcomes are mapped row by row in `migration_map.md`;
  none was silently deleted or reclassified as PASS.
- The exact pre-edit repository commands produced:

| Command | Exit | Duration | Honest result |
| --- | ---: | ---: | --- |
| `make unittest_no_fail_fast` | 2 | 91.73 s | axfs FAT `test_devfs_ramfs()` failed with `NotFound` |
| `cargo fmt --all -- --check` | 1 | 5.12 s | existing production formatting drift |
| `make all` | 0 | 289.49 s | complete success |

## Resulting test structure

```text
test/
├── README.md
├── run_official_suite.sh
├── run_suite.py
├── run_unittest_suite.py     # exact runtime identity binder for canonical units
├── suite_manifest.json
├── checks/                 # 18 registered semantic guards
├── unit/                   # 22 registered suites, 532 methods
├── fixtures/               # static positive/negative contracts
├── evaluation/
│   ├── config/
│   ├── official_case_plan.json
│   ├── run_official_evaluation.sh
│   ├── parse_official_results.py
│   ├── summarize_ltp_results.py
│   └── report_evaluation_failures.py
├── docs/
│   ├── migration_map.md
│   └── baseline_validation.md
└── output/.gitignore       # all generated evidence remains ignored
```

The root `run-eval.sh` is retained only as an executable compatibility shim. It
executes `test/run_official_suite.sh`, which invokes the strict manifest runner;
the low-level adapter cannot by itself award official PASS.

## Migration and new coverage counts

- 33/33 inventoried legacy test/evaluation assets have an explicit row in
  `migration_map.md` and a semantic destination or documented compatibility role.
- 13 legacy guards became 16 focused registered checks after the broad
  timer/memory-policy/socket guard was split and test-asset integrity was added.
- 14 legacy unit scripts contained 192 methods. Their 16 migrated semantic
  families now contain 209 methods, adding 17 focused regression fixtures
  without weakening a legacy assertion.
- Four wholly new suites contain 283 methods:
  evaluation failure reporter 8, official result validator 106, manifest runner
  133, and test-asset integrity 36.
- The unified-suite source branch contained 20 suites and 492 methods. Relative to the 192
  legacy methods, 300 methods are newly added (17 migrated-family expansions plus
  283 methods in new suites).
- Four-branch integration adds the Linux-boundary and file-object/event guards plus
  their 16- and 24-method mutation suites. The current canonical total is 18 checks,
  22 unit suites, and 532 exactly bound unit methods.
- The current manifest registers 50 semantic cases. `quick` plans 40 cases and
  `baseline` plans 48 cases. The earlier 46/36/44 source-branch totals below remain
  historical evidence for commit `b3488194`, not current integration evidence.

## Focused implementation validation

These post-compatibility commands completed without QEMU:

| Exact command | Status |
| --- | --- |
| `bash -n run-eval.sh test/run_official_suite.sh test/evaluation/run_official_evaluation.sh` | exit 0 |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py test/unit/test_evaluation_runner_and_parser_integrity.py` | 23/23 PASS |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py test/unit/test_evaluation_failure_report.py` | 8/8 PASS; paired stdout/stderr and strict empty/truncated lifecycle rejection |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py test/unit/test_ltp_result_summary.py` | 20/20 PASS; strict capture pairing and SHA-256 provenance |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py test/unit/test_suite_runner.py` | 133/133 PASS; exact runtime binding, cleanup-alias, exit-contract, timeout/process and manifest integrity fixtures |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py test/unit/test_test_asset_integrity.py` | 36/36 PASS; canonical inventory, wrapper, naming, output-hygiene and mutation fixtures |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py test/unit/test_official_result_validation.py` | 106/106 PASS |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py test/unit/test_runtime_binary_patch_prohibition.py` | 9/9 PASS; isolated promotion command and process-evidence contract mutations |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/checks/check_evaluation_runner_and_parser_integrity.py` | PASS, 0 findings |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/checks/check_test_asset_integrity.py` | PASS, 0 findings |
| `python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --list` | exit 0, 46 registered cases |
| `cd /tmp && python3 -I -S -B -X pycache_prefix=/dev/null <worktree>/test/run_suite.py --list` | exit 0, same 46-case plan |

The units prove that a child exit 0, empty/missing/malformed suite, duplicate ID,
unknown state, incomplete lifecycle, timeout, crash, signal exit, skipped case,
nonzero embedded result, startup-hook early exit, or planned/executed mismatch
cannot be counted as PASS. New static wrapper rules have both the current-tree
positive fixture and mutations for missing, non-executable, non-delegating,
policy-bearing, and duplicated-logic entrypoints.

## Logical implementation commits

The test-only branch contains these reviewable commits after the unchanged
baseline. The final documentation commit that contains this report necessarily
follows the tested implementation commit; the handoff records its exact hash
because a commit cannot embed its own content-dependent hash.

| Commit | Subject |
| --- | --- |
| `ed3e95c7` | `test: unify local test infrastructure` |
| `6910752f` | `test: harden zero-exit result classification` |
| `94a87c28` | `test: close official evaluation trust gaps` |
| `07fce567` | `test: isolate official build environment` |
| `bf841332` | `test: preserve official entry compatibility` |
| `d11586c9` | `docs: document unified local test suite` |
| `706c761d` | `test: close final evidence integrity gaps` |
| `b3488194` | `test: fix unittest binding and signal fixture ownership` |

## Final clean commit-attributed profile evidence

Every completed row below ran with starting and final runner commit
`b34881941843892d6e7907a976a7306c80f94f16`, clean starting and final porcelain
status, and `runner_provenance_stable: true`. The explicit environment prevents
nested Python bytecode caches from becoming executable test inputs. Evidence
paths are relative and ignored.

| Exact portable command | Duration | Complete totals | Exit/result | Evidence |
| --- | ---: | --- | --- | --- |
| `PYTHONDONTWRITEBYTECODE=1 PYTHONPYCACHEPREFIX=/dev/null PYTHONNOUSERSITE=1 python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile quick --output-dir test/output/final-quick-b3488194` | 252.765473 s | 36/36 executed/completed; 34 PASS, 2 FAIL, 0 INFRA | 1 / FAIL | `test/output/final-quick-b3488194/summary.json` |
| `cd /tmp && PYTHONDONTWRITEBYTECODE=1 PYTHONPYCACHEPREFIX=/dev/null PYTHONNOUSERSITE=1 python3 -I -S -B -X pycache_prefix=/dev/null <worktree>/test/run_suite.py --profile quick --output-dir <worktree>/test/output/final-quick-cwd-b3488194` | 253.627179 s | 36/36 executed/completed; 34 PASS, 2 FAIL, 0 INFRA | 1 / FAIL | `test/output/final-quick-cwd-b3488194/summary.json` |
| `PYTHONDONTWRITEBYTECODE=1 PYTHONPYCACHEPREFIX=/dev/null PYTHONNOUSERSITE=1 python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile baseline --output-dir test/output/final-baseline-b3488194` | 451.608015 s | 44 planned/completed, 43 executed; 38 PASS, 5 FAIL, 1 INFRA | 2 / INFRA_ERROR | `test/output/final-baseline-b3488194/summary.json` |
| `RV_TESTSUITE_IMG=<OFFICIAL_IMAGE_DIR>/sdcard-rv.img PYTHONDONTWRITEBYTECODE=1 PYTHONPYCACHEPREFIX=/dev/null PYTHONNOUSERSITE=1 python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile official --arch rv --output-dir test/output/final-official-rv-b3488194` | 3544.815866 s | 1/1/1 planned/executed/completed; 0 PASS, 1 INFRA | 2 / INFRA_ERROR | `test/output/final-official-rv-b3488194/summary.json` |

Both quick runs have the same two explicit failures and no timeout, crash,
infrastructure error, unknown state, or not-run case. In each run,
`unit.suite_runner` independently completed all 133 exact runtime-bound methods
and returned PASS. This closes the post-commit integration failure in which a
binding record had been attached to unittest progress output and an intentionally
signalled harness fixture had leaked its two relay zombies to the outer runner.

### Current quick failure details

- `check.kernel_state_backed_semantics` exits 1 and reports ten missing RR
  skipped-task aging contracts in `vendor/axsched/src/round_robin.rs`.
- `unit.kernel_state_backed_semantics` runs 36 methods; 35 pass and the
  current-tree assertion fails on the same ten contracts.

Changing scheduler production behavior, deleting the assertion, or suppressing
the finding is outside the test-only scope. The two results therefore remain
FAIL. The cwd-independent run reproduces the same result rather than a path or
discovery error.

### Current baseline details

The baseline repeats those two quick failures and adds these explicit outcomes:

| Case | Executed/status | Exit | Duration | Cause or result |
| --- | --- | ---: | ---: | --- |
| `baseline.cargo_format` | yes / FAIL | 1 | 1.983321 s | existing rustfmt drift |
| `baseline.workspace_unit_tests` | yes / FAIL | 2 | 1.181599 s | axfs `test_fatfs`: `test_devfs_ramfs()` returned `NotFound` |
| `baseline.clippy_default` | yes / FAIL | 2 | 4.992822 s | existing `arceos-shell` cross-architecture compile errors |
| `baseline.clippy_riscv64` | yes / PASS | 0 | recorded in summary | complete success |
| `baseline.clippy_loongarch64` | no / INFRA_ERROR | n/a | 0.431435 s | the resolved host clang rejects `loongarch64-unknown-none`; the unavailable capability is not execution |
| `baseline.kernel_riscv64` | yes / PASS | 0 | recorded in summary | complete success |
| `baseline.kernel_loongarch64` | yes / PASS | 0 | recorded in summary | complete success |
| `baseline.submission_build` | yes / PASS | 0 | recorded in summary | complete success |

The one unexecuted capability-gated case explains why planned/completed are
44/44 while executed is 43. It is an infrastructure error, not a skip or PASS.

### Current official RISC-V64 result

The clean current RISC-V64 run reached guest shutdown and returned child exit 0,
but strict result validation produced `INFRA_ERROR` and suite exit 2:

- planned/executed/completed: 1/1/1; PASS 0, INFRA_ERROR 1;
- case duration 3544.716743 s;
- strict reason: `captured output is malformed: stdout contains unsupported
  output character U+0000`;
- 2,378,016-byte stdout contains exactly one NUL at byte offset 3918;
- stderr contains 12,230 bytes and no NUL;
- raw framing contains 24/24 group START/END markers.

The complete group framing and zero child exit prove that the local path ran;
they do not override malformed capture or the real failures inside the groups.
This is not official PASS.

### Current LoongArch64 attempt boundary

There is no completed current-code LoongArch64 verdict. Two generated directories
are deliberately excluded from result tables:

1. `test/output/final-official-la-b3488194/summary.json` started clean at
   `2026-07-15T07:58:05.262355Z`, but an execution-environment transition
   terminated its process namespace. Its summary remains `RUNNING` with no
   final commit/status fields. The 2,383,396-byte stdout stops at 15 group START
   and 14 group END markers.
2. `test/output/final-official-la-retry-b3488194/summary.json` started clean at
   `2026-07-15T09:03:27.041051Z`. The operator then requested that this path no
   longer be run, so the waiting shell/session was interrupted and returned 130.
   Its summary itself remains `RUNNING`; the 7,477-byte stdout has 1 group START
   and 0 group END markers.

Neither incomplete summary has an executed/completed case, final provenance, or
exit verdict. They are interruption diagnostics only, never official evidence.
The locally available image remains known and readable, but the operator's
explicit stop instruction supersedes further execution in this handoff. The
older completed LA run below remains useful blocker history, not current-code
validation.

## Earlier pre-hardening local profile evidence

These completed runs are retained to show the unchanged production failures.
They predate the current isolation, exact unittest identity binding, and final
provenance checks, so they are not final evidence for the current implementation.

| Exact command | Runner commit | Duration | Complete totals | Exit/result | Evidence |
| --- | --- | ---: | --- | --- | --- |
| `python3 test/run_suite.py --profile checks` | `d11586c9` | 1.576722 s | 16/16 executed/completed; 15 PASS, 1 FAIL | 1 / FAIL | `test/output/20260715T011604Z-checks-none-3245603/summary.json` |
| `python3 test/run_suite.py --profile unit` | `d11586c9` | 48.076294 s | 20/20 executed/completed; 19 PASS, 1 FAIL | 1 / FAIL | `test/output/20260715T011610Z-unit-none-3245640/summary.json` |
| `python3 test/run_suite.py --profile quick` | `d11586c9` | 49.570719 s | 36/36 executed/completed; 34 PASS, 2 FAIL | 1 / FAIL | `test/output/20260715T010731Z-quick-none-3238653/summary.json` |
| `cd /tmp && python3 <worktree>/test/run_suite.py --profile quick` | `d11586c9` | 49.527927 s | 36/36 executed/completed; 34 PASS, 2 FAIL | 1 / FAIL | `test/output/20260715T010828Z-quick-none-3239887/summary.json` |
| `python3 test/run_suite.py --profile baseline` | `d11586c9` | 243.009753 s | 44/44 executed/completed; 38 PASS, 6 FAIL | 1 / FAIL | `test/output/20260715T010925Z-baseline-none-3241135/summary.json` |

Every row has zero timeout, crash, infrastructure error, unknown status, and
`NOT_RUN` cases. The two quick runs have identical results from different
working directories.

### Why quick is not green

Both quick failures are explicit unchanged-production defects:

- `check.kernel_state_backed_semantics` reports ten missing RR skipped-task
  aging contracts in `vendor/axsched/src/round_robin.rs`.
- `unit.kernel_state_backed_semantics` executes 36 methods: 35 pass and the
  real-current-tree assertion fails on the same ten contracts.

Making quick green would require either changing scheduler production behavior,
weakening/removing the guard, or suppressing a real failure. All three are
forbidden by this test-only task. Therefore the goal's literal “quick passes”
criterion is unattainable within the authorized scope. The suite instead proves
complete 36/36 execution and keeps the two failures visible; it does not claim
quick PASS.

### Baseline case details

The earlier baseline repeats the same two quick failures and records four existing
repository failures:

| Case | Exit | Duration | Explicit cause |
| --- | ---: | ---: | --- |
| `baseline.cargo_format` | 1 | 2.030295 s | rustfmt diffs in existing production files including pipe, axfs, and wait-queue sources |
| `baseline.workspace_unit_tests` | 2 | 1.378776 s | axfs `test_fatfs` panicked because `test_devfs_ramfs()` returned `NotFound` |
| `baseline.clippy_default` | 2 | 5.088853 s | `arceos-shell` failed with 42 existing cross-architecture compile errors, including missing TrapFrame fields and type mismatches |
| `baseline.clippy_loongarch64` | 2 | 1.528656 s | bindgen/libclang rejected target triple `loongarch64-unknown-none` |

The remaining deterministic build/lint cases explicitly passed:

| Case | Exit | Duration | Result |
| --- | ---: | ---: | --- |
| `baseline.clippy_riscv64` | 0 | 1.137950 s | PASS |
| `baseline.kernel_riscv64` | 0 | 47.615594 s | PASS |
| `baseline.kernel_loongarch64` | 0 | 44.476983 s | PASS |
| `baseline.submission_build` (`make all`) | 0 | 90.283878 s | PASS |

These historical durations reflect the local build cache state; status and captured
stdout/stderr, not timing, are the result contract.

## Earlier pre-hardening official RISC-V64 blocker evidence

Portable reproduction command (set `OFFICIAL_IMAGE_DIR` to the directory that
contains the two official image files):

```bash
RV_TESTSUITE_IMG="${OFFICIAL_IMAGE_DIR}/sdcard-rv.img" \
  python3 -B -E -s test/run_suite.py --profile official --arch rv
```

Evidence: `test/output/20260714T190411Z-official-rv-3139624/summary.json`.

- Runner commit `07fce567`; duration 3553.465490 s.
- Planned/executed/completed: 1/1/1; child return code 0; suite exit 2;
  final status `INFRA_ERROR`.
- Strict reason: captured stdout contains unsupported `U+0000`.
- The 2,382,310-byte raw stdout contains exactly one NUL byte at offset 3958,
  inside the OpenSBI ASCII banner. It also contains terminal escape output; the
  NUL alone is sufficient to make the capture malformed.
- Raw lifecycle inventory nevertheless shows 24/24 group START/END markers,
  LTP 2000/2000 START/END records, BusyBox 55 musl + 55 glibc records,
  libctest 217/217 musl + 217/217 glibc records, and one explicit QEMU
  `Shutting down...` message.
- This inventory proves the requested path ran to guest shutdown; it does not
  override the malformed-output result and is not official PASS.

## Earlier pre-hardening official LoongArch64 blocker evidence

Portable reproduction command using the same `OFFICIAL_IMAGE_DIR` setting:

```bash
LA_TESTSUITE_IMG="${OFFICIAL_IMAGE_DIR}/sdcard-la.img" \
  python3 -B -E -s test/run_suite.py --profile official --arch la
```

Evidence: `test/output/20260714T233759Z-official-la-3209556/summary.json`.

- Runner commit `07fce567`; duration 4661.846958 s.
- Planned/executed/completed: 1/1/1; child return code 0; suite exit 2;
  final status `INFRA_ERROR`.
- Strict reason: captured stdout contains unsupported `U+001B`.
- The 2,426,467-byte raw stdout has no NUL byte. Raw inspection identifies two
  `ESC[H ESC[J` clear-screen sequences immediately before the musl/glibc BusyBox
  `clear` success records; non-styling cursor/screen manipulation is deliberately
  outside the accepted output normalization grammar.
- Raw lifecycle inventory shows 24/24 group START/END markers, LTP 2000/2000
  START/END records, BusyBox 55 musl + 55 glibc records, libctest 217/217 musl
  + 217/217 glibc records, and one explicit QEMU `Shutting down...` message.
- The run reached guest shutdown without timeout or crash, but malformed output
  remains an infrastructure error and is not official PASS.

## Independent official blocker in the tracked identity plan

The reviewed RV/LA source snapshot contains 55 BusyBox rows but only 54 unique
identities: `echo "bbbbbbb" >> test.txt` occurs at two distinct rows. The strict
validator rejects duplicate identities for both libc variants. This is
independent of the raw-control-character blockers above and is covered by the
106-method official validator suite. The external testcase plan/image must be
corrected and consciously re-snapshotted; deleting the duplicate check or
inventing an identity would be a false PASS.

## Independent review findings and fixes

Read-only reviews were performed throughout implementation. Findings already
fixed and revalidated include:

1. A raw low-level wrapper exit 0 could be mistaken for official success.
   The public entry now always reaches the manifest runner and strict parser.
2. Inherited `BASH_ENV`, shell functions, Make/Rust flags, and ambient kernel
   identity/configuration variables could alter the selected executor/build.
   The official child environment now removes them and records resolved host
   executable paths.
3. The root wrapper directly owned Python/profile framing, violated the literal
   canonical `/test` delegation rule, and lost zero-argument RV compatibility.
   It now only execs `test/run_official_suite.sh`; focused startup-hook and argv
   tests cover explicit LA and default RV.
4. Manifest empty values silently erased the legacy blacklist composition
   interface. The runner now preserves caller inputs and caller-relative file
   paths, while any blacklist/skip scouting configuration is permanently
   ineligible for official PASS. Explicit guest FAIL remains FAIL.
5. Documentation omitted runner/result-tool naming conventions and described an
   outdated call chain. The README, migration map, and module diagram now
   describe the actual chain and trust boundary; the project document records
   canonical test ownership and current suite counts.
6. The selected Python/Make/Cargo/QEMU installations and their transitive
   toolchains cannot be attested by a text parser. They are documented as a
   trusted-host boundary; a process exit 0 without a fresh complete
   `summary.json` is not sufficient PASS evidence.
7. The first clean post-hardening quick run showed that the unittest harness
   wrote its identity binding immediately after verbosity dots, so the strict
   complete-line parser correctly found zero records. The producer now ends the
   progress line first. Glued, duplicated, forged, and true harness-to-runner
   cases lock the boundary in
   `test/run_unittest_suite.py::execute_bound_suite` and
   `test/unit/test_suite_runner.py::SuiteRunnerTest.test_unittest_exact_count_and_plain_ok_pass`.
8. The signal fixture originally killed the harness while its two stdout/stderr
   relay children were still alive, leaving adopted zombies for the outer
   canonical runner. The fixture now runs that abnormal harness through a
   nested real runner that owns and reaps both relays. It records the two child
   PIDs, proves both disappear, asserts aggregate 1/1/1 with 0 PASS and 1 INFRA,
   preserves `return_code == -SIGTERM`, and does not relax the outer survivor
   rule. The regression is in
   `test/unit/test_suite_runner.py::SuiteRunnerTest.test_canonical_unittest_expected_count_cannot_be_lowered`;
   the strict outer ownership checks remain in `test/run_suite.py::_case_related_pids`
   and `test/run_suite.py::run_case`.

The final read-only review found no high- or medium-risk issue after those fixes.
Its only low-risk diagnostic observation is that the simultaneous signal-plus-
survivor record preserves `return_code=-15` but leaves the optional `signal`
field unset while its human-readable result emphasizes descendants. This cannot
produce PASS, the negative return code is asserted, and the canonical outer
133-method case plus both clean quick runs passed. No production or result
classification change was made for that non-blocking presentation issue.

## External and document-production blockers

- The full external Docker evaluator is unavailable because the `docker`
  command is missing.
- No evaluator `kernel.zip` or official compressed image artifact is present in
  the worktree. The locally available raw RV image was run to completion. The
  raw LA path was interrupted and then stopped at the operator's request as
  documented above; neither incomplete attempt is reported as PASS.
- No network fetch is attempted to manufacture the missing environment.
- The checked-in project PDF is treated as a production snapshot and was not
  silently regenerated. This host lacks `google-chrome`/Chromium, `pdftotext`,
  Python `bs4`, and Python `markdown_it`, so the Markdown-to-PDF production path
  cannot be rendered and independently text-verified here. The current Markdown
  is linked separately from the PDF snapshot in the root README.

## Scope confirmation

- No kernel, syscall, libc, scheduler, filesystem, networking, or other
  production behavior was changed to obtain a test result.
- Production-adjacent edits are limited to test targets/path references in the
  Makefile and build helper shims, the root compatibility wrapper, documentation,
  and `.gitignore`/test infrastructure permitted by the task.
- The moved LoongArch evaluation configuration is byte-identical to its baseline
  source (`SHA-256 0a8799e0ba0b1dd72a8a9406a70599945cfc692cfa1cf17ce29c980f42b8f81b`).
- GitHub Actions are untouched.
- Linux differential testing is not implemented.
- No network-fetched dependency is added.
- Generated logs, reports, overlays, and official images are not tracked.
- Operator-owned `AGENTS.md`, `CODEX_TEST_SUITE_GOAL.md`, and `.codex/` are not
  edited or committed.

## Final static and scope audit

After drafting the current evidence report:

- `bash -n run-eval.sh test/run_official_suite.sh test/evaluation/run_official_evaluation.sh`
  exited 0;
- `test/checks/check_test_asset_integrity.py` reported PASS with 0 findings;
- the exact identity harness ran all 36 methods in
  `test/unit/test_test_asset_integrity.py`: 36 PASS;
- `test/run_suite.py --list` exited 0 and validated 46 registered cases;
- `git diff --check` passed;
- `git ls-files` found no legacy `scripts/check_g*`, `scripts/test_g*`, legacy
  selfcheck unit, legacy LTP-summary unit, or legacy evaluation-reporter
  implementation;
- the historical-sequence-ID scan found no forbidden token under canonical
  `/test` outside the audit-only migration map;
- no success-masking shell or Python execution construct exists in canonical
  executable test sources;
- diffs for operator controls and `.github/` are empty;
- the only tracked path under `test/output/` is its two-line `.gitignore`;
- no repository-local `__pycache__` directory exists.

Before the report commit, `git status --short` contains only the two final
documentation changes. The exact clean post-commit status and report commit hash
are recorded in the final handoff; embedding a commit's own hash in its contents
is impossible.
