# Worker 3 fd/pipe/iovec lane report

Date: 2026-05-25
Worker: `worker-3`
Task: `task-3` / fd-pipe-iovec lane

## Scope and guardrails

- Lane scope: `writev03`, `pipe2_02`, and adjacent `readv` / `writev` / `preadv` / `pwritev` / `pipe` / `pipe2` / `dup` / `fcntl` candidates.
- ACK and task claim completed before lane work.
- Did not edit `.omx/ultragoal`.
- Did not final-edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; live static check in this worktree reports `350` unique stable cases and `writev03` absent.
- Did not run QEMU. Runtime/evaluator evidence below is from existing leader/phase artifacts only.
- No fake PASS, case-name hardcoding, SKIP/TCONF laundering, or wrapper-only promotion claim is made here.

## Executive outcome

No new syscall/fd-layer code patch was made in this lane. Static inspection plus existing phase evidence indicates:

1. `pipe2_02` was a runner/resource-helper staging blocker, not a `sys_pipe2()` semantics blocker. The current tree already contains the runner environment fix in `examples/shell/src/cmd.rs`, and existing leader artifacts show `pipe2_02` clean on RV and LA for both libcs.
2. `writev03` remains non-promotable. Current evidence is real wrapper `FAIL` with internal `TCONF=1` per libc on RV blocker runs; static `sys_writev()` inspection does not justify a narrow fd-layer patch without fresh non-TCONF failure evidence.
3. Adjacent iovec and fd candidates currently have strong stable350 evidence, but future hidden failures may still hit pipe atomicity/capacity or RLIMIT/O_APPEND edge cases; those should be fixed only with targeted evidence.

## Source inspection

| Area | Files/functions inspected | Finding |
| --- | --- | --- |
| `writev()` / `readv()` | `examples/shell/src/uspace/fd_table.rs::sys_writev`, `sys_readv` | `sys_writev()` reads each non-empty user iovec into an owned buffer before calling `FdTable::write()`, returning partial success if a later iovec faults. Zero-length entries are validated with length 0. This does not match the historical kernel bug shape where uninitialized data is written before fault-in completes. |
| `preadv()` / `pwritev()` | `fd_table.rs::sys_preadv`, `sys_pwritev`, `read_file_at_into_fd`, `write_file_at` | Positioned vector I/O advances a local offset and does not alter shared file offset. Existing stable350 RV/LA evidence is clean for `preadv01_64`, `preadv02_64`, `pwritev01_64`, and `pwritev02_64`. |
| Shared offsets / dup/fork | `FileEntry { offset: Arc<Mutex<u64>> }`, `FdEntry::duplicate_for_fork`, `dup_min_with_flags`, `file_entry_read/write/seek` | Duplicated and fork-copied `FileEntry` values share the same offset lock, so `dup*` and fork shared-offset semantics look correct for current promoted cases. |
| `O_APPEND` | `fd_table.rs::file_entry_write`, `write_file_at` | Normal `write()`/`writev()` append at EOF. Positioned writes ignore `O_APPEND`; Linux behavior can append even for `pwrite()`, but existing `pwritev*` evidence is clean, so no speculative patch was made. |
| `pipe2(O_CLOEXEC)` | `examples/shell/src/uspace/fd_pipe.rs::sys_pipe2`, `fd_table.rs::close_cloexec`, `process_lifecycle.rs::sys_execve` | `sys_pipe2()` applies `FD_CLOEXEC` to both ends, and `execve` calls `close_cloexec()`. The old `pipe2_02` failure shape points to helper resource execution, not pipe fd inheritance. |
| Pipe read/write/blocking | `fd_pipe.rs::PipeEndpoint::read`, `write`, `poll`, `available_read` | Blocking/yield and `SIGPIPE`/`EPIPE` paths are implemented. One future risk remains: small writes `<= PIPE_BUF` can currently partially write if the ring has less free space than the write size; POSIX pipe atomicity may require waiting or `EAGAIN` instead. |
| Pipe capacity / `FIONREAD` | `fd_pipe.rs::PIPE_BUF_SIZE`, `capacity`, `fd_table.rs::sys_ioctl(FIONREAD)`, `fcntl(F_GETPIPE_SZ/F_SETPIPE_SZ)` | `FIONREAD` for pipes returns current readable bytes. Capacity is fixed at 4096 and equal to `PIPE_BUF`; future capacity-sensitive tests may need separate `PIPE_BUF=4096` and larger `PIPE_CAPACITY`. |
| RLIMIT file writes | `fd_table.rs::limit_file_write_len`, `file_entry_write` | Possible future risk: RLIMIT truncation uses the underlying file seek position, while regular writes use `FileEntry.offset` or EOF for append. No current lane evidence ties this to a failing candidate. |

## Case decisions

