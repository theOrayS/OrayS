# stable300 -> stable350 candidate matrix

Date: 2026-05-25
Mode: Ultragoal + Team, leader-owned promotion gates
Live stable list: **300 total / 300 unique / 0 duplicates**

## Decision summary

No case was promoted in this round. Team discovery plus leader-serialized follow-up fixes produced a useful clean seed set, but not the required stable315 tranche.

Fresh leader-serialized evidence now has **7 cases** clean across RV + LA x musl + glibc:

- `prctl05`
- `sched_getscheduler02`
- `sethostname01`
- `setrlimit01`
- `signal03`
- `signal04`
- `waitpid01`

This is below the stable315 target (+15), so `LTP_STABLE_CASES` stays at stable300.

Promotion remains blocked by the rule that wrapper success alone is insufficient: each candidate must be clean for RV and LA, musl and glibc, with zero new internal `TFAIL`/`TBROK`/`TCONF`, timeout, ENOSYS, panic, or trap.

## Fresh follow-up gate evidence

| Gate | Cases | Result | Promotion use |
| --- | --- | --- | --- |
| RV targeted `followup-rv-targeted-001` | `pipe2_02,waitpid01,sched_getscheduler02,setrlimit01,signal03,signal04,prctl05,sethostname01` | `PASS LTP CASE 13`, `FAIL LTP CASE 3`; `pipe2_02` TBROK on both libc; `waitpid01` musl TFAIL=40; timeout/ENOSYS/panic/trap 0; marker bad=0. | RV-clean subset only: `prctl05,sched_getscheduler02,sethostname01,setrlimit01,signal03,signal04`. |
| LA targeted `followup-la-targeted-004` | RV-clean subset above | `PASS LTP CASE 11`, `FAIL LTP CASE 1`; `sched_getscheduler02` musl TFAIL=1; timeout/ENOSYS/panic/trap 0; marker bad=0. | Pre-fix four-way clean subset: `prctl05,sethostname01,setrlimit01,signal03,signal04`. |
| LA targeted `followup-la-sched_getscheduler02-afterfix-001` | `sched_getscheduler02` only | Parser semantic PASS 2 / FAIL 0; `ltp-musl 1/0`, `ltp-glibc 1/0`; internal TFAIL/TBROK/TCONF=0; timeout/ENOSYS/panic/trap 0; marker prefix bad=0. | Adds `sched_getscheduler02` to four-way clean seeds. |
| RV targeted `followup-rv-waitpid01-maskrestore-001` | `waitpid01` only | PASS 2 / FAIL 0; `ltp-musl 1/0`, `ltp-glibc 1/0`; internal TFAIL/TBROK/TCONF=0; timeout/ENOSYS/panic/trap 0; marker prefix bad=0. | Confirms RV waitpid fix. |
| LA targeted `followup-la-waitpid01-maskrestore-001` | `waitpid01` only | PASS 2 / FAIL 0; `ltp-musl 1/0`, `ltp-glibc 1/0`; internal TFAIL/TBROK/TCONF=0; timeout/ENOSYS/panic/trap 0; marker prefix bad=0. | Adds `waitpid01` to four-way clean seeds. |
| RV guard `followup-rv-waitpid-signal-guard-001` | `waitpid01,rt_sigprocmask01,sigprocmask01,rt_sigsuspend01,sigsuspend01,signal03,signal04,waitpid10` | PASS 16 / FAIL 0; both libc 8/0; internal TFAIL/TBROK/TCONF=0; timeout/ENOSYS/panic/trap 0; marker prefix bad=0. | Regression guard for signal mask/wait semantics. |
| LA guard `followup-la-waitpid-signal-guard-001` | same guard set | PASS 16 / FAIL 0; both libc 8/0; internal TFAIL/TBROK/TCONF=0; timeout/ENOSYS/panic/trap 0; marker prefix bad=0. | Regression guard for both architectures. |
| RV targeted `followup-rv-pipe2_02-resource-prestage-003` | `pipe2_02` only | PASS 0 / FAIL 2; both libc `TBROK=1` from helper resource copy; timeout/ENOSYS/panic/trap 0; marker prefix bad=0. | Still blocked; not promotion evidence. |
| LA targeted `followup-la-targeted-001/002/003` | same intent | Aborted/untrusted before completion due duplicated LA attempts; raw logs renamed `*-aborted-untrusted.log`. | Excluded from promotion evidence. |

## Current candidate table

