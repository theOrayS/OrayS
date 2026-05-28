# LTP stable300 -> stable350 Candidate Matrix

Date: 2026-05-25
Status: **stable350 delivered**

## Live stable list

- Source: `examples/shell/src/cmd.rs::LTP_STABLE_CASES`
- Artifact: `stable350-live.cases`
- Count: 350 total / 350 unique / 0 duplicates
- Explicitly demoted from final list: `kill02`
- Replacement promoted: `abs01`

## Promoted matrix

| Stage | Case group | Cases | Required gate | Result |
| --- | --- | --- | --- | --- |
| stable315 | alarm/time/write/wait/pipe/sched/statfs | `alarm05, alarm07, write05, gettimeofday02, waitpid01, pipe2_02, sched_getscheduler02, fstat03, fstat03_64, statfs02, fstatfs02, fstatfs02_64, sched_getparam03, sched_setparam04, sched_setparam05` | RV+LA stable aggregate, musl+glibc clean | Passed |
| stable330 | fchdir/fcntl/fdatasync/readlinkat/sched/symlink/ftruncate | `fchdir01, fchdir03, fcntl05, fcntl05_64, fcntl12, fcntl12_64, fcntl13, fcntl13_64, fdatasync01, fdatasync02, readlinkat01, sched_setscheduler01, sched_setscheduler02, symlinkat01, ftruncate03_64` | RV+LA stable aggregate, musl+glibc clean | Passed |
| stable350 | fs/proc/process/signal/libc low-risk | `chdir04, chown01, chown02, chown03, chown05, creat05, abs01, mkdir05, statfs02_64, truncate03_64, fork03, fork04, fork07, fork08, fork09, signal05, string01, memcmp01, memcpy01, memset01` | RV+LA final stable aggregate, musl+glibc clean | Passed |

## Evidence table

| Evidence | Arch | Scope | PASS/FAIL | musl | glibc | Internal TFAIL/TBROK/TCONF | timeout | ENOSYS | panic/trap | Marker prefix |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `raw/stable315-rv-aggregate-002-summary.txt` | RV | stable315 aggregate | 630/0 | 315/0 | 315/0 | TCONF=4 read02 only | 0 | 0 | 0 | `raw/stable315-rv-aggregate-002-marker-prefix.txt` bad=0 |
| `raw/stable315-la-aggregate-001-summary.txt` | LA | stable315 aggregate | 630/0 | 315/0 | 315/0 | TCONF=4 read02 only | 0 | 0 | 0 | `raw/stable315-la-aggregate-001-marker-prefix.txt` bad=0 |
| `raw/stable330-rv-aggregate-002-summary.txt` | RV | stable330 aggregate | 660/0 | 330/0 | 330/0 | TCONF=4 read02 only | 0 | 0 | 0 | `raw/stable330-rv-aggregate-002-marker-prefix.txt` bad=0 |
| `raw/stable330-la-aggregate-001-summary.txt` | LA | stable330 aggregate | 660/0 | 330/0 | 330/0 | TCONF=4 read02 only | 0 | 0 | 0 | `raw/stable330-la-aggregate-001-marker-prefix.txt` bad=0 |
| `raw/stable350-rv-final-002-summary.txt` | RV | stable350 final aggregate | 700/0 | 350/0 | 350/0 | TCONF=4 read02 only | 0 | 0 | 0 | `raw/stable350-rv-final-002-marker-prefix.txt` bad=0 |
| `raw/stable350-la-final-002-summary.txt` | LA | stable350 final aggregate | 700/0 | 350/0 | 350/0 | TCONF=4 read02 only | 0 | 0 | 0 | `raw/stable350-la-final-002-marker-prefix.txt` bad=0 |

## Demoted / blocked candidates

| Candidate | Reason not promoted | Evidence / note | Next action |
| --- | --- | --- | --- |
| `kill02` | Failed LA glibc final aggregate with TBROK setup failures despite targeted promise | `raw/stable350-la-final-summary.txt`: PASS 699 / FAIL 1; glibc 349/1; TBROK=4 | Re-run isolated LA glibc and inspect signal/kill setup semantics before future promotion |
| `readlinkat02` | LA musl TFAIL in discovery | Discovery summaries under `raw/followup-*` / `raw/stable350-*` | Fix AT_EMPTY_PATH / O_PATH / symlink boundary semantics before promotion |
| `fork05` | Not promoted; architecture/libc assumptions and ix86-specific behavior need review | Process discovery clean set did not justify final stable inclusion | Re-evaluate source requirements and hidden-test value |
| `signal01`, `pause01`, `pause03`, `signal06`, `kill05` | Signal/timing near-misses need stronger four-way evidence | Follow-up summaries under `raw/followup-*` | Keep in next stable350 follow-up pool |
| `access02`, `access04`, `chmod05`, `statx01`, `writev03`, `mmap04/05/06`, `mprotect01/02`, `munmap01` | User-priority blockers remain not four-way clean | Historical stable300 blocker list; not used as clean evidence | Candidate-first repair lanes for next campaign |

## Honesty notes

- Wrapper success was never used alone for promotion; summary files were produced with `python3 -B scripts/ltp_summary.py`.
- `read02` TCONF remains visible as pass-with-TCONF and is not described as clean.
- Timeout, ENOSYS/not implemented, panic/trap, internal TFAIL, and internal TBROK are all zero in final RV/LA stable350 gates.
