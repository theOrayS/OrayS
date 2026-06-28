# Phase0 regression matrix and validation plan

日期：2026-06-01
Worker：`complete-dev-1000ltp-c632b4a0/worker-1`
任务边界：report-only；不修改源码、不修改 stable list、不运行 QEMU、不 checkpoint Ultragoal。

## 1. Baseline and evidence floor

- live stable source：`examples/shell/src/cmd.rs::LTP_STABLE_CASES`。
- 本轮复核：`506 total / 506 unique / 0 duplicates`。
- stable506 final evidence：`archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md` 与 `validation.md`。
- stable506 gate floor：RV/LA 均 `PASS LTP CASE 1012`、`FAIL 0`、`ltp-musl 506/0`、`ltp-glibc 506/0`；timeout/ENOSYS/panic/trap 为 0。
- 透明 caveat：继承的 `read02` `TCONF 4` 必须继续报告；它不是新增失败，但也不是 internal-clean。

## 2. Stable506 regression family matrix

| Family | Stable anchors / nearby cases | Source surfaces | Minimal regression subset | Why it matters |
| --- | --- | --- | --- | --- |
| access/stat/path permissions | `access01`, `stat02`, `statx*`, `readlink*`, `mknod*`, `rename*`, `fs_perms`, `readdir01` | `examples/shell/src/uspace/metadata.rs`, `runtime_paths.rs`, `synthetic_fs.rs`, `fd_table.rs`, `syscall_dispatch.rs` | current stable path/stat/access rows plus candidate lane rows before promotion | Parent search/write permission, symlink loop, `ENOTDIR/EISDIR/EFAULT/ELOOP`, sticky-bit and metadata layout changes can silently regress many LTP rows. |
| FD/fcntl/pipe/io | `pipe11`, `fcntl11/14/19/22`, `readv/writev`, `sendfile`, `fchown*`, `dup*` | `fd_table.rs`, `fd_pipe.rs`, `fd_socket.rs`, `process_lifecycle.rs` | stable FD/pipe/read/write rows plus blocked `fcntl30`, `pipe07`, `pipe15`, `writev03`, `pwritev03` scouts | Shared offsets, `O_APPEND`, close-on-exec, record locks, pipe EOF/SIGPIPE/nonblock/EINTR and fd lifetime are high-regression surfaces. |
| time/select/signal | `signal03`, `signal04`, `clock_gettime*`, `nanosleep*`, `poll/ppoll/pselect/select`, `getitimer/setitimer` | `time_abi.rs`, `select_fdset.rs`, `signal_abi.rs`, `process_lifecycle.rs`, `syscall_dispatch.rs` | stable signal/time rows plus seed rows `clock_gettime04`, `clock_nanosleep02`, `nanosleep01`, `poll02`, `pselect01`, `pselect01_64`, `settimeofday01`, `time-schedule` | Timing hacks, remaining-time writeback, EINTR, signal mask restore and readiness semantics can produce TCONF/timeout rather than obvious crashes. |
| process/wait/scheduler/resource | `fork`, `wait`, `vfork01/02`, `kill02`, `tkill01/02`, `sched_tc*`, `setrlimit*`, `waitid*`, `clone*`, `execve*` | `process_lifecycle.rs`, `process_abi.rs`, `task_registry.rs`, `resource_sched.rs`, `signal_abi.rs` | stable fork/wait/signal rows plus blocked `waitid07`, `clone02`, `clone04`, `execve01/05`, `nice04` | Reparent/zombie state, child setup, signal delivery, scheduler/priority errno and rlimit inheritance can invalidate aggregate gates. |
| mmap/mm/user memory | `mmap001`, `mmap15`, `mmap17`, `mmap19`, `mprotect05`, `mincore*`, `diotest*`, `page*` | `memory_map.rs`, `user_memory.rs`, `memory_policy.rs`, `program_loader.rs` | stable mmap/mincore rows plus `diotest4`, `mprotect01`, `mprotect02`, `data_space`, `dirty`, `mlockall01`, `page01/02`, `stack_space` | User pointer validation, VMA split/merge, page permissions, SIGSEGV recovery and allocator pressure can regress both correctness and LA resource stability. |
| futex/thread/IPC | `futex_wait02/04`, `futex_wake01`, SysV shm rows `shmt*`, `shmdt02`, `shmem_2nstest`, `shmnstest` | `futex.rs`, `sysv_shm.rs`, `task_registry.rs`, `process_lifecycle.rs`, `fd_pipe.rs` | stable futex/shm rows plus blocked `futex_wait03/05` and future sem/msg rows | Waiter cleanup, timeout/EINTR, keying, shared object lifetime and exit cleanup are easy to break with local fixes. |
| network/proc/syntheticfs/LA blockers | `accept01`, `listen01`, `socket02`, `socketpair02`, `/proc` and syntheticfs probes, LA blacklist families | `fd_socket.rs`, `synthetic_fs.rs`, `system_info.rs`, `process_abi.rs`, blacklist docs | targeted LTP-only closure first; stable regression only after ordinary closure and parser-clean PASS | Many rows are quality/blocker-reduction work, not immediate promotion; LA resource/network blockers can poison full-sweep closure. |
| runner/marker/log parser | wrapper marker lines, `scripts/ltp_summary.py`, final gate summaries | `examples/shell/src/cmd.rs`, `scripts/ltp_summary.py`, `run-eval.sh` | marker-prefix audit plus parser JSON/text summaries for every promotion/final gate | Bad prefixes, glued markers, missing `RUN_META`, truncation or wrapper-only reasoning can create false PASS/FAIL accounting. |

## 3. Minimal targeted validation ladder

Use the smallest rung that proves the claim; advance only after the previous rung is clean.

