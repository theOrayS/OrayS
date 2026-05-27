# Worker 3 pipe02 panic diagnosis

Team: `ltp-stable413-to-stab-d9f99e59`  
Task: `11` / report-only pipe02 panic diagnosis  
Date: 2026-05-27  
Worker: `worker-3`

## Scope / guardrails

- Read-only diagnosis plus this report file only.
- Did **not** run QEMU/evaluator.
- Did **not** edit `.omx/ultragoal` or `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Raw runtime evidence is leader-owned under `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-27-phase-a/raw/`; this worker worktree did not have a local `raw/` copy when task-11 started.

## Evidence read

- Leader command: `raw/batch-002-rv-command.txt`
  - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=pipe02,pipe07,dup05,select01,select02,select03,select04,sendfile07,sendfile07_64,getpgid01 LTP_CASE_TIMEOUT_SECS=70 timeout 70m ./run-eval.sh rv`
- Parser summary: `raw/batch-002-rv-summary.txt`
  - `PASS LTP CASE: 0`, `FAIL LTP CASE: 0`, `panic/trap matches: 1`.
  - Only parsed row: `pipe02 | rv | musl | ltp-musl | UNKNOWN | ... | panic/trap=1`.
- Raw log: `raw/batch-002-rv.log`
  - Batch starts `ltp-musl` and `pipe02` at log lines 275-279.
  - Panic at log lines 284-287:
    - `panicked at /root/oskernel2026-orays/kernel/sync/axsync/src/mutex.rs:52:21`
    - `Task(7, "user:fork") tried to acquire mutex it already owns`
    - `left: 7`, `right: 7`.
- LTP source shape: upstream `testcases/kernel/syscalls/pipe/pipe02.c` (fetched read-only to `/tmp/ltp-src/pipe02.c` for inspection).

## LTP pipe02 expectation

`pipe02` is not a generic pipe-capacity test. It is a fork + SIGPIPE wait-status test:

1. Parent creates a pipe.
2. Child sets `SIGPIPE` default, closes the read end, writes a small buffer once, waits for parent checkpoint, then writes again.
3. Parent closes write end, reads the first buffer, closes read end, wakes child, then `wait()`s.
4. Expected result: child terminates by `SIGPIPE`; parent observes `WIFSIGNALED(status)` and `WTERMSIG(status) == SIGPIPE`.

That means this case crosses all of these surfaces at once: pipe endpoint peer accounting, fd table close/fork copy semantics, synchronous write-to-closed-pipe behavior, signal default action, process teardown, and wait status encoding.

## Exact panic path inferred from code

The panic is consistent with a re-entrant acquisition of the same process fd-table mutex while the child is handling `write(fd[1], ...)` to a closed pipe:

1. `sys_write()` obtains a mutable fd-table guard before dispatching the write:
   - `examples/shell/src/uspace/fd_table.rs:231-238`
   - `process.fds.lock().write(fd as i32, src, Some(file_size_limit))`
2. `FdTable::write()` keeps that guard while calling the pipe endpoint:
   - `examples/shell/src/uspace/fd_table.rs:992-1014`
   - `FdEntry::Pipe(pipe) => pipe.write(src)`
3. `PipeEndpoint::write()` detects a closed peer and synchronously raises SIGPIPE:
   - `examples/shell/src/uspace/fd_pipe.rs:252-264`
   - `if self.peer_closed() { Self::raise_sigpipe(); ... Err(EPIPE) }`
4. `raise_sigpipe()` handles the default SIGPIPE action synchronously and immediately terminates the current thread/group:
   - `examples/shell/src/uspace/fd_pipe.rs:187-206`
   - it calls `request_signal_exit_group(SIGPIPE_NUM)` and `terminate_current_thread_for_exit_group(..., 128 + SIGPIPE_NUM)` when the handler is default and the signal is unblocked.
5. `terminate_current_thread_for_exit_group()` calls process teardown immediately:
   - `examples/shell/src/uspace/process_lifecycle.rs:1147-1156`
   - `teardown_now=true` leads to `process.teardown()`.
6. `process.teardown()` calls `ProcessTeardown::run(...)`, which tries to lock the same `fds` mutex and close all descriptors:
   - `examples/shell/src/uspace/process_lifecycle.rs:414-416`
   - `examples/shell/src/uspace/process_lifecycle.rs:59-75`
   - line 72 takes `let mut fds = fds.lock();` and then `fds.close_all()`.
7. The lock implementation asserts on same-task recursive lock attempts:
   - `kernel/sync/axsync/src/mutex.rs:39-57`
   - line 52 asserts `owner_id != current_id`.
8. The log reports exactly that assertion for `Task(7, "user:fork")`, so the current child task already owned the mutex when teardown tried to lock it again.

