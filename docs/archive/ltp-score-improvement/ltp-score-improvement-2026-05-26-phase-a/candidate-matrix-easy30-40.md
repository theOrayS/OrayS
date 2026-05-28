# Candidate matrix: stable383 -> stable413/423 easy-first pool

Date: 2026-05-26
Team: `ltp-stable383-to-stab-2374dbd5`
Worker: `worker-1`
Task: `task-1` / Inventory + easy candidate matrix

## Scope and guardrails

- Report/discovery only: no QEMU was started, `examples/shell/src/cmd.rs::LTP_STABLE_CASES` was not edited, and `.omx/ultragoal` remained leader-owned.
- This matrix is a scout queue, not promotion evidence. A case can be promoted only after fresh RV+LA x musl+glibc clean evidence and aggregate stable gates.
- Clean means wrapper PASS plus no internal TFAIL/TBROK/new TCONF, timeout, ENOSYS/not-implemented, panic, or trap. The inherited `read02` TCONF remains transparent and is not a model for new cases.

## Source refresh

- Live stable list in this worktree: **383 total / 383 unique / 0 duplicates**.
- Leader-root `examples/shell/src/cmd.rs` was also checked separately during this lane and matched stable383.
- Phase-a plan/prompt reviewed: `docs/ltp-score-improvement-2026-05-26-phase-a/plan-stable383-to-413-easy30-40.md` and `next-session-prompt-stable383-to-423-easy30-40.md`.
- Phase-c positive/negative evidence reviewed: `docs/ltp-score-improvement-2026-05-25-phase-c/candidate-matrix.md`, `stable383-promotion-gate-report.md`, `stable400-attempt3-scout-report.md`, `stable400-attempt5-inventory-statx-report.md`, and `worker1-discovery-candidate-matrix.md`.
- Read-only sdcard inventory was refreshed with `debugfs -R "ls -p /{musl,glibc}/ltp/testcases/bin"` on `/root/oskernel2026-orays/sdcard-{rv,la}.img`.

## Inventory summary

| Image/libc | LTP bin entries excluding `.`/`..` |
| --- | ---: |
| `rv:musl` | 2822 |
| `rv:glibc` | 2842 |
| `la:musl` | 2822 |
| `la:glibc` | 2842 |
| Four-way common names | 2822 |

- Candidate pool size: **74** cases; all **74** are present in RV/LA x musl/glibc sdcard inventories.
- Already stable among this pool: **0**.

## First-wave easy30-40 queue

These 40 cases are the lowest-cost first scout proposal because they are four-way inventory-present, not already stable, and have no case-specific phase-c blocker row found (except normal unproven status). Run in RV batches first; only RV-clean cases should go to LA.

### Batch A: lightweight syscall + metadata/statfs/getdents + first scalar IO

```text
poll02,times03,gethostname02,gethostid01,getpgid01,getcpu01,fstat02,fstat02_64,fstatfs01,fstatfs01_64,statfs01,statfs01_64,statfs03,statfs03_64,statvfs01,getcwd03,getcwd04,getdents01,getdents02,pread02
```

### Batch B: remaining scalar IO/sendfile/open plus one low-risk create candidate

```text
pread02_64,pwrite02,pwrite02_64,pwrite04,pwrite04_64,sendfile02,sendfile03,sendfile04,sendfile05,sendfile02_64,sendfile03_64,sendfile04_64,sendfile05_64,open06,open07,open10,open11,open12,open14,creat08
```

## Full 74-case easy-first matrix

