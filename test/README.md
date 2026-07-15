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
- a Linux host with `prctl(PR_SET_CHILD_SUBREAPER)` support for executing cases
  and reliably reaping timed-out descendants (`--list` does not launch a case);
- `cargo` and `make` for the baseline build/test cases;
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
├── suite_manifest.json           # explicit profiles, case IDs, argv, and contracts
├── checks/                       # semantic static regression checks
├── unit/                         # positive/negative check, parser, and runner fixtures
├── fixtures/                     # checked-in static test contracts only
├── evaluation/
│   ├── config/                   # evaluation-specific platform configuration
│   ├── official_case_plan.json   # tracked BusyBox/libctest identity snapshot
│   ├── run_official_evaluation.sh
│   ├── validate_official_results.py
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
`python3 -B -E -s test/run_suite.py --profile official --arch rv`; the LA form
behaves the same way. A zero-argument `./run-eval.sh` retains the legacy RV
default. The manifest runner performs preflight, the suite-level timeout,
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
python3 -B -E -s /absolute/path/to/worktree/test/run_suite.py --profile quick
```

| Profile | Contents | Architecture argument |
| --- | --- | --- |
| `checks` | All 16 registered static/structural guards | none |
| `unit` | All 20 Python unit scripts with 488 exact-counted methods | none |
| `quick` | `checks` followed by `unit` (36 planned cases) | none |
| `baseline` | `quick`, format, workspace units, three clippy cases, RV/LA kernel builds, and `make all` | none |
| `official` | Exactly one canonical local official evaluation | required: `rv` or `la` |
| `full` | `baseline` followed by the requested official path(s) | `rv`, `la`, or `all`; default is `all` |

```bash
python3 test/run_suite.py --list
python3 test/run_suite.py --profile checks
python3 test/run_suite.py --profile unit
python3 test/run_suite.py --profile quick
python3 test/run_suite.py --profile baseline
python3 test/run_suite.py --profile official --arch rv
python3 test/run_suite.py --profile official --arch la
python3 test/run_suite.py --profile full --arch all
```

Those exact goal-form commands remain supported. For controlled evidence runs,
use `python3 -B -E -s test/run_suite.py ...`; the additional flags prevent caller
Python startup configuration from executing before the runner. A direct
`python3 test/run_suite.py ...` invocation necessarily trusts the selected
interpreter plus `PYTHONHOME`, `PYTHONPATH`, and user/system site startup. In
either form, a shell status without a newly created `summary.json` whose
`planned`, `executed`, and `completed` counts agree is incomplete evidence, not
a suite PASS.

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
  python3 test/run_suite.py --profile official --arch rv

LA_TESTSUITE_IMG=/path/to/sdcard-la.img \
  python3 test/run_suite.py --profile official --arch la
```

Alternatively, set `TESTSUITE_DIR` to a directory containing
`sdcard-rv.img` and `sdcard-la.img`. If neither override is present, the runner
checks the repository sibling `../testsuits-for-oskernel/`. Image absence is an
infrastructure error and no download is attempted.

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
- BusyBox musl and glibc must each emit exactly 55 ordered terminal records that
  match `test/evaluation/official_case_plan.json`; duplicate identities,
  invented identities, missing rows, reordering, or any failure is non-passing;
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

Known BusyBox identity blocker: the tracked 2026-07-14 RV/LA image snapshot has
55 executable rows but only 54 unique identities because
`echo "bbbbbbb" >> test.txt` appears twice. The validator intentionally rejects
the duplicate. Therefore these images cannot produce canonical official PASS
even if every command reports success. The external testcase plan/image must be
corrected and then consciously re-snapshotted; weakening duplicate-ID detection
or inventing a replacement identity would be a false result.

The unchanged production baseline currently emits explicit generic-group
failures but does not emit an explicit generic-group success record. Consequently,
a generic group that merely ends without failure remains an integrity error; the
validator will not infer PASS from silence. This is a documented production output
contract blocker, not a reason to relax the test driver.

The root compatibility wrapper passes caller arguments and environment through
unchanged to the canonical `/test` launcher. Both shell entrypoints use a
privileged-mode Bash shebang that ignores inherited shell startup hooks, and the
launcher uses `-E -s` so caller-controlled `PYTHON*` and user-site startup
configuration cannot run before the canonical runner. When constructing the
official case child environment, the runner deliberately removes inherited
shell functions, `BASH_ENV`/`ENV`, Make/Rust control
flags, and ambient variables that can change the canonical kernel app, features,
mode, platform configuration, output image, memory, or network configuration.
It records the resolved absolute path of every required host command in
`summary.json`. The selected Python, Make, Cargo, QEMU, and related executables
and their transitive toolchain remain an explicit trusted-host boundary; the
text protocol is not binary execution attestation.

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
baseline and runner commits, invocation, suite start/end/duration, planned,
executed, and completed totals, final result and exit code, plus an ordered case
array. Every case records its argv, cwd, timeout, result contract, start/end,
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

- Exit 2 before launch: read the printed preflight reason and the case's `result`
  in `summary.json`; install/provide the named local command or image rather than
  suppressing the case.
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
   method count.
4. Run `python3 test/run_suite.py --list`, the focused unit, the structural
   integrity guard, and then `--profile quick`.
5. Document any genuine unchanged-baseline defect with exact reproduction rather
   than adding a skip, allowlist, broad exception, or fake PASS.

Linux differential testing and GitHub Actions are explicitly outside this test
infrastructure change. Neither is implemented or modified here.
