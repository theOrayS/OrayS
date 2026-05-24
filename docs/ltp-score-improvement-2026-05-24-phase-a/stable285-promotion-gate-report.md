# stable285 promotion gate report

Status: **NOT REACHED in this run**.

## Highest trusted result before this gate

- Live stable list after code changes: stable270 (270 entries, 270 unique, 0 duplicates).
- RV and LA stable270 aggregate gates passed with PASS 540 / FAIL 0 per architecture and only known transparent `read02` TCONF.

## Additional clean candidates found after stable270

These cases have RV+LA, musl+glibc targeted clean evidence, but were **not promoted** because the next requested phase is stable285 and only four additional clean cases were available:

| Case | RV evidence | LA evidence | Notes |
| --- | --- | --- | --- |
| `sched_getattr02` | `target-sched-known-clean-rv-summary.txt` and `target-post270-batch1-rv-summary.txt` | `target-sched-known-clean-la-summary.txt`, reconfirmed by `target-post270-rv-clean-la-confirm-summary.txt` | clean; `sched_getscheduler02` is not clean on LA musl |
| `open09` | `target-post270-batch1-rv-summary.txt` | `target-post270-rv-clean-la-confirm-summary.txt` | clean; permission/access-mode semantics covered by stable270 fixes |
| `fcntl23` | `target-post270-batch4-fcntl-rv-summary.txt` | `target-post270-rv-clean-la-confirm-summary.txt` | clean; neighboring lease/locking cases are not clean |
| `getegid01_16` | `target-post270-batch2-uid16-rv-summary.txt` | `target-post270-rv-clean-la-confirm-summary.txt` | clean, but kept unpromoted with the rest of the shortfall |

## Blockers from post270 discovery

- User-priority A/C/E groups remain mostly blocked:
  - `access02/access04/open08/open13/chmod05/statx01` produced failures/TBROK/TFAIL/ENOSYS patterns in `user-priority-ae-rv-summary.txt` and were not clean.
  - `pipe12/pipe2_* / pipe13` included failures and `pipe13` timeout; not promotable.
  - `mmap05/mprotect*/munmap01/mmap04/mmap06` were not clean in the user-priority RV run.
- `waitpid01` improved on glibc but still failed on musl with internal TFAIL (`fix-waitpid01-rv-summary.txt`), so it was not promoted.
- `target-post270-batch1-rv-summary.txt`: only `sched_getattr02` and `open09` were clean for both RV libc variants; other rows had TFAIL/TBROK/timeout/ENOSYS or were only one-libc clean.
- `target-post270-batch3-rv-summary.txt`: all cases were missing from the test image / not runnable (`FAIL code -1`), so no promotion.
- `target-post270-batch4-fcntl-rv-summary.txt`: only `fcntl23` was clean; most fcntl lease/lock cases had large TFAIL/TCONF counts.
- `target-post270-batch5-mixed-rv-summary.txt`: no both-libc clean case; examples include openat2 TCONF, close_range ENOSYS/TBROK, statx failures/TBROK/TCONF, timer_create TCONF, nanosleep timing TFAIL.

## Decision

Do not edit `LTP_STABLE_CASES` beyond stable270 in this run. Promoting four extra cases would not satisfy the stable285 phase and would require another aggregate gate while still leaving the durable goal below the next milestone.
