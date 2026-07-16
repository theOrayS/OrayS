# OrayS local test suite

This directory is the canonical home of OrayS-specific test infrastructure. It
contains the semantic regression checks, their unit fixtures, the explicit local
suite manifest, official RV/LA evaluation drivers, result validators, and audit
documentation. Production crate tests remain beside their crates and are reached
through the baseline profile; build and development helpers remain under
`scripts/`.

The suite is deliberately fail-closed. A case passes only after its process exits
zero and its declared result contract observes a complete, explicit success. A
missing dependency, missing image, skipped or empty suite, malformed lifecycle,
timeout, crash, signal exit, unknown result, or planned/executed mismatch is never
reported as PASS.

## Prerequisites

All quick-profile tooling is local and uses the Python standard library. The
current runner requires Python 3.10 or newer. The broader profiles additionally
use the repository's pinned Rust toolchain and existing host tools:

- `git`, `python3`, and Bash for metadata, checks, units, and wrappers;
- a Linux host with `prctl(PR_SET_CHILD_SUBREAPER)` support and a readable,
  complete `/proc` process table for executing cases, tracking descendants, and
  reliably reaping timed-out descendants (`--list` does not launch a case);
- `cargo` and `make` for the baseline build/test cases, plus `clang` for the
  LoongArch64 clippy case; that `clang` must accept an empty C translation unit
  for target `loongarch64-unknown-none`;
- `qemu-img` plus `qemu-system-riscv64` or
  `qemu-system-loongarch64` for the selected official profile;
- a readable local official image for the selected architecture.

The runner is offline by default. It does not download an image, dependency, or
replacement toolchain.

## Layout

```text
test/
├── README.md
├── run_official_suite.sh       # strict canonical public-profile launcher
├── run_suite.py                  # manifest validator and local runner
├── run_unittest_suite.py         # exact AST-to-runtime unittest identity binder
├── suite_manifest.json           # explicit profiles, case IDs, argv, and contracts
├── checks/                       # semantic static regression checks
├── unit/                         # positive/negative check, parser, and runner fixtures
├── fixtures/                     # checked-in static test contracts only
├── evaluation/
│   ├── config/                   # evaluation-specific platform configuration
│   ├── official_case_plan.json   # tracked BusyBox/libctest identity snapshot
│   ├── run_official_evaluation.sh
│   ├── parse_official_results.py
│   ├── summarize_ltp_results.py
│   └── report_evaluation_failures.py
├── docs/
│   ├── migration_map.md
│   └── baseline_validation.md
└── output/                       # ignored generated logs, reports, and run artifacts
```

The retained repository-root `run-eval.sh` is the strict public compatibility
entry. It only `exec`s `test/run_official_suite.sh` with the caller's arguments
and environment. The canonical launcher owns the hardened Python invocation and
official-profile framing, so `./run-eval.sh rv` is equivalent to
`python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile official --arch rv`; the LA form
behaves the same way. A zero-argument `./run-eval.sh` retains the legacy RV
default. The public launcher accepts only the architecture, one optional
`--output-dir PATH`, and optional `--fail-fast`; it rejects `--manifest`,
`--profile`, repeated `--arch`, `--list`, duplicate supported options, and all
other policy-changing arguments before Python starts. The manifest runner
performs preflight, the suite-level timeout,
process-group control, separate stdout/stderr capture, exact result validation,
and final status accounting.

`test/evaluation/run_official_evaluation.sh` is the canonical low-level adapter
invoked by the manifest case. Its zero exit status means only that the underlying
`make`/QEMU process returned zero. It is not an official PASS until
`test/run_suite.py` validates the complete captured protocol.

## Naming and registration

- Python and shell files use lowercase snake case.
- Static guards are named `check_<behavior>.py`.
- Guard/parser/runner units are named `test_<behavior>.py`.
- Executable runners are named `run_<purpose>.py` or `run_<purpose>.sh`.
- Result tools use a `parse_`, `summarize_`, or `report_` purpose prefix.
- Manifest IDs are stable semantic dotted IDs such as
  `check.user_memory_copy_boundaries`.
- Historical campaign sequence identifiers are prohibited in canonical paths,
  IDs, and ordinary output. They exist only in the migration audit document.
- Registration is explicit. Files discovered under `checks/` and `unit/` must
  exactly match the corresponding manifest profiles; the structural integrity
  check rejects omissions and stale registrations.

