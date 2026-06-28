# Phase 0 report: MM/mmap/futex/IPC/resource candidate lane

日期：2026-06-01
Worker：`worker-4` / Task 6
状态：report-only/source-diagnosis；未修改 `LTP_STABLE_CASES`，未运行 QEMU/evaluator。

## 0. Scope and promotion boundary

本报告只为 stable506 -> 1000 的后续 milestone 准备候选池和风险边界。它不代表 promotion proof。

必须继续遵守现有长期计划的门槛：`LTP_STABLE_CASES` 中的新增 case 只有在 RV + LA × musl + glibc 四路 wrapper PASS，且 `scripts/ltp_summary.py` 不报告新增 `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap` 时才可推广。blacklist、SKIP、status0、full-sweep 局部 TPASS、单架构/单 libc PASS 都不是计分证据。

本次 live 复核 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`：

```text
506 total / 506 unique / 0 duplicate
```

当前 stable506 中与本 lane 相关的已可信家族：

- mmap/mm：`mmap01,mmap02,mmap03,mmap06,mmap10,mmap09,mmap11,mmap001,mmap15,mmap17,mmap19`，`mprotect05`，`mincore01`，`mlock01,mlock03,mlock04`，`munlock01`，`diotest1,diotest2,diotest3,diotest5,diotest6`。
- futex/thread/process/IPC：`futex_wait02,futex_wait04,futex_wake01`，`clone01,clone03,clone06,clone07`，`vfork01,vfork02`，`waitid01,waitid02,waitid03,waitid04,waitid05,waitid06,waitid09,waitid11`，`shmdt02,shmem_2nstest,shmnstest,shmt02,shmt03,shmt06,shmt07,shmt08,shmt10`。
- resource：`getrlimit01,getrlimit02,getrlimit03`，`setrlimit01,setrlimit02,setrlimit03,setrlimit05`。

## 1. Evidence sources used

- Long-term plan baseline: `docs/ltp-1000-long-term-plan-2026-06-01/ltp-1000-long-term-plan.md` records live stable506, promotion exclusions, leader-owned stable gates, and Phase 3/4 scope.
- Previous final report: `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/final-report.md` records stable460 -> stable506, final RV/LA stable506 gate, and `read02` TCONF caveat.
- Session 1 candidate matrix: `archive/docs-pre-review-2026-06-28/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix/candidate-matrix-stable460-to-500plus.md` and `clean-candidates-not-in-stable460.txt` classify clean-not-stable and blocked cases from rv-arch002/la-arch012 full sweep. That evidence is scouting only.
- Session 5/6 reports: `session-05-mmap-mm-resource/` and `session-06-futex-process-ipc/` show which mmap/mm/futex/IPC cases were already promoted, and which blockers were deliberately left out.
- Full-sweep blacklist closure: `docs/ltp-full-sweep-blacklist-2026-05-30-arch/` records RV/LA full-sweep blocker semantics and LA-only severe blocker caveats.
- Source inspection: `examples/shell/src/uspace/{syscall_dispatch.rs,memory_map.rs,futex.rs,sysv_shm.rs,process_lifecycle.rs,resource_sched.rs,user_memory.rs}`.

## 2. Ranked candidate backlog

| Rank | Cases / family | Current evidence | Likely touchpoints | Lifetime/resource/LA risks | Required proof before promotion |
| ---: | --- | --- | --- | --- | --- |
| P0.1 | `mlockall01` | Listed clean-not-stable in Session 1 full-sweep scout; still absent from stable506. Dispatch currently returns `0` for `mlock`, `munlock`, `mlockall`, `munlockall`, `mlock2`. | `syscall_dispatch.rs` mlock-family dispatch; `resource_sched.rs` RLIMIT defaults; existing `mlock01/03/04`, `munlock01`. | Current success is semantically shallow. If LTP checks RLIMIT_MEMLOCK, page-lock accounting, or invalid flags, a no-op can produce false confidence. | Targeted RV first, then LA; include stable regressions `mlock01,mlock03,mlock04,munlock01,mincore01,getrlimit*,setrlimit*`. Parser must show no TCONF/timeout/ENOSYS/panic/trap. |
| P0.2 | `data_space`, `page01`, `page02`, `sbrk02`, `stack_space`, `ulimit01`, `mmstress_dummy`, `mmap-corruption01`, `dirty` | Session 1 clean-not-stable list contains these mm/resource/harness cases. They are not in stable506. | `brk`/mmap allocator boundaries, user stack layout, `resource_sched.rs`, page fault path, mmap shared/private range tracking. | These may be low-code or no-code promotions, but can expose allocator/free-frame drift. `ulimit01` can interact with rlimit defaults. | Run as small isolated batches, not shared QEMU artifacts. Promote only if RV/LA × musl/glibc clean and stable mm/resource regressions stay clean. |
| P0.3 | `mmap04`, `mmap05`, `munmap01`, `mmap10_1`, `mmap12`, `mmap13`, `mmap14`, `vma01`, `vma02` | Phase-B reserve queue identified these VM/mmap cases for repair after VMA behavior is understood. | `memory_map.rs::sys_mmap/sys_munmap`, `process_lifecycle.rs::record_shared_mmap/forget_mmap_range`, aspace map/protect/unmap. | MAP_FIXED, overlapping mappings, partial unmap, VMA split/merge, and shared/file-backed ranges can regress existing stable mmap/fork/thread cases. | Source-diagnose expected errno/flags first; then targeted RV singletons; only LA after RV clean. Regress stable `mmap01/02/03/06/09/10/11/001/15/17/19`, `mincore01`, `mprotect05`, `clone*`, `vfork*`. |
| P1.1 | `diotest4` | Session 1 shows 0/4 clean with `TFAIL` + `TCONF`; Session 5 explicitly did not promote it. | `user_memory.rs` user pointer validation/copy-in/out; fd read/write paths; direct I/O-like buffer validation. | Could require tightening invalid user buffer behavior. Bad fix can regress `readv/writev`, `diotest1/2/3/5/6`, and generic EFAULT semantics. | Reproduce exact failing assertions first; implement real EFAULT/nonexistent-buffer semantics only. Regression: `diotest1/2/3/5/6`, `read01/read02/read04`, `write*`, `readv*`, `writev*`. |
| P1.2 | `mprotect01` | Session 1 0/4 clean; Session 5 notes `addr=0`, read-only shared mapping, `ENOMEM/EACCES` boundary not closed. | `memory_map.rs::sys_mprotect`, `mmap_prot_to_flags`, aspace protect/error mapping, file-backed mapping flags. | Incorrect errno or permission broadening can silently make protected pages writable/readable. | Target exact errno matrix before code. Regression: `mprotect05`, all stable mmap/mincore cases, file-backed mmap cases, fork/shared mapping smoke. |
| P1.3 | `mprotect02` | Session 1 0/4 clean with `TBROK`; Session 5 notes child exits with SIGSEGV/139 and user SIGSEGV handler recovery is missing. | Page-fault path, signal delivery/trampoline, user signal frame restore, `sys_mprotect`. | High risk: signal recovery and page fault semantics touch global process behavior. Do not batch with simple mmap promotions. | Separate design/fix lane. Regression: `sigaction*`, `rt_sigaction*`, `signal*`, `sigsuspend01`, stable wait/kill/clone, mmap/mprotect. |
| P1.4 | `mincore03`, `shmat1`, `mmstress` | `mincore03` and `shmat1` are common severe blacklists; LA `mmstress` has allocator panic evidence. | `memory_map.rs::sys_mincore`, `sysv_shm.rs`, global allocator/resource telemetry. | These are resource/OOM blockers, not near-promotion cases. Removing blacklist without isolation can kill QEMU/host. | Isolated stress harness with memory caps and marker closure. Blacklist removal only after normal PASS/FAIL/TIMEOUT markers and no panic/OOM; still not stable promotion unless case passes clean. |
| P2.1 | `futex_wait03` | Session 1 0/4 clean with timeout; Session 6 explicitly not promoted. | `futex.rs::sys_futex` timeout path, scheduler wait queue, signal cancel/EINTR handling. | Timing-sensitive; false fixes can hide hangs with watchdog behavior. | Target RV singleton with parser and raw marker audit. Regression: `futex_wait02,futex_wait04,futex_wake01`, `clone*`, `vfork*`, `set_tid_address01`. |
| P2.2 | `futex_wait05` | Session 1 0/2 clean with `TFAIL` + timeout; LA blacklist documents guest hang during timer-slack loop. | `futex.rs` timeout accounting, timer slack behavior if present, LA scheduler/timer path. | LA-only hang/no-log-growth history; do not run in broad batch until single-case closure is proven. | RV first; LA isolated after RV. Require wrapper marker closure, no log-growth stall, no timeout leak. Regression: stable futex + timer cases. |
| P2.3 | `futex_wait01` | LA blacklist documents hang after partial TPASS output. | `futex.rs`, `process_lifecycle.rs` clear-child-tid futex wake, scheduler wait queue. | LA severe blocker; partial TPASS is not PASS. | Treat as blocker-removal candidate before promotion. Single LA run must close normally before any batch inclusion. |
| P2.4 | Futex requeue/advanced ops, e.g. `futex_cmp_requeue*` if targeted later | Source currently supports only `FUTEX_WAIT` and `FUTEX_WAKE`; other commands return `ENOSYS`. | `futex.rs` queue table, keying, wake/requeue semantics, robust-list integration. | Requires moving waiters across queues and preserving key lifetime. High fan-out but higher concurrency risk. | Implement only after P2.1/P2.2 timing semantics are stable. Add stress regressions and teardown regressions. |
| P2.5 | SysV shm followups beyond promoted `shmt*` subset | Existing SysV shm supports minimal `shmget/shmat/shmdt/shmctl`; Session 6 promoted a clean subset. | `sysv_shm.rs`, `process_lifecycle.rs` fork attachment inheritance, `memory_map.rs::sys_munmap`. | `IPC_RMID` keeps removed segments without refcount/free; `shmctl(IPC_STAT)` clears a fixed-size buffer rather than filling real metadata. Long runs can leak backing pages. | Add attachment refcount/lifetime model before stress cases. Regression: all stable `shm*`, fork/vfork/clone, `mincore01`, mmap/unmap. |
| P3.1 | SysV sem/msg families | `rg` found no sem/msg syscall dispatch/implementation; default dispatch returns `ENOSYS`. | New `sysv_sem.rs` / `sysv_msg.rs` style model plus dispatch and process teardown. | Larger surface: permissions, undo lists, blocking wakeups, timeouts, IPC_RMID lifetime. Low short-term ROI unless hidden-score priority changes. | Plan-first implementation with strict subset boundaries; do not add case-name shims. |
| P3.2 | `move_pages`, `madvise`, `mremap`, `process_madvise` families | No current dispatch/implementation found for these names; default path is `ENOSYS`. | `syscall_dispatch.rs`, VM/aspace APIs. | `mremap` can invalidate VMA assumptions; `move_pages` may be mostly unsupported on this kernel. | Only implement if LTP source expectations are simple and Linux-compatible errno model is clear. |

## 3. Source diagnosis: mmap/mm/resource

### 3.1 Existing mmap/mincore/mprotect shape

- `syscall_dispatch.rs` dispatches `brk`, SysV shm, `mmap`, `mincore`, `mprotect`, `msync`, and `munmap` directly, while mlock-family syscalls currently return `0` unconditionally.
- `memory_map.rs::sys_mmap` validates length/type, picks an address below the stack, maps allocated pages, optionally copies file content for non-anonymous mappings, and records writable shared mmaps. It does not implement file-backed dirty writeback.
- `sys_munmap` aligns ranges, defers self-stack unmap for thread teardown, forgets shared-mmap ranges, then unmaps the aspace range.
- `sys_msync` validates flags/alignment/mapped pages but returns success without writeback. Treat `msync*`/file-backed shared-write cases as semantic work, not no-op promotion.
- `sys_mincore` now implements the stable506-minimum model: zero length succeeds; unaligned address is `EINVAL`; overflow/unmapped/out-of-user range is `ENOMEM`; bad vec is `EFAULT`; mapped pages copy out residency byte `1`.
- `sys_mprotect` validates alignment/range and calls aspace protect; it pre-faults small writable stack-like ranges. It does not by itself provide user SIGSEGV handler recovery, which is why `mprotect02` remains high-risk.

### 3.2 User-pointer and resource boundaries

`user_memory.rs` has common validation/copy helpers. Candidate `diotest4` should be fixed through these generic semantics, not by recognizing test names. `resource_sched.rs` defaults only stack and nofile to finite values; all other rlimits default to `u64::MAX`. Therefore `ulimit01`, `mlockall01`, and future RLIMIT_MEMLOCK-related work need explicit Linux-compatible expectations before promotion.

### 3.3 VMA/lifetime risks

`process_lifecycle.rs` tracks writable shared mmap ranges and re-protects them across fork. `forget_mmap_range` removes overlapping recorded ranges on unmap. This is enough for current stable cases, but VMA split/merge and partial unmap candidates can expose missing interval precision. Any MAP_FIXED/munmap/vma candidate must include fork/shared-mmap regressions.

## 4. Source diagnosis: futex/thread/IPC

### 4.1 Futex

`futex.rs` keys futex wait queues by physical frame plus page offset, which is better than plain virtual address for forked shared mappings. The syscall currently handles only `FUTEX_WAIT` and `FUTEX_WAKE`; unsupported commands return `ENOSYS`. WAIT checks alignment/null, compares current user value, waits on a queue, handles optional relative timeout, and reports `ETIMEDOUT`/`EINTR` for timeout/watchdog/signal cases.

Implication: near-term candidates should focus on WAIT/WAKE timeout/EINTR correctness (`futex_wait03`, then maybe `futex_wait05`) before advanced requeue/PI/bitset operations. LA `futex_wait01`/`futex_wait05` are blocker-removal work first because they previously hung without normal wrapper closure.

### 4.2 Clone/thread teardown

`sys_clone` has separate fork-like and thread paths. Fork-like clones allow a limited set of CLONE flags; thread clones require `CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_SYSVSEM|CLONE_THREAD` plus selected optional flags. `clear_child_tid` writes zero and wakes the futex address on thread exit, and deferred self-unmap handles thread-stack teardown.

Implication: future clone/futex regressions must include `set_tid_address01`, stable `clone*`, `vfork*`, futex wait/wake, and signal/exit wait cases. Unsupported flag combinations returning `ENOSYS` are real blockers for some LTP rows and should not be hidden.

### 4.3 SysV shm and missing IPC families

`sysv_shm.rs` implements a minimal real shared-memory model: global segment table, page backing allocation, linear mapping into user space, shmdt via `munmap`, and minimal `shmctl` commands. Current `IPC_RMID` retires removed segments into `removed_segments` without attachment refcount/free; this prevents freeing live pages under LTP buffers but creates lifetime/leak risk. `IPC_STAT` clears a fixed-size user buffer instead of filling real `shmid_ds` metadata.

No current sem/msg syscall dispatch/implementation was found; those families likely fall to default `ENOSYS`. They should be treated as larger Phase 4 work, not easy promotion.

## 5. LA and resource caveats

- Common severe blacklists include `shmat1` and `mincore03` because prior runs exhausted host/guest memory and killed QEMU/runner. These require isolated stress runs and allocator telemetry before blacklist removal.
- LA-only blacklist includes allocator panics or hangs for `mmstress`, `dirtyc0w`, `write01`, `futex_wait01`, `futex_wait05`, and neighboring resource/timer cases. LA-only blockers must not be copied into RV/common without evidence, and must not be removed until targeted runs close with normal markers.
- The local `output_rv.md` / `output_la.md` parser summaries are only 63-case smoke logs: each shows `PASS LTP CASE 126`, `FAIL 0`, internal `TCONF 4`, timeout-string matches 10, ENOSYS 0, panic/trap 0. They are not candidate promotion evidence for this lane.

## 6. Required regression matrix

When any case from this report is moved from diagnosis to code/promotion work, run the smallest proving batch first, then expand only if clean:

1. **mmap/mm/resource singleton gate**
   - Candidate singleton(s), RV first.
   - Stable regressions: `mmap01,mmap02,mmap03,mmap06,mmap09,mmap10,mmap11,mmap001,mmap15,mmap17,mmap19,mincore01,mprotect05,mlock01,mlock03,mlock04,munlock01,diotest1,diotest2,diotest3,diotest5,diotest6`.
   - Resource/rlimit regressions when relevant: `getrlimit01,getrlimit02,getrlimit03,setrlimit01,setrlimit02,setrlimit03,setrlimit05,ulimit01` if under test.
2. **futex/thread singleton gate**
   - Candidate singleton(s), RV first.
   - Stable regressions: `futex_wait02,futex_wait04,futex_wake01,set_tid_address01,clone01,clone03,clone06,clone07,vfork01,vfork02,waitid01,waitid02,waitid03,waitid04,waitid05,waitid06,waitid09,waitid11`.
   - For timeout/EINTR changes, include timer/signal smoke: `nanosleep04,clock_nanosleep04,alarm05,alarm07,kill02,tkill01,tkill02`.
3. **SysV shm/IPC gate**
   - Stable regressions: `shmdt02,shmem_2nstest,shmnstest,shmt02,shmt03,shmt06,shmt07,shmt08,shmt10` plus fork/clone/vfork and mmap/mincore.
   - Stress rows such as `shmat1` must be isolated with explicit memory telemetry and host/QEMU safety guardrails.
4. **Promotion batch gate**
   - Only after RV targeted clean, run LA targeted for the same exact set.
   - Then run final candidate+regression batch on both arch/libc combinations and parse with `scripts/ltp_summary.py`.
   - Completion evidence must include wrapper PASS/FAIL counts, internal TFAIL/TBROK/TCONF counts, timeout/ENOSYS/panic/trap counts, raw log paths/checksums, and marker/incomplete audit if the run is broad.

Suggested command shape for a future leader-owned isolated run, not executed in this report:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='<comma-separated-candidates-and-regressions>' ./run-eval.sh rv
python3 scripts/ltp_summary.py <raw-log> > <summary.txt>
OSCOMP_TEST_GROUPS=ltp LTP_CASES='<same-list>' ./run-eval.sh la
python3 scripts/ltp_summary.py <raw-log> > <summary.txt>
```