| Case | Subsystem | Four-way sdcard present | Already stable | Prior RV/LA status or blocker | Estimated cost/risk | Recommendation |
| --- | --- | --- | --- | --- | --- | --- |
| `poll02` | light syscall/libc | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `times03` | light syscall/libc | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `gethostname02` | light syscall/libc | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `gethostid01` | light syscall/libc | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `getpgid01` | light syscall/libc | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `getcpu01` | light syscall/libc | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `fork13` | process/signal | yes | no | phase-c worker4: prior RV scout timeout/failure; no LA spend before RV root cause | high/blocker | defer from easy-first promotion |
| `fork14` | process/signal | yes | no | phase-c worker4: prior RV scout timeout/failure; no LA spend before RV root cause | high/blocker | defer from easy-first promotion |
| `clone06` | process/signal | yes | no | no phase-c targeted result found; adjacent clone02/04/05 had blockers, so scout only after lighter syscalls | medium/high | reserve after first-wave or fix lane |
| `clone07` | process/signal | yes | no | no phase-c targeted result found; adjacent clone02/04/05 had blockers, so scout only after lighter syscalls | medium/high | reserve after first-wave or fix lane |
| `clone08` | process/signal | yes | no | no phase-c targeted result found; adjacent clone02/04/05 had blockers, so scout only after lighter syscalls | medium/high | reserve after first-wave or fix lane |
| `clone09` | process/signal | yes | no | no phase-c targeted result found; adjacent clone02/04/05 had blockers, so scout only after lighter syscalls | medium/high | reserve after first-wave or fix lane |
| `kill05` | process/signal | yes | no | phase-c worker4: RV TFAIL+TBROK history; needs raw subtest text | high/blocker | defer from easy-first promotion |
| `kill10` | process/signal | yes | no | phase-c worker4: RV musl timeout history; signal wakeup/pause risk | high/blocker | defer from easy-first promotion |
| `fstat02` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `fstat02_64` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `fstatfs01` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `fstatfs01_64` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `statfs01` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `statfs01_64` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `statfs03` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `statfs03_64` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `statvfs01` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `getcwd03` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `getcwd04` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `getdents01` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `getdents02` | metadata/statfs/getdents | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `pread02` | FD offset IO | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `pread02_64` | FD offset IO | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `pwrite02` | FD offset IO | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `pwrite02_64` | FD offset IO | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `pwrite04` | FD offset IO | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `pwrite04_64` | FD offset IO | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `preadv01` | iovec IO | yes | no | no case-specific phase-c result found; iovec lane has TCONF/TFAIL caution, scout after scalar pread/pwrite | medium/high | reserve after scalar IO first-wave |
| `preadv02` | iovec IO | yes | no | no case-specific phase-c result found; iovec lane has TCONF/TFAIL caution, scout after scalar pread/pwrite | medium/high | reserve after scalar IO first-wave |
| `preadv03` | iovec IO | yes | no | no case-specific phase-c result found; iovec lane has TCONF/TFAIL caution, scout after scalar pread/pwrite | medium/high | reserve after scalar IO first-wave |
| `pwritev01` | iovec IO | yes | no | no case-specific phase-c result found; iovec lane has TCONF/TFAIL caution, scout after scalar pread/pwrite | medium/high | reserve after scalar IO first-wave |
| `pwritev02` | iovec IO | yes | no | no case-specific phase-c result found; iovec lane has TCONF/TFAIL caution, scout after scalar pread/pwrite | medium/high | reserve after scalar IO first-wave |
| `pwritev03` | iovec IO | yes | no | no case-specific phase-c result found; iovec lane has TCONF/TFAIL caution, scout after scalar pread/pwrite | medium/high | reserve after scalar IO first-wave |
| `sendfile02` | sendfile/FD copy | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `sendfile03` | sendfile/FD copy | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `sendfile04` | sendfile/FD copy | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `sendfile05` | sendfile/FD copy | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `sendfile02_64` | sendfile/FD copy | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `sendfile03_64` | sendfile/FD copy | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `sendfile04_64` | sendfile/FD copy | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `sendfile05_64` | sendfile/FD copy | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `open06` | VFS open | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `open07` | VFS open | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `open10` | VFS open | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `open11` | VFS open | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `open12` | VFS open | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `open14` | VFS open | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `creat04` | VFS create | yes | no | phase-c light scout: RV musl UNKNOWN then FAIL/TFAIL; not clean | high/blocker | defer from easy-first promotion |
| `creat06` | VFS create | yes | no | phase-c light scout: RV musl FAIL with TCONF; not clean | high/blocker | defer from easy-first promotion |
| `creat07` | VFS create | yes | no | phase-c light scout: RV musl FAIL with TCONF; not clean | high/blocker | defer from easy-first promotion |
| `creat08` | VFS create | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | low/medium | RV scout first-wave |
| `creat09` | VFS create | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `mkdir03` | VFS mkdir | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `mkdir04` | VFS mkdir | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `mkdir09` | VFS mkdir | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `rmdir02` | VFS rmdir | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `rmdir03` | VFS rmdir | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `unlink07` | VFS unlink | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `unlink08` | VFS unlink | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `unlink09` | VFS unlink | yes | no | no fresh phase-c targeted row found; candidate is inventory-present but unproven | medium | reserve scout; drop on first internal failure |
| `fs_perms` | fs-suite substitute | yes | no | no case-specific clean proof found; fs-suite can hit timeout/missing path/ENOSYS, scout only as substitutes | medium/high | reserve substitute after first-wave |
| `rwtest` | fs-suite substitute | yes | no | no case-specific clean proof found; fs-suite can hit timeout/missing path/ENOSYS, scout only as substitutes | medium/high | reserve substitute after first-wave |
| `writetest` | fs-suite substitute | yes | no | no case-specific clean proof found; fs-suite can hit timeout/missing path/ENOSYS, scout only as substitutes | medium/high | reserve substitute after first-wave |
| `iogen` | fs-suite substitute | yes | no | no case-specific clean proof found; fs-suite can hit timeout/missing path/ENOSYS, scout only as substitutes | medium/high | reserve substitute after first-wave |
| `fs_inod` | fs-suite substitute | yes | no | no case-specific clean proof found; fs-suite can hit timeout/missing path/ENOSYS, scout only as substitutes | medium/high | reserve substitute after first-wave |
| `openfile` | VFS open | yes | no | no case-specific clean proof found; fs-suite can hit timeout/missing path/ENOSYS, scout only as substitutes | medium/high | reserve substitute after first-wave |
| `inode02` | fs-suite substitute | yes | no | phase-c: RV musl+glibc clean, LA musl clean, LA glibc timeout; blocked | high/blocker | defer from easy-first promotion |
| `ftest06` | fs-suite substitute | yes | no | phase-c scout: RV musl+glibc wrapper FAIL 4; blocked | high/blocker | defer from easy-first promotion |