## Profiles and commands

The relative commands below assume the repository root. The runner itself derives
the repository root from its own file location, so from any other current working
directory invoke it through an absolute script path, for example:

```bash
cd /tmp
python3 -I -S -B -X pycache_prefix=/dev/null /absolute/path/to/worktree/test/run_suite.py --profile quick
```

| Profile | Contents | Architecture argument |
| --- | --- | --- |
| `checks` | All 19 registered static/structural guards | none |
| `unit` | All 26 Python unit scripts with 690 exact-counted methods | none |
| `quick` | `checks` followed by `unit` (45 planned cases) | none |
| `evidence-host` | One fail-closed PR3 host evidence shard | none |
| `evidence-runtime` | One fixed build plus repository-contained ABI smoke shard | required: `rv` or `la` |
| `evidence-aggregate` | Strictly merge host/RV/LA shards and render reports | none |
| `evidence-required` | Host, RV, LA, then aggregate (4 planned cases) | none |
| `baseline` | `quick`, `evidence-required`, format, workspace units, three clippy cases, RV/LA kernel builds, and `make all` (57 planned cases) | none |
| `official` | Exactly one canonical local official evaluation | required: `rv` or `la` |
| `full` | `baseline` followed by the requested official path(s) | `rv`, `la`, or `all`; default is `all` |

```bash
python3 test/run_suite.py --list
python3 test/run_suite.py --profile checks
python3 test/run_suite.py --profile unit
python3 test/run_suite.py --profile quick
python3 test/run_suite.py --profile evidence-host
python3 test/run_suite.py --profile evidence-runtime --arch rv
python3 test/run_suite.py --profile evidence-runtime --arch la
python3 test/run_suite.py --profile evidence-aggregate
python3 test/run_suite.py --profile evidence-required
python3 test/run_suite.py --profile baseline
python3 test/run_suite.py --profile official --arch rv
python3 test/run_suite.py --profile official --arch la
python3 test/run_suite.py --profile full --arch all
```

Those exact goal-form commands remain supported as convenience compatibility
entries, and the runner immediately re-executes itself in isolated mode. They
cannot undo code that an untrusted Python startup hook executed before the script
began. For commit-attributed evidence, isolate the first interpreter process:
`python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py ...`. `-I`
removes caller path and user-site injection, `-S` disables system site startup,
and the cache prefix prevents ignored repository bytecode from being loaded.
The Make targets and official launcher use this form. In either form, a shell
status without a newly created `summary.json` whose
`planned`, `executed`, and `completed` counts agree is incomplete evidence, not
a suite PASS.

Except for read-only `--list`, every canonical `checks`, `unit`, `quick`,
`baseline`, `official`, or `full` run requires a clean Git worktree. Before any
repository-local parser is loaded, the runner checks tracked and untracked state
with a closed Git environment. A dirty-tree refusal exits 2 before creating an
output directory or `summary.json`; commit-attributed evidence must therefore
start clean and remain at the same clean `HEAD` through suite completion.

`--fail-fast` is opt-in. When used, every remaining planned case is recorded as
`NOT_RUN`; this keeps the plan count visible and the suite non-passing. Use
`--output-dir /new/path` to choose an exact new evidence directory. An existing
directory is rejected so stale and current evidence cannot be mixed.

Convenience Make targets delegate to the same canonical runner:

```bash
make test-list
make test-checks
make test-unit
make test-quick
make test-baseline
```

## Official RV/LA evaluation

Set an architecture-specific image directly:

```bash
RV_TESTSUITE_IMG=/path/to/sdcard-rv.img \
  python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile official --arch rv

LA_TESTSUITE_IMG=/path/to/sdcard-la.img \
  python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --profile official --arch la
```

The reserved `official` and `full` profile names can execute only with the
canonical resolved `test/suite_manifest.json`. The same anti-impersonation rule
reserves `checks`, `unit`, `quick`, and `baseline`: an alternate manifest may
use only non-reserved fixture profile names. `--manifest` remains available for
integrity tests, but its output cannot masquerade as a canonical profile verdict.

Alternatively, set `ORAYS_WORKSPACE_ROOT` or `TESTSUITE_DIR` to a directory
containing `sdcard-rv.img` and `sdcard-la.img`. If no override is present, the
runner checks the repository parent directory. The architecture-specific image
variables have highest priority. Image absence is an infrastructure error and no
download is attempted.