1. **Static/source preflight**
   - Recompute `LTP_STABLE_CASES` total/unique/duplicates.
   - Read relevant LTP source/runtest entries and repo source surfaces.
   - Confirm no hardcoded case/path/process/output special casing.
2. **Build hygiene for code changes**
   - For POSIX/user-space behavior: at least `make A=examples/shell ARCH=riscv64`.
   - For formatting: `cargo fmt --all -- --check` or targeted formatter when relevant.
   - Always run `git diff --check` before commit.
3. **Targeted RV first**
   - Run a small `LTP_CASES=<case-list>` or equivalent targeted batch on RV only after artifact isolation is clear.
   - Parse raw log with `scripts/ltp_summary.py`; any TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap blocks promotion.
4. **Adjacent regression subset**
   - Run stable anchors for the touched family, e.g. access/stat/pipe/signal/read/write/mmap/process/futex.
   - Include both previously stable rows and near-candidate rows that exercise the same syscall/errno/lifetime boundary.
5. **LA confirmation**
   - Repeat the targeted batch and adjacent subset on LA.
   - Record arch-specific caveats; LA-only ordinary FAIL/TBROK/TCONF is not PASS, and severe blockers stay blacklist-only until ordinary closure is proven.
6. **Four-way libc gate**
   - Confirm RV + LA x musl + glibc wrapper PASS for every newly promoted row.
   - Confirm parser sees no new internal failures, timeout, ENOSYS, panic/trap, incomplete stack or marker-prefix issue.
7. **Milestone aggregate gate**
   - After a leader-owned stable-list edit, run `LTP_CASES=stable` aggregate gates for RV and LA.
   - Preserve raw log paths, summary text/JSON, checksum or retention policy, marker-prefix audit and known caveats.
8. **Final quality checks**
   - `git diff --check`, build/typecheck appropriate to the changed scope, code-review/cleanup when assigned, and Lore commit with `Tested`/`Not-tested`.

## 4. Parser commands and expected summaries

Canonical parser commands:

```bash
python3 -B scripts/ltp_summary.py <raw-log>
python3 -B scripts/ltp_summary.py --json <raw-log> > <summary>.json
python3 -B scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc <rv-log> <la-log>
```

Marker-prefix / closure audit shape for raw logs:

```bash
rg '^RUN LTP CASE|^PASS LTP CASE|^FAIL LTP CASE|^TIMEOUT LTP CASE|^\[CONTEST\]\[LTP\]\[SKIP\]' <raw-log>
python3 - <<'PY'
from pathlib import Path
for path in ['rv.log', 'la.log']:
    bad = []
    for i, line in enumerate(Path(path).read_text(errors='ignore').splitlines(), 1):
        if 'LTP CASE' in line and not (
            line.startswith('RUN LTP CASE ')
            or line.startswith('PASS LTP CASE ')
            or line.startswith('FAIL LTP CASE ')
            or line.startswith('TIMEOUT LTP CASE ')
            or line.startswith('LTP CASE RUNTIME ')
        ):
            bad.append((i, line[:220]))
    print(path, 'non_prefix_ltp_case_lines', len(bad))
PY
```

Each report must record:

- command and environment (`OSCOMP_TEST_GROUPS`, `LTP_CASES`, `LTP_CASE_TIMEOUT_SECS`, arch, timeout wrapper);
- raw log path and whether raw log is retained or summarized only;
- summary text/JSON path;
- PASS/FAIL counts, suite summaries (`ltp-musl`, `ltp-glibc`), internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic/trap, incomplete and marker-prefix status;
- known caveats, especially inherited `read02` `TCONF`.

## 5. RV/LA x musl/glibc gates

For stable promotion or milestone gates, the target shape is:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=<targeted-list-or-stable> LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=<targeted-list-or-stable> LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
```

Acceptance:

- wrapper PASS for every targeted/promoted case in both `ltp-musl` and `ltp-glibc` suites;
- `FAIL LTP CASE` count 0 for stable aggregate gates;
- no new internal `TFAIL/TBROK/TCONF` beyond explicitly inherited caveats;
- timeout/ENOSYS/panic/trap/incomplete 0;
- marker-prefix bad lines 0;
- any arch/libc-specific downgrade is documented as no-promotion, not hidden.

## 6. QEMU artifact isolation rules

This worker did not run QEMU. Future QEMU/evaluator work must follow these isolation rules:

- Run `df -h / /root` before and after long QEMU/evaluator/full-sweep jobs.
- Do not let multiple workers share the same default QEMU/sdcard/qcow2 artifacts. If isolated artifacts are not assigned, leader runs promotion/final gates serially.
- Do not stage generated artifacts: `kernel-rv`, `kernel-la`, `sdcard-*.img`, `disk*.img`, `*.log`, `target/`, `build/`, `.axconfig.toml`, or large remote-output files unless explicitly requested.
- If a detached/team worktree lacks a local image, use only leader-approved image paths; prior full-sweep lanes sometimes required root-target images rather than worktree-local paths.
- Full-sweep/blacklist jobs must record `LTP_CASES` mode, blacklist sources, skipped counts, raw log path, parser summary, marker audit and closure status.
- If raw output is truncated, glued, lacks `RUN_META`, or contains ambiguous wrapper lines, preserve a marker audit and do not use the log for promotion.

## 7. Report-only verification for this file

- Cited current plan, archived stable506 final/validation docs, candidate/backlog docs, and workflow validation docs.
- No code changed; no QEMU/build/evaluator run.
- Subagent skip reason: task-3 delegation is optional and the report is a bounded validation-policy synthesis from local docs and already collected task-1 evidence; serial execution was sufficient.