## 7. Recommended next slices

1. **No-code/low-code scout refresh**: run isolated targeted gates for `mlockall01` plus the simple clean-not-stable mm/resource set (`data_space,page01,page02,sbrk02,stack_space,ulimit01,mmstress_dummy,mmap-corruption01,dirty`). If any are four-way clean, promote only in a leader-owned milestone batch with stable-list diff and parser evidence.
2. **mprotect/diotest fix lane**: diagnose `diotest4` and `mprotect01` before `mprotect02`; avoid mixing signal-frame work into simple mmap errno fixes.
3. **futex timeout lane**: start with `futex_wait03` on RV. Defer `futex_wait05` LA until RV evidence and single-case LA isolation are ready.
4. **SysV shm lifetime lane**: add attachment refcount/free accounting before tackling `shmat1`/stress or broader `shmctl` metadata cases.
5. **Defer big IPC and advanced VM**: sem/msg, futex requeue/PI/bitset, `mremap`, `move_pages`, and `madvise` should get plan-first treatment because they touch global lifetime/concurrency/VM invariants.

## 8. Stop condition for this worker report

Report artifact written only. No stable promotion, no blacklist edit, no source edit, no QEMU/evaluator run. The next safe action is for the leader to select one isolated candidate batch or assign a narrow code-fix lane with dedicated verification artifacts.