The low-level Makefile does not quote every image/overlay expansion. To prevent
paths from becoming Make or shell syntax, the official adapter accepts only
absolute repository, image, and output paths composed of ASCII letters, digits,
`.` `_` `/` `+` `=` `,` `@` `:` `%` and `-`. A path containing whitespace,
Unicode, `$`, quotes, backticks, shell metacharacters, or control characters is
an explicit infrastructure error. Documented selector values containing `$`
are likewise rejected before Make. Choose a safe-path output directory rather
than weakening this boundary.

The official manifest explicitly requests the full ordered 24-group evaluator
plan with `OSCOMP_TEST_GROUPS=all`. With no caller blacklist or group-skip input,
this is the canonical official mode. The wrapper preserves its real `make`/QEMU
status, creates a unique qcow2 overlay in the case artifact directory, and
removes that overlay at exit. The suite captures QEMU stdout and stderr
separately and validates group and testcase lifecycles after a zero process
exit.

Legacy blacklist composition remains available for diagnosis. The low-level
adapter composes the inherited inline `LTP_BLACKLIST`, then each whitespace-
separated file named by `LTP_BLACKLIST_FILE`, then
`LTP_BLACKLIST_COMMON_FILE`, and finally the selected architecture's
`LTP_BLACKLIST_RV_FILE` or `LTP_BLACKLIST_LA_FILE`. Relative file tokens are
resolved against the directory from which the public runner was invoked before
the child changes to the repository root. The architecture string variables
`LTP_BLACKLIST_RV`, `LTP_BLACKLIST_RISCV64`, `LTP_BLACKLIST_LA`, and
`LTP_BLACKLIST_LOONGARCH64` are also preserved.

Any non-empty blacklist variable or a non-`none` `OSCOMP_SKIP_TEST_GROUPS`
marks the run as noncanonical scouting. Even if the resulting transcript is
otherwise a complete success and the blacklist matches no case, the runner
forces `INFRA_ERROR` and records only the configured variable names under
`details.noncanonical_official_environment`; it never awards official PASS.
An explicit guest FAIL, timeout, crash, malformed result, or adapter
infrastructure error retains its more specific non-PASS classification.

The canonical validator binds output to exact tracked plans:

- the 24 official groups must each appear once in the fixed manifest order;
- LTP musl and glibc must each execute exactly 1000 unique cases in the exact
  order read from tracked `user/shell/src/cmd.rs::LTP_STABLE_CASES`, with one
  complete START/RUN/RESULT/END lifecycle per identity;
- BusyBox musl and glibc must each emit exactly 55 ordered START/RESULT/END
  frames whose one-based execution ordinals and command payloads match
  `test/evaluation/official_case_plan.json`. Each frame also contains exactly
  one `testcase busybox ... success|fail` compatibility projection for the
  existing official scorer; its command and status must exactly match the
  structured terminal record. Replayed ordinals, duplicate explicit IDs,
  invented identities, missing rows or projections, reordering, mismatched or
  malformed records, incomplete frames, and any semantic failure are all
  non-passing;
- libctest musl and glibc must each execute exactly 217 unique ordered
  `(binary, case)` identities from that same tracked plan, with paired
  start/result/end records and one exact summary;
- every other official group requires exactly one explicit zero-status success
  record. A group that merely ends without failure remains incomplete.

`official_case_plan.json` is a reviewed tracked snapshot and the parser's
BusyBox/libctest identity trust anchor. Its source hashes record provenance; the
runner does not mount the selected image and recompute those hashes on every
run. If an image changes and emitted identities diverge, validation reports an
infrastructure error. Updating the snapshot requires re-extracting and comparing
the RV/LA musl/glibc source files, reviewing identity/count changes, and rerunning
the parser mutation tests. The plan must never be edited merely to make one log
pass.

BusyBox command text is evidence payload, not a unique identity. The tracked
2026-07-14 RV/LA snapshot has 55 executable ordered rows and 54 distinct command
strings because `echo "bbbbbbb" >> test.txt` is an intentional state-mutating
step at ordinals 37 and 41. Those are two valid cases. The producer derives the
ordinal from actual non-empty execution order, and the parser independently
checks ordinal, payload, plan order, frame completion, and the paired scorer
projection. The projection is compatibility output, never case identity. A
replay of the same ordinal, a duplicate/mismatched projection, or a duplicate
explicit plan ID still fails closed. Legacy text-only records outside a
structured frame remain useful for forensic replay but are structurally
noncanonical and cannot produce official PASS.