## Explicit non-goals / skip-now list carried from phase-c

- Do not use existing artifacts to promote `readlinkat02`: RV and LA-glibc were clean, but LA-musl fails because the zero-size testcase reaches the syscall as `bufsiz=1`.
- Do not promote `kill02` from targeted clean rows: LA aggregate had `kill02` TBROK/setup timeout.
- Keep `access04`, `chmod06`, `fchmod06`, `chmod07`, and `fchmod02` out of easy-first batches until tmpfs/group setup blockers are repaired.
- Keep `waitid07`, `waitid08`, `waitid10`, `munmap01`, `mmap04`, `mmap05`, `mprotect01`, `mprotect02`, `pipe07`, and `pipe15` out of this easy-first lane; they have real phase-c failures.
- Avoid `pipe02` in broad batches because prior discovery hit panic/trap.
- Avoid `lseek03`-`lseek10` because phase-c inventory/scout showed missing testcase binaries; avoid `lseek11` until SEEK_DATA/SEEK_HOLE semantics are real.
- Avoid `statx04`-`statx12` in this easy-first lane; phase-c statx-tail scout hit device/tool/config blockers.

## Suggested next execution by leader/other lanes

1. RV scout Batch A with a 45-60s timeout and parse using `python3 -B scripts/ltp_summary.py`.
2. Send only RV musl+glibc clean cases to LA; drop any case with TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
3. If Batch A yields enough clean cases, promote in 8-12 case chunks (`stable393`, `stable403`, `stable413`) with serialized aggregate gates.
4. If Batch A under-yields, scout Batch B, then only then pull from reserve rows marked medium risk.

## Subagent findings integrated

- Subagent `019e64a3-09d6-7503-bf28-ee95d2cf4d1d` confirmed phase-c is the source of positive/negative evidence, phase-a had plan/prompt but no `candidate-matrix-easy30-40.md`, and highlighted blockers including `readlinkat02`, `inode02`, `kill02`, `waitid*`, `pipe02`, and statx tail.
- Subagent `019e64a3-2f6c-71a0-9299-e9a89f89eb14` independently recounted stable383, confirmed the prompt pool is inventory-present in RV musl/glibc lists and not already stable, and grouped the pool into light syscall/process, metadata/statfs, FD/IO, VFS create/open/remove, and fs-suite substitutes.
- Local worker-1 additionally refreshed LA sdcard inventory read-only with `debugfs`, so this report uses four-way RV/LA x musl/glibc presence rather than RV-only presence.

## Verification performed in this lane

- No QEMU / `run-eval.sh` was started.
- Live stable list counted from worktree and leader-root `examples/shell/src/cmd.rs`: 383 total / 383 unique / 0 duplicates.
- Read-only `debugfs` inventory confirmed all 74 candidate-pool cases exist in RV/LA x musl/glibc sdcard images.
- Phase-c negative evidence was applied before ranking blockers and first-wave candidates.
