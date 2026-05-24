# Candidate matrix: stable250 -> stable300 (2026-05-24 phase-a)

Status: **updated after leader-verified stable270 gate**. This matrix is evidence classification, not promotion by itself.

## Promotion rule used

A case can be promoted only when all required rows are clean: RV + LA, musl + glibc, wrapper PASS/zero status, internal `TFAIL=0`, `TBROK=0`, `TCONF=0`, timeout 0, ENOSYS 0, panic/trap 0. Existing `read02` stays transparent `pass_with_tconf` and does not make new TCONF acceptable.

## Baseline and delivered state

| Item | Value |
| --- | --- |
| Live baseline at start | 250 entries, 250 unique, 0 duplicates (`stable250-live.cases`) |
| Delivered in this run | 270 entries, 270 unique, 0 duplicates (`stable270-live.cases`) |
| Stable270 RV aggregate | `stable270-rv-aggregate-summary.txt`: PASS 540, FAIL 0, known `read02` TCONF only |
| Stable270 LA aggregate | `stable270-la-aggregate-summary.txt`: PASS 540, FAIL 0, known `read02` TCONF only |

## Promoted to stable270

| Subsystem | Cases | Evidence |
| --- | --- | --- |
| writev user priority | `writev05`, `writev06`, `writev07` | RV user-priority matrix clean subset + `user-priority-rv-clean-writev-la-summary.txt` |
| iovec validation | `readv02`, `writev01` | `fix-iovec-readv-writev-rv-summary.txt`, `fix-iovec-readv-writev-la-summary.txt` |
| waitid base | `waitid01`, `waitid02`, `waitid03`, `waitid04` | `fix-waitid-rv-summary.txt`, `fix-waitid-la-summary.txt` |
| waitid extended clean subset | `waitid05`, `waitid06`, `waitid09`, `waitid11` | `target-waitid-extended-rv-summary.txt`, `target-waitid-kill-la-confirm-summary.txt` |
| signal/kill clean subset | `kill07`, `kill08`, `kill09` | `target-kill-signal-rv-summary.txt`, `target-waitid-kill-la-confirm-summary.txt` |
| positional vectored IO | `preadv01_64`, `preadv02_64`, `pwritev01_64`, `pwritev02_64` | `fix-preadv-pwritev-rv2-summary.txt`, `fix-preadv-pwritev-la-summary.txt` |

## Clean but not promoted after stable270

Only four additional fully clean cases were found. They are retained for a future stable285 wave but were not added here because they do not reach the next required milestone.

| Case | Classification | Evidence |
| --- | --- | --- |
| `sched_getattr02` | RV+LA musl+glibc clean | `target-sched-known-clean-rv-summary.txt`, `target-sched-known-clean-la-summary.txt`, `target-post270-rv-clean-la-confirm-summary.txt` |
| `open09` | RV+LA musl+glibc clean | `target-post270-batch1-rv-summary.txt`, `target-post270-rv-clean-la-confirm-summary.txt` |
| `fcntl23` | RV+LA musl+glibc clean | `target-post270-batch4-fcntl-rv-summary.txt`, `target-post270-rv-clean-la-confirm-summary.txt` |
| `getegid01_16` | RV+LA musl+glibc clean | `target-post270-batch2-uid16-rv-summary.txt`, `target-post270-rv-clean-la-confirm-summary.txt` |

## User-priority A-E classification

| Group | Cases | Result |
| --- | --- | --- |
| A permission/VFS/error | `access02`, `access04`, `open08`, `open13`, `chmod05`, `statx01` | `user-priority-ae-rv-summary.txt`: not clean; real TFAIL/TBROK/ENOSYS present. Not promoted. |
| B read/write vector | `readv02`, `writev01`, `writev03`, `writev05`, `writev06`, `writev07` | Promoted: `readv02`, `writev01`, `writev05`, `writev06`, `writev07`; `writev03` not clean in discovery. |
| C pipe/fd lifecycle | `pipe12`, `pipe2_01`, `pipe2_02`, `pipe2_04`, `pipe13` | Not clean; `pipe13` timed out. Timeout is not PASS. |
| D wait/fork extension | `waitpid01`, `waitid01`, `waitid02`, `waitid03`, `waitid04` | Promoted `waitid01..04`; `waitpid01` still musl TFAIL in `fix-waitpid01-rv-summary.txt`. |
| E mmap/signal/page permission | `mmap05`, `mprotect01`, `mprotect02`, `munmap01`, `mmap04`, `mmap06` | Not clean in user-priority RV discovery. Not promoted. |

## Post270 blocked discovery batches

| Batch | Summary | Clean rows | Blocking signals |
| --- | --- | --- | --- |
| `target-post270-batch1-rv-summary.txt` | PASS 9, FAIL 21; internal TFAIL/TBROK 38; timeout 1; ENOSYS 6 | RV both-libc clean: `sched_getattr02`, `open09` | `times03` timeout; multiple TFAIL/TBROK/ENOSYS; several one-libc-only clean rows |
| `target-post270-batch2-uid16-rv-summary.txt` | PASS 2, FAIL 28; internal TCONF 36 | RV both-libc clean: `getegid01_16` | most uid16 cases TCONF |
| `target-post270-batch3-rv-summary.txt` | all candidate rows failed as missing/not runnable | none | test image missing entries / code -1 |
| `target-post270-batch4-fcntl-rv-summary.txt` | PASS 2, FAIL 20; internal TFAIL/TCONF 366 | RV both-libc clean: `fcntl23` | lease/lock TFAIL and tmpfs TCONF across neighboring fcntl cases |
| `target-post270-batch5-mixed-rv-summary.txt` | PASS 2, FAIL 28; internal TFAIL/TBROK/TCONF 51; ENOSYS 8 | no both-libc clean row | openat2 TCONF, close_range ENOSYS/TBROK, statx failures/TBROK/TCONF, timer_create TCONF, nanosleep timing TFAIL |

## Next likely work

To continue toward stable285, start from the four clean unpromoted cases above, then fix real blockers instead of broad promotion. Highest-value blockers by target coverage: `waitpid01` musl signal/wait status behavior, pipe lifecycle timeouts/failures, permission/VFS error-code mismatches, and statx/openat2/close_range syscall coverage.