| Case/family | Evidence | Decision |
| --- | --- | --- |
| `pipe2_02` | Upstream test checks that a `pipe2(O_CLOEXEC)` fd is closed across `exec` of helper `pipe2_02_child`. Existing current tree stages helper-resource cases through per-case run dirs and `PATH=helper_dir:.:target_dir:...`. Existing summaries show clean RV and LA: `followup-rv-pipe2_02-binsh-001-summary.txt`, `followup-la-pipe2_02-binsh-001-summary.txt`, and stable350 final summaries all report PASS with no internal TFAIL/TBROK/TCONF. | Already safe as stable evidence; no new code patch needed in this lane. |
| `writev03` | Upstream test is a multi-thread/page-fault-in stress requiring `.min_cpus = 2`. Existing blocker evidence shows wrapper `FAIL` code `32` and internal `TCONF=1` for both RV libcs, not invalid data (`TFAIL`). | Not promotable. Do not convert TCONF to clean. Next action is leader-serialized SMP/min-cpu policy validation; only inspect mmap/writev data path if fresh evidence becomes `TFAIL: invalid data`. |
| `readv01`, `readv02`, `writev01`, `writev02`, `writev05`, `writev06`, `writev07` | stable350 RV and LA final summaries show PASS for both libcs with zero internal failure counters. | Keep as promoted/stable. |
| `preadv01_64`, `preadv02_64`, `pwritev01_64`, `pwritev02_64` | stable350 RV and LA final summaries show PASS for both libcs with zero internal failure counters. | Keep as promoted/stable. |
| `pipe` / `pipe2` adjacent cases | stable350 evidence includes `pipe2_02` clean; previous phase reports warned about capacity/atomicity risks. | No broad patch. Patch pipe atomicity/capacity only if a targeted case exposes it. |
| `dup` / `fcntl` adjacent cases | Shared offset, cloexec, and stable evidence look acceptable for the currently promoted set. | No code change recommended. |

## Evidence used

Existing local artifacts parsed/read in this lane:

- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/blocker-batch-rv-summary.txt`: `writev03` RV glibc+musl wrapper `FAIL`, internal `TCONF=1`; `pipe2_02` older RV blocker `TBROK=1` before runner/helper repair.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-rv-pipe2_02-binsh-001-summary.txt`: `pipe2_02` RV glibc+musl PASS, zero TFAIL/TBROK/TCONF/timeout/ENOSYS/panic.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/followup-la-pipe2_02-binsh-001-summary.txt`: `pipe2_02` LA glibc+musl PASS, zero TFAIL/TBROK/TCONF/timeout/ENOSYS/panic.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-rv-final-002-summary.txt`: stable350 RV final includes clean `pipe2_02`, `readv*`, `writev01/02/05/06/07`, `preadv*`, `pwritev*` rows.
- `docs/ltp-score-improvement-2026-05-25-phase-a/raw/stable350-la-final-002-summary.txt`: stable350 LA final includes the same clean rows.
- Upstream LTP source references for expected semantics:
  - `writev03`: https://raw.githubusercontent.com/linux-test-project/ltp/master/testcases/kernel/syscalls/writev/writev03.c
  - `pipe2_02`: https://raw.githubusercontent.com/linux-test-project/ltp/master/testcases/kernel/syscalls/pipe2/pipe2_02.c

## Verification performed in this worker worktree

| Check | Result |
| --- | --- |
| `omx team api send-message ... ACK` | PASS after retrying stale/dead dispatch lock owner with `OMX_DISPATCH_LOCK_TIMEOUT_MS=120000`. |
| `omx team api claim-task ... task_id=3` | PASS; task moved to `in_progress` for `worker-3`. |
| `omx team api mailbox-list ... worker-3` | PASS; no pending worker messages. |
| `git status --short` before work | PASS; clean. |
| `omx explore --prompt ...` | Timed out after 120s; fell back to direct `rg`/`sed` source inspection. |
| Static symbol/source inspection with `rg` and `sed` | PASS; mapped fd/iovec/pipe/fcntl implementation files and evidence artifacts. |
| Sidecar code-review probe | PASS; returned read-only findings, no edits/QEMU. |
| `git diff --check` | PASS before report write. |
| `cargo fmt --all -- --check` | BLOCKED by worktree workspace metadata: `vendor/rust-fatfs` believes workspace root is `/root/oskernel2026-orays/Cargo.toml`. |
| `rustfmt --edition 2021 --check examples/shell/src/cmd.rs` | PASS. |
| `rustfmt --edition 2021 --check examples/shell/src/uspace/fd_table.rs examples/shell/src/uspace/fd_pipe.rs` | Pre-existing formatting drift detected in import ordering only; not modified by this lane. |
| `cargo check --manifest-path examples/shell/Cargo.toml --target riscv64gc-unknown-none-elf --features 'uspace auto-run-tests'` | BLOCKED in worker worktree by missing `axplat_riscv64_qemu_virt` crate (`E0463` from `kernel/arch/axhal/src/lib.rs`). |
| QEMU/evaluator run | Not run by instruction. |

## Recommended next steps for leader

1. Treat `pipe2_02` as already covered by existing leader evidence; no worker QEMU proof was generated here.
2. Keep `writev03` out of `LTP_STABLE_CASES` until there is transparent SMP/min-cpu clean evidence. If a future run reports `TFAIL` rather than `TCONF`, inspect mmap-backed `MAP_SHARED` read fault behavior before changing `sys_writev()`.
3. If future pipe candidates fail on atomicity/capacity, patch `PipeEndpoint::write()` to preserve `PIPE_BUF` atomicity for writes `<= 4096` and split capacity from `PIPE_BUF` rather than changing existing `pipe2(O_CLOEXEC)` logic.
4. If future pwrite/O_APPEND/RLIMIT candidates fail, patch `limit_file_write_len()` / `write_file_at()` against effective write offset semantics under targeted evidence.