For an `official` case, a non-infrastructure nonzero child exit never bypasses
the validator. The runner first parses both captured streams: incomplete or
malformed structure remains `INFRA_ERROR`, a complete transcript with explicit
test failures remains `FAIL`, and a complete PASS transcript conflicting with a
nonzero child exit is `INFRA_ERROR`. The actual nonzero status is retained in
`details.process_exit_code`. Signal exits, timeouts, process-containment errors,
and manifest-declared infrastructure exit codes retain their stronger runner
classifications.

### LTP-only promotion review

Promotion review is a narrower forensic report, not a replacement for the
24-group official verdict. It requires both streams from each runner artifact:

```bash
python3 -I -S -B -X pycache_prefix=/dev/null \
  test/evaluation/summarize_ltp_results.py \
  --promotion-candidates \
  --promotion-arches rv,la \
  --promotion-libcs musl,glibc \
  --stderr-log test/output/<rv-run>/logs/official.riscv64.stderr.log \
  --stderr-log test/output/<la-run>/logs/official.loongarch64.stderr.log \
  --process-exit-code <rv-evaluator-exit-code> \
  --process-exit-code <la-evaluator-exit-code> \
  test/output/<rv-run>/logs/official.riscv64.stdout.log \
  test/output/<la-run>/logs/official.loongarch64.stdout.log
```

The two `--stderr-log` and two `--process-exit-code` values pair with the
positional stdout logs in order and are mandatory. The exit-code values must be
copied from the corresponding evaluator child records; any nonzero value blocks
promotion. Each promotion pair must also be two unique regular files in the
same resolved directory, with the same capture key (`<key>.stdout.log` plus
`<key>.stderr.log`, or legacy `<key>.log` plus `<key>.stderr.log`) and one exact,
matching RV/LA filename token. Swapped companions, duplicate architectures,
ambiguous names, symlink/hardlink aliases, or mismatched keys are rejected before
parsing. Both streams are decoded strictly; omitting stderr, malformed
bytes, unsupported controls, group/protocol framing errors, empty or unknown
promotion dimensions, incomplete case lifecycles, planned/executed/summary
mismatches, `TCONF`/`TBROK`/`TFAIL`/`ENOSYS`, timeout, panic/trap, skips, or any
prior failure make the affected evidence non-candidate. A candidate binds the
exact stdout path, stderr path, raw-byte SHA-256 digests, group, case, numeric zero result, and ordered
`START/RUN/RESULT/PASS/END` events across every requested architecture/libc
combination. Failures that genuinely belong to a non-LTP group do not determine
this LTP-only verdict, but malformed framing, protocol records outside a group,
or a record naming a different group remain global integrity errors.

Exit 0 from the report tool means its inputs were structurally processed; it
does not assert that the candidate list is nonempty and does not prove a full
official run passed. Inspect `candidate_count`, every combo binding, and the
input validation records.

The single-input `--strict` mode likewise requires one `--stderr-log`; only the
loose, non-verdict forensic summary may read stdout without a companion.

### Failure report generation

The failure reporter also requires paired stdout/stderr evidence. Repeated
`--stderr-log` arguments pair by position with the stdout logs:

```bash
python3 -I -S -B -X pycache_prefix=/dev/null \
  test/evaluation/report_evaluation_failures.py \
  --stderr-log test/output/<rv-run>/logs/official.riscv64.stderr.log \
  --stderr-log test/output/<la-run>/logs/official.loongarch64.stderr.log \
  --process-exit-code <rv-evaluator-exit-code> \
  --process-exit-code <la-evaluator-exit-code> \
  test/output/<rv-run>/logs/official.riscv64.stdout.log \
  test/output/<la-run>/logs/official.loongarch64.stdout.log \
  --output test/output/evaluation-failures.md
```

Both streams must exist and decode as strict UTF-8. The reporter reuses the
canonical LTP lifecycle validator, so an empty log, missing group end, missing
manifest or summary, incomplete/duplicate case lifecycle, stderr failure signal,
or unknown state is displayed as `ERROR`/`FAIL`; it cannot appear as an empty
successful report. Reporter exit 0 means only that the Markdown file was written.
The report's strict status and findings remain the evaluation evidence.

