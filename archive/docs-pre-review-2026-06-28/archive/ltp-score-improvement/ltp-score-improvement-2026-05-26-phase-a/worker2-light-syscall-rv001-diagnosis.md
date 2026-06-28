# Worker 2 light syscall RV001 narrow-fix diagnosis

Date: 2026-05-26
Team: `ltp-stable383-to-stab-2374dbd5`
Task: 6 / follow-up diagnosis for `poll02,gethostid01,getcpu01,gethostname02`

## Scope and inputs

Leader constraints honored: no QEMU, no `.omx/ultragoal` edit, no
`examples/shell/src/cmd.rs::LTP_STABLE_CASES` edit, no `kill02` promotion, and no
LTP source edits. This is a report-only diagnosis.

Inputs used:

- Current worker task JSON: task 6, `requires_code_change=false`.
- Prior worker report: `docs/ltp-score-improvement-2026-05-26-phase-a/worker2-light-syscall-process-scout-report.md`.
- Leader raw status/log from the main checkout:
  - `docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker2-light-syscall-rv-001.status`
  - `docs/ltp-score-improvement-2026-05-26-phase-a/raw/worker2-light-syscall-rv-001.log`
- `worker2-light-syscall-rv-001-summary.txt` was not present in either this
  worker worktree or the leader checkout at the time of this report, so I parsed
  the leader raw log directly with `scripts/ltp_summary.py` instead.
- Local source surfaces:
  - `examples/shell/src/uspace/select_fdset.rs`
  - `examples/shell/src/uspace/system_info.rs`
  - `examples/shell/src/uspace/syscall_dispatch.rs`
  - `examples/shell/src/uspace/resource_sched.rs`
- Contest LTP C/runtest sources for these cases were not present in this checkout;
  the diagnosis uses raw LTP output line text plus local syscall/libc-facing code.

The raw status records `arch=rv`,
`cases=poll02,gethostid01,getcpu01,gethostname02`, and `run-eval rc=0`.
Parser output for the raw log:

- wrapper PASS: 1
- wrapper FAIL: 7
- internal markers: 20 `TFAIL`, 2 `TCONF`
- timeout: 0
- ENOSYS/not implemented: 1
- panic/trap: 0

RV-only promotion-candidate gate (`--promotion-arches rv --promotion-libcs musl,glibc`)
reports zero candidates and four blocked/incomplete cases.

## Per-case diagnosis

| Case | RV musl result | RV glibc result | Clean/blocker status | Likely root cause | Narrow recommendation |
| --- | --- | --- | --- | --- | --- |
| `poll02` | FAIL, 7 `TFAIL` | FAIL, 7 `TFAIL` | Blocked on both libcs | LTP reports `poll() slept for too long` at 1ms/2ms/5ms timer checks. Local `ppoll`/`poll` path exists, but the implementation spins/yields until `axhal::time::wall_time()` reaches the deadline; this is timer/scheduler precision behavior, not a missing syscall. | Do not promote. No low-risk one-line syscall patch. Any fix should be a timer/yield precision investigation around `select_fdset.rs::sys_poll_until` and platform timer resolution, with a dedicated timing regression; avoid broad scheduler changes inside this easy lane. |
| `gethostid01` | FAIL code 32 with `TCONF: sethostid is undefined` | FAIL code 1, 4 `TFAIL`; `set hostid ... failed: EROFS (30)` | Blocked; musl is TCONF, glibc is real failure | Not a direct kernel syscall gap. glibc `sethostid` is trying to persist hostid state and hits read-only filesystem behavior; musl lacks `sethostid`, causing TCONF. | Do not promote. A kernel syscall patch will not fix the musl TCONF. If leader wants a glibc-only experiment, investigate a writable `/etc/hostid`/filesystem policy, but that is broader than a narrow syscall fix and still cannot satisfy musl+glibc promotion. |
| `getcpu01` | FAIL code 32 with `TCONF: syscall(168) __NR_getcpu not supported on your arch` | FAIL code 1, `TFAIL`, ENOSYS: `getcpu() Failed, errno=38:Function not implemented` | Blocked; glibc has a narrow syscall hole, musl remains TCONF | `linux_raw_sys` defines `__NR_getcpu = 168` for both riscv64 and loongarch64, but `syscall_dispatch.rs` has no `general::__NR_getcpu` arm. | Narrow low-risk patch candidate for glibc only: add `sys_getcpu(process, cpu_ptr, node_ptr, cache_ptr)` that writes `0u32` to non-null `cpu` and `node` pointers and ignores cache, then route `general::__NR_getcpu`. Do not promote from that alone because the current musl LTP binary TCONF still blocks the required libc matrix. |
| `gethostname02` | FAIL code 1 with `TFAIL: len is smaller than the actual size succeeded` | PASS clean | Blocked only on musl | Local kernel exposes `uname` and mutable nodename via `sys_uname`/`sys_sethostname`. On these arch/libc paths, `gethostname` is libc behavior over `uname`; glibc returns expected `ENAMETOOLONG`, musl succeeds on a short buffer, so this is not a missing kernel syscall. | Do not promote. No narrow kernel patch is recommended unless leader explicitly allows libc/test-harness work. Kernel-side hostname already feeds `uname`; changing hostname length/content may be benchmark-specific and risks existing `gethostname01`/`uname*` stable cases. |

