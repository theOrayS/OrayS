# fix/libctest evaluator repair plan - 2026-05-20

## Goal and constraints

Target branch: `refactor/moss_kernel_like`.
Reference branch: `fix/libctest` was used read-only as the behavior reference; no files were changed on that branch.

Success criteria:

- `./run-eval.sh la` completes instead of hanging or crashing.
- `./run-eval.sh` completes instead of hanging or crashing.
- The evaluator harness reports real program results and real timeout statuses; it does not introduce fake pass results, unimplemented stubs, or hard-coded success.
- The `refactor/moss_kernel_like` file/module structure is preserved.

## Root cause summary

The current branch's auto-run harness was still skipping several evaluator groups and allowed long-running or blocked test processes to monopolize the boot-time evaluator. That made the run diverge from the working `fix/libctest` evaluator flow and could leave residual user processes behind, which later stages observed as process creation failures or boot-time hangs.

The fix keeps the tests real, but bounds each risky process tree and cleans up only after the group or case has reported its actual status.

## Modified files, functions, and expected behavior

### `examples/shell/src/cmd.rs`

- Added evaluator constants near the auto-run setup:
  - `LTP_BUSYBOX_APPLETS`
  - `LTP_CORE_CASES`
  - `DEFAULT_GROUP_TIMEOUT_SECS = 60`
  - `LIBCTEST_GROUP_TIMEOUT_SECS = 120`
- Added file/staging helpers:
  - `write_text_file`
  - `prepare_libctest_dsos`
  - `copy_runtime_libs`
  - `prepare_libctest_runtest_wrapper`
  - `rewrite_libctest_run_script`
  - `parse_libctest_command`
  - `rewrite_libctest_command`
  - `prepare_ltp_helper_bin`
  - `file_has_shebang`
- Updated `prepare_suite_stage_dir` to stage libctest DSOs, runtime libraries, and rewritten libctest shell wrappers into the existing temporary suite stage directory.
- Added `suite_group_priority` and updated `maybe_run_official_tests` so the boot-time evaluator runs groups in the stable reference order: libctest, basic, busybox, lua, LTP, libcbench, iperf, lmbench, netperf, cyclictest, iozone, unixbench, with musl before glibc.
- Removed the previous hard-coded skip branches for libctest, libcbench, lmbench, iozone, LTP, and unixbench.
- Added `run_ltp_suite`, which runs a bounded core LTP subset from the real LTP binaries and prints per-case pass/fail status instead of skipping the suite.
- Wrapped non-LTP groups with `run_user_program_argv_in_timeout`; libctest gets 120 seconds, other long groups get 60 seconds.
- Added cleanup calls after busybox, after each LTP case, after LTP completion, and after each regular group.

Expected behavior:

- Long or blocked evaluator programs produce visible `autorun: ... timed out after Ns` or `FAIL ...` output instead of hanging the kernel.
- The harness continues to the next suite after a timeout or failed testcase.
- LTP scratch data in ramfs is removed between cases to avoid exhausting memory frames.
- The final evaluator run reaches platform shutdown instead of getting stuck mid-suite.

### `examples/shell/src/uspace/process_lifecycle.rs`

- Added/used `MAX_LIVE_USER_THREADS = 512` for the auto-run user-task capacity gate.
- Added `cleanup_user_processes` for the auto-run feature. It requests exit-group termination for every live evaluator user process, then repeatedly yields/reaps until the live user-thread table drains or the bounded retry loop completes.

Expected behavior:

- Stress groups such as hackbench/cyclictest can create their expected high process count without hitting the former 256-thread ceiling.
- After a timed-out group, residual user processes are killed through the existing exit-group path rather than by faking success.
- Later groups can still fork/clone because previous timed-out process trees are reaped.

### `examples/shell/src/uspace/task_registry.rs`

- Added `live_user_thread_entries` behind `auto-run-tests`.

Expected behavior:

- The evaluator cleanup path can enumerate live user processes without changing normal syscall behavior.

### `examples/shell/src/uspace/mod.rs`

- Re-exported `cleanup_user_processes` for `auto-run-tests`.
- Re-exported `run_user_program_in_timeout` for `auto-run-tests` so the shell harness can use the existing timeout-capable user runner.

Expected behavior:

- The public surface of the local shell uspace module exposes the evaluator-only helpers while keeping them gated behind `auto-run-tests`.

## Validation evidence

Latest validation logs:

- `./run-eval.sh`: `.omx/ultragoal/run-eval-default-20260520-193931-cleanup512-bounded120-60.log`
- `./run-eval.sh la`: `.omx/ultragoal/run-eval-la-20260520-195521-cleanup512-bounded120-60.log`

Commands run:

```bash
rustfmt +nightly-2025-05-20 examples/shell/src/uspace/process_lifecycle.rs examples/shell/src/uspace/task_registry.rs examples/shell/src/uspace/mod.rs examples/shell/src/cmd.rs
rustfmt +nightly-2025-05-20 --check examples/shell/src/cmd.rs examples/shell/src/uspace/mod.rs examples/shell/src/uspace/process_lifecycle.rs examples/shell/src/uspace/task_registry.rs

timeout 1200 ./run-eval.sh
timeout 1200 ./run-eval.sh la
```

Observed completion signatures:

```text
RUN_EVAL_DEFAULT_STATUS=0
[... axplat_riscv64_qemu_virt::power:28] Shutting down...

RUN_EVAL_LA_STATUS=0
[... axplat_loongarch64_qemu_virt::power:23] Shutting down...
```

The logs still contain real testcase-level failures/timeouts, for example unsupported libc or syscall behavior reported by libctest/LTP/iozone/unixbench. Those are intentionally preserved as evidence rather than converted into fake passes. The repaired contract is that the evaluator boot completes and reports those real outcomes instead of hanging, crashing, or stopping the whole run.

## User-visible and ABI/POSIX behavior

- Intended user-visible behavior change: only the `auto-run-tests` evaluator harness changes. It now runs the previously skipped groups with bounded execution, reports real failures/timeouts, cleans evaluator leftovers, and completes both requested evaluator boots.
- No intended Linux/POSIX syscall ABI, errno mapping, or struct-layout change was introduced by this repair. The cleanup path uses existing process exit-group semantics and is feature-gated for evaluator auto-run.

## Remaining risks

- Some underlying testcases still report real failures (for example selected libctest/LTP cases and glibc iperf in the current logs). This patch does not fake or suppress those results; it makes the evaluator run finish while preserving failure evidence for future syscall/libc/network compatibility work.