Short form: `pipe02 child write` -> `process.fds.lock()` -> `PipeEndpoint::write()` -> default `SIGPIPE` -> synchronous `terminate_current_thread_for_exit_group()` -> `process.teardown()` -> `process.fds.lock()` again -> axsync self-acquire assertion.

## Likely subsystem / root-cause area

Primary subsystem: `examples/shell/src/uspace` Linux compatibility layer, specifically the interaction among:

- `fd_table.rs` fd-table locking around potentially blocking/terminating `PipeEndpoint::write()` calls;
- `fd_pipe.rs` synchronous default-SIGPIPE termination inside the pipe write path;
- `process_lifecycle.rs` teardown closing fd tables while the fd table can already be locked by the terminating syscall.

This is not evidence of a generic `axsync::Mutex` bug. The mutex assert is doing its job: it exposed a higher-level lock-order/re-entrancy bug.

## Promotion impact

Batch-002 produced no promotion evidence:

- Parser result has `PASS LTP CASE: 0` and `panic/trap matches: 1`.
- Only `rv:musl:pipe02` reached the matrix; the panic aborts before `pipe07`, `dup05`, `select01`-`select04`, `sendfile07`, `sendfile07_64`, or `getpgid01` can be classified.
- Therefore the non-pipe cases in the same command are **unclassified**, not failed and not clean.

Exclude from candidate/promotion batches until the pipe/SIGPIPE teardown path is fixed:

- Must exclude: `pipe02`.
- Exclude pipe-heavy or pipe-peer/blocking/SIGPIPE-adjacent scouts as a family until a narrow fix is validated: `pipe07`, `pipe15`, `pipe2_03`, `pipeio`, `shell_pipe01.sh`, and any new non-stable pipe case whose source expects forked peer close, blocking read/write, `SIGPIPE`, or wait-status propagation.
- Existing stable pipe cases should remain as regression sentinels; do not demote them from this report alone. The panic is a new non-stable candidate failure, not a stable-list edit request.

Replacement leader-run batch can keep the non-pipe cases from the aborted command after removing pipe cases:

`dup05,select01,select02,select03,select04,sendfile07,sendfile07_64,getpgid01`

That replacement batch still needs normal parser proof on RV and then LA before any promotion decision.

## Minimal safe reproduction / next debug steps

Do not run this concurrently from worker panes sharing default QEMU/sdcard state. Leader-only suggested steps:

1. Reproduce with the single case first:
   - `OSCOMP_TEST_GROUPS=ltp LTP_CASES=pipe02 LTP_CASE_TIMEOUT_SECS=70 timeout 70m ./run-eval.sh rv`
2. If reproduces, add temporary trace points around:
   - `sys_write()` before/after taking `process.fds.lock()`;
   - `FdTable::write()` before `FdEntry::Pipe(pipe) => pipe.write(src)`;
   - `PipeEndpoint::write()` before `raise_sigpipe()`;
   - `terminate_current_thread_inner()` before `process.teardown()`;
   - `ProcessTeardown::run()` before `fds.lock()`.
3. Minimal repair shape to evaluate (not implemented here): avoid synchronous process teardown while holding `process.fds`. Two likely safe directions:
   - drop the fd-table guard before invoking pipe read/write by cloning the `PipeEndpoint`/entry under lock and performing the potentially terminating operation after unlock; or
   - make default `SIGPIPE` delivery mark pending exit and return from the syscall path so normal syscall-dispatch termination happens after the fd-table guard has unwound.
4. After a fix, gate in this order:
   - single `pipe02` RV musl reproduction;
   - RV musl+glibc `pipe02`;
   - LA musl+glibc `pipe02`;
   - only then re-scout `pipe07`/other pipe-heavy cases.

## Verification performed by worker-3

- `python3 -B scripts/ltp_summary.py /root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-27-phase-a/raw/batch-002-rv.log` -> confirms `panic/trap matches: 1` and only `pipe02 | rv | musl | UNKNOWN` row.
- `rg -n "panic|self-acquire|mutex|pipe02|..." /root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-27-phase-a/raw/batch-002-rv.log ...` -> raw log contains mutex self-acquire panic in `pipe02`.
- Source inspection commands:
  - `nl -ba examples/shell/src/uspace/fd_pipe.rs | sed -n '1,380p'`
  - `nl -ba examples/shell/src/uspace/fd_table.rs | sed -n '190,330p'`
  - `nl -ba examples/shell/src/uspace/fd_table.rs | sed -n '930,1065p'`
  - `nl -ba examples/shell/src/uspace/process_lifecycle.rs | sed -n '420,480p'`
  - `nl -ba examples/shell/src/uspace/process_lifecycle.rs | sed -n '1120,1175p'`
  - `nl -ba kernel/sync/axsync/src/mutex.rs | sed -n '1,130p'`
- QEMU/evaluator: not run by worker-3 per task instruction.