| Case / batch | Subsystem | Current evidence | Decision |
| --- | --- | --- | --- |
| `prctl05`, `sethostname01`, `setrlimit01`, `signal03`, `signal04` | proc/rlimit/signal | Fresh RV and LA targeted gates clean for musl+glibc. | Keep as high-confidence next tranche seeds; not enough alone for stable315. |
| `sched_getscheduler02` | process/sched | RV clean for musl+glibc; after loader wrapper fix, LA musl+glibc targeted gate clean (`raw/followup-la-sched_getscheduler02-afterfix-001-summary.txt`). | Four-way clean seed; still not enough for stable315 without more cases. |
| `waitpid01` | process/wait/signal | After fork-time signal mask restore fix, RV and LA targeted gates are clean for musl+glibc; RV/LA signal guard batches also clean. | Four-way clean seed; do not promote until a >=15 clean tranche exists. |
| `pipe2_02` | fd/pipe/helper cwd | Fresh RV targeted still TBROK on both libc: `Failed to copy '/musl/ltp/testcases/bin/pipe2_02_child' to '.'` and glibc equivalent. Three leader attempts were reverted. | Repair helper/resource semantics before any LA gate. |
| `chmod05` | permissions/VFS | RV blocker batch: glibc PASS, musl TBROK. | Repair musl setup/special-bit behavior first; no promotion. |
| `writev03` | iovec/SIGPIPE | RV blocker batch: both libc fail with TCONF=1 / code 32. | Not clean; do not launder TCONF. |
| `access02`, `access04` | permissions/errno | RV blocker batch: `access02` TFAIL, `access04` TBROK on both libc. | Repair permissions/setup semantics first. |
| `statx01` | metadata/statx | RV blocker batch: both libc TBROK + ENOSYS marker, despite source dispatch existing. | Fresh serialized statx rerun and syscall argument/ABI audit required. |
| `mmap04`, `mmap05`, `mmap06`, `mprotect01`, `mprotect02`, `munmap01` | VM/protection/signal | RV blocker batch: real FAIL/TFAIL/TBROK or segfault-style code 139. | High hidden-test value but not near-clean. |
| Batch-A one-combo tails (`alarm05`, `alarm07`, `write05`, `fchmod05`, `fstat03`, `fstat03_64`, `statfs02`, `fstatfs02`, `gethostname02`, `gettid02`, `nice04`, `sbrk01`) | mixed | RV batch-A found glibc-only or musl-only partial passes. | Use as next discovery/repair queue, not stable additions. |

## Evidence files

- Fresh follow-up parsed summaries: `raw/followup-rv-targeted-001-summary.txt`, `raw/followup-la-targeted-004-summary.txt`, `raw/followup-la-sched_getscheduler02-afterfix-001-summary.txt`, `raw/followup-rv-waitpid01-maskrestore-001-summary.txt`, `raw/followup-la-waitpid01-maskrestore-001-summary.txt`, `raw/followup-rv-waitpid-signal-guard-001-summary.txt`, `raw/followup-la-waitpid-signal-guard-001-summary.txt`, `raw/followup-rv-pipe2_02-resource-prestage-003-summary.txt`.
- Marker-prefix follow-up scans: `raw/followup-marker-prefix-check.txt`, `raw/followup-waitpid-marker-prefix-check.txt`.
- Worker discovery matrix: `worker1-discovery-candidate-matrix.md`.
- VFS/permissions report: `worker2-vfs-permission-report.md`.
- fd/pipe/iovec report: `worker3-fd-pipe-iovec-report.md`.
- process/wait/sched/rlimit report: `worker4-process-wait-report.md`.
- mmap/mprotect/guardrail report: `worker5-mmap-guardrail-report.md`.
- Parsed discovery summaries: `raw/batch-a-rv-summary.txt`, `raw/blocker-batch-rv-summary.txt`, `raw/worker5-readonly-blocker-batch-rv-summary.txt`.
- Aborted/untrusted attempts are recorded by `raw/post-team-candidate*.status` and `raw/followup-la-targeted-00{1,2,3}-aborted-untrusted.log`; they must not be used for promotion.

## Next clean candidate order

1. Keep the 7 fresh four-way clean cases as tranche seeds: `prctl05,sched_getscheduler02,sethostname01,setrlimit01,signal03,signal04,waitpid01`.
2. Next prioritize `pipe2_02` helper resources, because it is a real multi-libc blocker and the latest leader workaround did not solve LTP's `SAFE_CP()` resource path.
3. Continue discovery on the one-combo tails (`alarm05`, `write05`, `fchmod05`, `fstat03`, `statfs02`, `gethostname02`, `gettid02`, `nice04`, `sbrk01`).
4. Once at least 15 candidates have fresh RV+LA x musl+glibc clean evidence, update `LTP_STABLE_CASES` and run stable315 aggregate gates.