## Source anchors and patch-risk notes

- `examples/shell/src/uspace/select_fdset.rs:248-263` implements `sys_ppoll()` and
  `sys_poll_until()`. `sys_poll_until()` loops with `axtask::yield_now()` until
  wall-clock deadline; raw failures are oversleep `TFAIL`, not ENOSYS.
- `examples/shell/src/uspace/select_fdset.rs:266-281` and
  `examples/shell/src/uspace/syscall_dispatch.rs:261-266` show direct `poll` is
  compiled out on riscv64/aarch64/loongarch64, while `ppoll` is available. Because
  the raw failure is timing, enabling direct `poll` alone is unlikely to fix `poll02`.
- `examples/shell/src/uspace/system_info.rs:73-98` constructs `new_utsname` and
  writes `process.hostname()` into `nodename`; `sys_uname()` is at lines 123-124
  and `sys_sethostname()` starts at line 134. This explains why glibc
  `gethostname02` can pass while musl libc semantics still fail.
- `examples/shell/src/uspace/syscall_dispatch.rs:57-59` imports system-info
  syscalls, and lines 284/395 route `uname`/`sethostname`; no `getcpu`,
  `gethostid`, or `gethostname` syscall route exists.
- `linux_raw_sys` has `__NR_getcpu = 168` for both riscv64 and loongarch64, so a
  `getcpu` dispatch arm is mechanically small. The promotion blocker is the musl
  `TCONF`, not just glibc ENOSYS.

## Recommended leader decisions

1. Keep all four cases out of `LTP_STABLE_CASES` for now.
2. If leader wants a safe code experiment, isolate `getcpu01` as a glibc-only
   syscall-hole patch, but gate it honestly as non-promotable until musl no longer
   TCONF-skips/fails the case.
3. Do not spend this easy-first lane on `poll02` until timer precision work is in
   scope; the failure is timing tolerance, not missing syscall plumbing.
4. Treat `gethostid01` as filesystem/libc policy work, not syscall work.
5. Treat `gethostname02` as musl libc/test-contract mismatch unless future raw
   evidence proves a kernel `uname` regression.

## Verification

- Parsed leader raw RV log with `python3 -B scripts/ltp_summary.py` and confirmed
  PASS=1, FAIL=7, internal markers=22, timeout=0, ENOSYS=1, panic/trap=0.
- Ran RV-only `scripts/ltp_summary.py --promotion-candidates --promotion-arches rv
  --promotion-libcs musl,glibc` and confirmed zero candidates with all four cases
  blocked.
- Rechecked stable list invariant from `examples/shell/src/cmd.rs`: total 383,
  unique 383, and none of `poll02,gethostid01,getcpu01,gethostname02,kill02` is
  present.
- Source search found no local contest LTP C/runtest sources for these case names;
  no source or stable-list edits were made.

Subagent spawn/skip evidence: skipped. Task 6 did not require parallel delegation,
and the work was bounded report-only diagnosis from one raw log plus local syscall
surfaces; spawning would duplicate the critical-path local read/parse work.
