# Worker 3 fd/pipe/iovec lane reconciliation report

Date: 2026-05-25
Worker: `worker-2` executing task-3 / fd-pipe-iovec lane
Leader reconciliation: yes; worker implementation was auto-integrated, but task lifecycle stayed `in_progress`, so this report records the terminal evidence without promoting cases.

## Scope and guardrails

- Lane scope: `writev03`, `pipe2_02`, adjacent readv/writev/preadv/pwritev/pipe/pipe2/dup/fcntl/poll/select candidates.
- Did not edit `.omx/ultragoal`.
- Did not edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; live stable list remains 300 unique cases.
- Did not claim any stable promotion gate. Worker/local QEMU or build evidence is discovery-only unless leader serializes it.
- No fake PASS, case-name hardcoding, timeout-as-pass, or failure-to-TCONF conversion was introduced.

## Implemented change integrated on leader

Commits integrated by team auto-checkpoint:

- `48740418 omx(team): auto-checkpoint worker-2 [2]`
- `0a3337a0 omx(team): auto-checkpoint worker-2 [2]`

Code touched: `examples/shell/src/cmd.rs`.

Intent:

- Preserve LTP resource-helper semantics for cases such as `pipe2_02` that copy helper binaries/files into the current working directory and then execute them by basename.
- Add `LTPROOT=<suite>/ltp` to the LTP environment so upstream helper/library code can find its resource root.
- Keep helper binary lookup through the staged helper directory while placing `.` before the original testcase bin directory in `PATH`.
- When a testcase has resource helper files named `<case>_*`, run that case from a per-case `/tmp/ltp-work/<case>-run` directory instead of the testcase bin directory.
- Detect resource helpers with explicit `fs::metadata(join_path(target_dir, name))` so helper detection is tied to the testcase bin directory entry path.

This is a runner-environment fix, not an LTP result shim; it changes where real test binaries run from, not how PASS/FAIL markers are emitted.

## Candidate decisions

| Case/family | Evidence/diagnosis | Promotion decision |
| --- | --- | --- |
| `pipe2_02` | Worker found the blocker is upstream LTP `SAFE_CP`/resource-helper behavior: helper resources must be available from the testcase working directory. The implemented runner change is a real harness/environment fix and may unblock the case, but no leader-serialized RV+LA x musl+glibc clean evidence exists yet. | Not promotable yet. Needs fresh leader-targeted RV then LA evidence parsed by `scripts/ltp_summary.py`. |
| `writev03` | Worker diagnosis: current evidence still hits SMP=1 `TCONF` shape; promotion likely requires an explicit SMP=2 serialized validation policy or a true implementation change if the failure persists. | Not promotable. Do not treat TCONF as clean. |
| adjacent pipe/iovec/fcntl/poll/select | No clean four-way matrix was produced in this task. | Discovery only. |

## Verification recorded so far

Worker/leader lightweight checks seen in pane and integrated state:

- `rustfmt --edition 2021 examples/shell/src/cmd.rs` ran in worker lane.
- `git diff --check` ran in worker lane after the runner edit.
- Worker attempted `cargo check --manifest-path examples/shell/Cargo.toml --target riscv64gc-unknown-none-elf --features "uspace auto-run-tests"` and a scoped `make test_build` path; terminal pass/fail evidence was not captured as a promotion gate.
- Leader parsed existing blocker/batch logs separately with `scripts/ltp_summary.py`; those logs remain discovery, not promotion evidence.

Required next verification before any promotion:

1. `cargo fmt --all -- --check` from repo root.
2. `make A=examples/shell ARCH=riscv64` from repo root.
3. Leader-serialized targeted RV run for at least `pipe2_02,writev03` and nearby pipe regressions, parsed by `python3 -B scripts/ltp_summary.py`.
4. If RV is clean, run the same LA targeted gate before adding anything to stable.

## Risks and regression guards

- The runner cwd/PATH/LTPROOT change affects all LTP cases that have `<case>_*` resource helper files; it must be validated with a stable aggregate or targeted subset before promotion.
- Marker prefix behavior was not intentionally changed; any later logging edit must keep `PASS LTP CASE`/`FAIL LTP CASE` markers at column 0.
- `writev03` remains explicitly non-clean until the SMP/TCONF issue is resolved transparently.
