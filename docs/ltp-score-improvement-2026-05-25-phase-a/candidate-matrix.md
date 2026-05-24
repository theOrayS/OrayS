# stable300 -> stable350 candidate matrix

Date: 2026-05-25
Mode: Ultragoal + Team, leader-owned promotion gates
Live stable list: **300 total / 300 unique / 0 duplicates**

## Decision summary

No case was promoted in this round. The Team produced useful discovery and two candidate implementation changes, but there is not yet leader-serialized RV+LA x musl+glibc clean evidence for any new case. Therefore `LTP_STABLE_CASES` stays at stable300.

Promotion remains blocked by the rule that wrapper PASS alone is insufficient: each candidate must be clean for RV and LA, musl and glibc, with zero new internal `TFAIL`/`TBROK`/`TCONF`, timeout, ENOSYS, panic, or trap.

## Current candidate table

| Case / batch | Subsystem | Current evidence | Decision |
| --- | --- | --- | --- |
| `sched_getscheduler02` | process/sched | RV batch-A clean for musl+glibc; prior worker matrix still has LA musl failure/unproven current clean evidence. | Do not promote before fresh LA x musl/glibc targeted gate. |
| `chmod05` | permissions/VFS | RV blocker batch: glibc PASS, musl TBROK. | Repair musl setup/special-bit behavior first; no promotion. |
| `waitpid01` | process/wait/signal | RV blocker batch: glibc PASS, musl TFAIL=40 before signal fix; post-fix targeted run was aborted/untrusted. | Needs clean serialized RV rerun, then LA. |
| `pipe2_02` | fd/pipe/helper cwd | Runner LTPROOT/PATH/cwd fix integrated, but post-fix targeted run was aborted/untrusted. | Needs clean serialized RV rerun, then LA. |
| `writev03` | iovec/SIGPIPE | RV blocker batch: both libc fail with TCONF=1 / code 32. | Not clean; do not launder TCONF. |
| `access02`, `access04` | permissions/errno | RV blocker batch: `access02` TFAIL, `access04` TBROK on both libc. | Repair permissions/setup semantics first. |
| `statx01` | metadata/statx | RV blocker batch: both libc TBROK + ENOSYS marker, despite source dispatch existing. | Fresh serialized statx rerun and syscall argument/ABI audit required. |
| `mmap04`, `mmap05`, `mmap06`, `mprotect01`, `mprotect02`, `munmap01` | VM/protection/signal | RV blocker batch: real FAIL/TFAIL/TBROK or segfault-style code 139. | High hidden-test value but not near-clean. |
| Batch-A one-combo tails (`alarm05`, `alarm07`, `write05`, `fchmod05`, `fstat03`, `fstat03_64`, `statfs02`, `fstatfs02`, `gethostname02`, `gettid02`, `nice04`, `sbrk01`) | mixed | RV batch-A found glibc-only or musl-only partial passes; no four-way clean case except RV-only `sched_getscheduler02`. | Use as next discovery/repair queue, not stable additions. |

## Evidence files

- Worker discovery matrix: `worker1-discovery-candidate-matrix.md`.
- VFS/permissions report: `worker2-vfs-permission-report.md`.
- fd/pipe/iovec report: `worker3-fd-pipe-iovec-report.md`.
- process/wait/sched/rlimit report: `worker4-process-wait-report.md`.
- mmap/mprotect/guardrail report: `worker5-mmap-guardrail-report.md`.
- Parsed discovery summaries: `raw/batch-a-rv-summary.txt`, `raw/blocker-batch-rv-summary.txt`, `raw/worker5-readonly-blocker-batch-rv-summary.txt`.
- Aborted/untrusted post-Team targeted attempts are recorded by `raw/post-team-candidate*.status` and must not be used for promotion.

## Next clean candidate order

1. Re-run **one** serialized RV targeted gate for `pipe2_02,waitpid01,sched_getscheduler02,setrlimit01,signal03,signal04,prctl05,sethostname01` using a unique log name after confirming no active evaluator/build process.
2. If and only if RV is clean for a subset, run the same subset on LA.
3. If no subset is clean, repair in this order: `pipe2_02` helper/resource semantics, `waitpid01` signal wait-status, then Batch-A musl/glibc one-combo tails.