The lower-level `parse_official_results.py` CLI likewise requires `--stdout`,
`--stderr`, and `--process-exit-code`; stdout-only input or an unknown child
status is never eligible for a strict official verdict. The manifest runner
imports the same parser implementation directly and supplies both captured
streams and the actual child return code.

For the standalone result CLIs, `--process-exit-code` is an operator-supplied
capture fact; the CLI can reject a declared nonzero code but cannot independently
discover whether somebody typed the wrong value after moving log files. Use the
manifest runner's `summary.json` and its exact stdout/stderr paths for
commit-attributed official evidence: that workflow records the return code from
the child process itself. Standalone promotion and failure reports remain
forensic derivatives of those retained runner artifacts, not substitutes for
the runner record.

The unchanged production baseline currently emits explicit generic-group
failures but does not emit an explicit generic-group success record. Consequently,
a generic group that merely ends without failure remains an integrity error; the
validator will not infer PASS from silence. This is a documented production output
contract blocker, not a reason to relax the test driver.

The root compatibility wrapper passes caller arguments and environment through
unchanged to the canonical `/test` launcher. That launcher validates the narrow
public argument contract above before constructing the fixed canonical runner
invocation. Both shell entrypoints use a privileged-mode Bash shebang that
ignores inherited shell startup hooks. The launcher starts Python with
`-I -S -B -X pycache_prefix=/dev/null`, so caller-controlled `PYTHON*`, user and
system site startup configuration, and ignored repository bytecode cannot run
before the canonical runner.

Every case child starts from a closed environment, not a copy of the caller
environment. The only generally inherited values are `PATH` and `HOME`; both
must be literal safe paths, `PATH` must contain only non-empty absolute entries,
and neither may contain `$` or control/newline syntax that GNU Make could expand.
The runner fixes `PWD`, `LC_ALL=C`, `LANG=C`, `CARGO_NET_OFFLINE=true`,
`PYTHONNOUSERSITE=1`, `PYTHONDONTWRITEBYTECODE=1`,
`PYTHONPYCACHEPREFIX=/dev/null`, and `CARGO_HOME=<repo>/cargo-home`, then adds
only the manifest-owned case environment. The runner re-locks the offline,
no-user-site, and bytecode values after that merge, so a fixture manifest cannot
override them. Official cases additionally admit the
documented image-directory/image, blacklist, and skip scouting inputs; canonical
group, case-list, timeout, and output selectors still come from the locked
manifest. Shell functions, startup hooks, Make controls, variable-based
Cargo/Rust tool overrides, submission `REMOTE_*` knobs, and arbitrary future
Make `?=` variables are therefore not inherited by a canonical case.

The low-level official adapter starts Make with an explicit empty environment
and adds back only checked host path/home/offline/bytecode values plus documented
LTP/official selectors. It records the resolved absolute path of every required
host command in `summary.json`. Executable selection through the validated
`PATH`, the selected Python/Make/Cargo/QEMU binaries, `$HOME`-backed Rustup state,
and their transitive toolchain remain explicit trusted-host boundaries; the
runner records paths but does not perform binary attestation.

Before the LoongArch64 clippy baseline case starts, the runner performs a fixed
five-second `clang --target=loongarch64-unknown-none -x c -fsyntax-only -`
capability probe. A missing command, rejected target, signal, timeout, or process
tracking failure is `INFRA_ERROR` and the main clippy command is not executed.
The probe establishes only that the resolved CLI frontend accepts that target;
the corresponding bindgen/libclang loading remains a trusted-host boundary. If
the probe succeeds but the main command later exposes a CLI/libclang mismatch,
the main nonzero exit is `FAIL`, never reclassified as an infrastructure PASS.

## Exit and status contract

Runner process exit codes are:

| Code | Meaning |
| --- | --- |
| `0` | Every planned case executed and explicitly passed |
| `1` | At least one case explicitly failed, timed out, crashed, or was not run |
| `2` | Manifest, dependency, environment, parser, or other infrastructure error |

Per-case JSON statuses are `PASS`, `FAIL`, `TIMEOUT`, `CRASH`, `INFRA_ERROR`, and
`NOT_RUN`. Unknown statuses make the suite an infrastructure error. `SKIP`,
`XFAIL`, `TCONF`, `TBROK`, `TFAIL`, `ENOSYS`, timeout/hang evidence, panic/trap
evidence, a signal exit, duplicate result, incomplete output, and zero executed
tests cannot be PASS.

## Evidence and JSON summary

Each run creates a unique ignored directory under `test/output/` unless an exact
new output path is provided. It contains:

```text
<run>/
├── summary.json
├── logs/<semantic-id>.stdout.log
├── logs/<semantic-id>.stderr.log
└── artifacts/<semantic-id>/...
```

`summary.json` records the manifest and schema, selected profile/architecture,
baseline commit, starting and final runner commits/statuses, whether the runner
was dirty, the final provenance-stability verdict, exact Python isolation
runtime, invocation, suite start/end/duration, planned, executed, and completed
totals, final result and exit code, plus an ordered case array. Every case
records its argv, cwd, timeout, result contract, start/end,
duration, execution flag, return code or signal, status, result explanation,
separate log paths, and parser details. The file is replaced atomically after
each completed case and at suite completion.

Generated logs, qcow overlays, and large official artifacts are ignored. Do not
commit them. The tracked `test/output/.gitignore` is the only file retained in
that directory.

The exact unchanged-baseline failures and final local evidence are recorded in
[`docs/baseline_validation.md`](docs/baseline_validation.md); a nonzero quick or
baseline result documented there must not be read as suite PASS.

## Troubleshooting

- Exit 2 before output creation (for example a dirty canonical worktree): read
  the printed bootstrap reason; no `summary.json` exists by design.
- Exit 2 after output creation: read the printed preflight reason and the case's
  `result` in `summary.json`; install/provide the named local command or image
  rather than suppressing the case.
- LoongArch64 clippy `INFRA_ERROR` before the main command: inspect the recorded
  `capability_probes` entry and its separate stdout/stderr logs. Merely finding a
  `clang` executable is insufficient when it rejects
  `loongarch64-unknown-none`; replace or configure the local toolchain rather
  than bypassing the probe.
- Exit 1 from the kernel state-backed guard on the unchanged baseline: inspect
  its stdout log. The known baseline finding is missing RR skipped-task aging;
  this test-only branch intentionally does not change scheduler behavior.
- Official process exit 0 but case status `INFRA_ERROR`: inspect parser `details`
  in JSON. A missing group, missing lifecycle record, absent generic success
  proof, unsupported control character, or noncanonical scouting configuration
  is not success.
- Timeout: the runner terminates the entire child process group with SIGTERM and
  follows with SIGKILL for descendants that ignore SIGTERM. The case remains
  `TIMEOUT` and its partial stdout/stderr stay available.
- Count mismatch: update neither the assertion nor manifest blindly. Confirm the
  intended test was added or removed, then update the exact expected count and
  add a reviewable explanation.

## Adding a check or unit test

1. Add one focused `test/checks/check_<behavior>.py` guard. Preserve `--root`,
   `--json`, and nonzero-on-finding behavior where applicable. Emit exactly one
   explicit PASS status line only when there are no findings.
2. Add `test/unit/test_<behavior>.py` with a real-tree positive case and focused
   negative mutation fixtures. Every new static rule needs both positive and
   negative proof; never weaken an existing assertion to turn the baseline green.
3. Register each implementation once in `suite_manifest.json`, add it to the
   explicit `checks` or `unit` profile, and set the unit contract's exact positive
   method count. The asset-integrity guard deliberately freezes this inventory:
   update `CANONICAL_CHECK_CASE_IDS`, `CANONICAL_UNIT_CASE_IDS`,
   `CANONICAL_UNIT_EXPECTED_TESTS`, `CANONICAL_CHECK_PATHS`, and
   `CANONICAL_UNIT_PATHS` together, and extend its positive and negative fixtures
   so an omitted ID, count, path, or profile membership cannot go unnoticed.
4. Run `python3 test/run_suite.py --list`, the focused unit, the structural
   integrity guard, and then `--profile quick`.
5. Document any genuine unchanged-baseline defect with exact reproduction rather
   than adding a skip, allowlist, broad exception, or fake PASS.

Linux differential testing and GitHub Actions are explicitly outside this test
infrastructure change. Neither is implemented or modified here.
