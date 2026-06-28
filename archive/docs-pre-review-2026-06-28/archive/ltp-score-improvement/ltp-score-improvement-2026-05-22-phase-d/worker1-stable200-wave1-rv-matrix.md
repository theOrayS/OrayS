# Worker 1 STABLE200 wave1 RV result matrix

Scope: read-only extraction from leader-root artifacts; no QEMU/evaluator run, no code edits, no `LTP_STABLE_CASES` edit, no `.omx/ultragoal` mutation.

## Source artifacts

- Raw log: `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable200-wave1-rv.log`
- Candidate list: `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-22-phase-d/stable200-wave1-candidates.cases`
- Parser used: `python3 -B scripts/ltp_summary.py --json <raw-log>` from this worker worktree.

## Parser totals

- Candidate cases requested: 67
- Parser rows observed: 134
- PASS wrapper rows: 22
- FAIL wrapper rows: 112
- Internal markers: TFAIL=1333, TBROK=39, TCONF=4
- Timeout rows: 5
- ENOSYS/not-implemented rows: 8
- Panic/trap rows: 0

## Recommended LA follow-up subset

These are the only RV both-libc clean cases in this wave and are suitable for LA follow-up before any leader-owned promotion decision:

```text
setgroups01
setgroups02
setreuid01
setuid01
statx02
open04
sched_getscheduler02
```

Count: 7 cases.

## Not promotable from this RV artifact

- PASS but with TCONF: setpriority01, getrusage02
- Partial one-libc RV clean only: gettimeofday02, pipe02, pipe08, fstatfs02
- Incomplete/missing parser row: none
- RV blocked: 54 cases (see matrix for reasons)

## Per-case matrix

| Case | rv/musl wrapper+internal | rv/glibc wrapper+internal | Verdict | Blocked reason / notes |
| --- | --- | --- | --- | --- |
| `setgroups01` | PASS<br>code=0<br>1039ms | PASS<br>code=0<br>2796ms | RV_CLEAN_BOTH_LIBC | clean RV both-libc |
| `setgroups02` | PASS<br>code=0<br>1137ms | PASS<br>code=0<br>1323ms | RV_CLEAN_BOTH_LIBC | clean RV both-libc |
| `setreuid01` | PASS<br>code=0<br>916ms | PASS<br>code=0<br>1632ms | RV_CLEAN_BOTH_LIBC | clean RV both-libc |
| `setuid01` | PASS<br>code=0<br>1140ms | PASS<br>code=0<br>1212ms | RV_CLEAN_BOTH_LIBC | clean RV both-libc |
| `statx02` | PASS<br>code=0<br>961ms | PASS<br>code=0<br>1173ms | RV_CLEAN_BOTH_LIBC | clean RV both-libc |
| `open04` | PASS<br>code=0<br>2453ms | PASS<br>code=0<br>2875ms | RV_CLEAN_BOTH_LIBC | clean RV both-libc |
| `sched_getscheduler02` | PASS<br>code=0<br>910ms | PASS<br>code=0<br>1418ms | RV_CLEAN_BOTH_LIBC | clean RV both-libc |
| `getpgid01` | FAIL<br>code=3<br>1263ms<br>TFAIL=1;TBROK=1 | FAIL<br>code=3<br>1559ms<br>TFAIL=1;TBROK=1 | BLOCKED_RV | musl: wrapper FAIL code=3; musl: TFAIL=1; musl: TBROK=1; glibc: wrapper FAIL code=3; glibc: TFAIL=1; glibc: TBROK=1 |
| `getgroups02` | FAIL<br>code=-1<br>8ms<br>missing-testcase | FAIL<br>code=-1<br>6ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/getgroups02, /musl/ltp/testcases/bin/getgroups02; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `getpriority03` | FAIL<br>code=-1<br>4ms<br>missing-testcase | FAIL<br>code=-1<br>2ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/getpriority03, /musl/ltp/testcases/bin/getpriority03; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `setpriority01` | PASS<br>code=0<br>1123ms<br>TCONF=1 | PASS<br>code=0<br>1739ms<br>TCONF=1 | PASS_WITH_TCONF_NOT_PROMOTABLE | musl: TCONF=1; glibc: TCONF=1 |
| `setpriority02` | FAIL<br>code=1<br>1066ms<br>TFAIL=1 | FAIL<br>code=1<br>1252ms<br>TFAIL=1 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=1; glibc: wrapper FAIL code=1; glibc: TFAIL=1 |
| `setpriority03` | FAIL<br>code=-1<br>5ms<br>missing-testcase | FAIL<br>code=-1<br>5ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/setpriority03, /musl/ltp/testcases/bin/setpriority03; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `getrusage02` | PASS<br>code=0<br>1018ms<br>TCONF=1 | PASS<br>code=0<br>1285ms<br>TCONF=1 | PASS_WITH_TCONF_NOT_PROMOTABLE | musl: TCONF=1; glibc: TCONF=1 |
| `getrusage03` | FAIL<br>code=2<br>1025ms<br>TBROK=1 | FAIL<br>code=2<br>1284ms<br>TBROK=1 | BLOCKED_RV | musl: wrapper FAIL code=2; musl: TBROK=1; glibc: wrapper FAIL code=2; glibc: TBROK=1 |
| `gettimeofday02` | FAIL<br>code=137<br>20192ms<br>timeout=1 | PASS<br>code=0<br>13398ms | PARTIAL_RV_CLEAN_ONE_LIBC | musl: wrapper FAIL code=137; musl: timeout=1 |
| `gettimeofday03` | FAIL<br>code=-1<br>38ms<br>missing-testcase | FAIL<br>code=-1<br>16ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/gettimeofday03, /musl/ltp/testcases/bin/gettimeofday03; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `setrlimit01` | FAIL<br>code=1<br>1358ms<br>TFAIL=1 | FAIL<br>code=1<br>2766ms<br>TFAIL=1 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=1; glibc: wrapper FAIL code=1; glibc: TFAIL=1 |
| `setrlimit03` | FAIL<br>code=1<br>1085ms<br>TFAIL=1 | FAIL<br>code=1<br>1994ms<br>TFAIL=1 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=1; glibc: wrapper FAIL code=1; glibc: TFAIL=1 |
| `prlimit01` | FAIL<br>code=-1<br>8ms<br>missing-testcase | FAIL<br>code=-1<br>24ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/prlimit01, /musl/ltp/testcases/bin/prlimit01; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `prlimit02` | FAIL<br>code=-1<br>3ms<br>missing-testcase | FAIL<br>code=-1<br>4ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/prlimit02, /musl/ltp/testcases/bin/prlimit02; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `waitpid01` | FAIL<br>code=1<br>2493ms<br>TFAIL=40 | FAIL<br>code=1<br>3524ms<br>TFAIL=32 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=40; glibc: wrapper FAIL code=1; glibc: TFAIL=32 |
| `pipe02` | FAIL<br>code=3<br>969ms<br>TFAIL=1;TBROK=1 | PASS<br>code=0<br>2567ms | PARTIAL_RV_CLEAN_ONE_LIBC | musl: wrapper FAIL code=3; musl: TFAIL=1; musl: TBROK=1 |
| `pipe07` | FAIL<br>code=2<br>855ms<br>TBROK=1 | FAIL<br>code=2<br>2282ms<br>TBROK=1 | BLOCKED_RV | musl: wrapper FAIL code=2; musl: TBROK=1; glibc: wrapper FAIL code=2; glibc: TBROK=1 |
| `pipe08` | FAIL<br>code=1<br>935ms<br>TFAIL=1 | PASS<br>code=0<br>1955ms | PARTIAL_RV_CLEAN_ONE_LIBC | musl: wrapper FAIL code=1; musl: TFAIL=1 |
| `pipe11` | FAIL<br>code=137<br>20219ms<br>timeout=1 | FAIL<br>code=137<br>20586ms<br>timeout=1 | BLOCKED_RV | musl: wrapper FAIL code=137; musl: timeout=1; glibc: wrapper FAIL code=137; glibc: timeout=1 |
| `fcntl05` | FAIL<br>code=1<br>933ms<br>TFAIL=1 | FAIL<br>code=1<br>1955ms<br>TFAIL=1 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=1; glibc: wrapper FAIL code=1; glibc: TFAIL=1 |
| `fcntl06` | FAIL<br>code=-1<br>8ms<br>missing-testcase | FAIL<br>code=-1<br>8ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/fcntl06, /musl/ltp/testcases/bin/fcntl06; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `fcntl07` | FAIL<br>code=2<br>905ms<br>TBROK=2;ENOSYS=1 | FAIL<br>code=2<br>1507ms<br>TBROK=2;ENOSYS=1 | BLOCKED_RV | musl: wrapper FAIL code=2; musl: TBROK=2; musl: ENOSYS=1; glibc: wrapper FAIL code=2; glibc: TBROK=2; glibc: ENOSYS=1 |
| `fcntl11` | FAIL<br>code=1<br>924ms<br>TFAIL=75 | FAIL<br>code=1<br>2376ms<br>TFAIL=75 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=75; glibc: wrapper FAIL code=1; glibc: TFAIL=75 |
| `fcntl12` | FAIL<br>code=5<br>1060ms<br>TFAIL=1 | FAIL<br>code=5<br>2112ms<br>TFAIL=1 | BLOCKED_RV | musl: wrapper FAIL code=5; musl: TFAIL=1; glibc: wrapper FAIL code=5; glibc: TFAIL=1 |
| `fcntl13` | FAIL<br>code=1<br>1002ms<br>TFAIL=3 | FAIL<br>code=1<br>1212ms<br>TFAIL=3 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=3; glibc: wrapper FAIL code=1; glibc: TFAIL=3 |
| `fcntl14` | FAIL<br>code=1<br>11170ms<br>TFAIL=361 | FAIL<br>code=1<br>14167ms<br>TFAIL=361 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=361; glibc: wrapper FAIL code=1; glibc: TFAIL=361 |
| `fcntl15` | FAIL<br>code=1<br>1344ms<br>TFAIL=7 | FAIL<br>code=1<br>2993ms<br>TFAIL=7 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=7; glibc: wrapper FAIL code=1; glibc: TFAIL=7 |
| `fcntl17` | FAIL<br>code=1<br>1106ms<br>TFAIL=7 | FAIL<br>code=1<br>3612ms<br>TFAIL=6 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=7; glibc: wrapper FAIL code=1; glibc: TFAIL=6 |
| `fcntl18` | FAIL<br>code=1<br>997ms<br>TFAIL=4 | FAIL<br>code=1<br>2616ms<br>TFAIL=4 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=4; glibc: wrapper FAIL code=1; glibc: TFAIL=4 |
| `fcntl19` | FAIL<br>code=1<br>961ms<br>TFAIL=37 | FAIL<br>code=1<br>3363ms<br>TFAIL=37 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=37; glibc: wrapper FAIL code=1; glibc: TFAIL=37 |
| `fcntl20` | FAIL<br>code=1<br>1083ms<br>TFAIL=45 | FAIL<br>code=1<br>3117ms<br>TFAIL=45 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=45; glibc: wrapper FAIL code=1; glibc: TFAIL=45 |
| `fcntl21` | FAIL<br>code=1<br>1081ms<br>TFAIL=81 | FAIL<br>code=1<br>6272ms<br>TFAIL=81 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=81; glibc: wrapper FAIL code=1; glibc: TFAIL=81 |
| `open05` | FAIL<br>code=-1<br>8ms<br>missing-testcase | FAIL<br>code=-1<br>22ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/open05, /musl/ltp/testcases/bin/open05; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `open06` | FAIL<br>code=2<br>950ms<br>TBROK=1;ENOSYS=1 | FAIL<br>code=2<br>3062ms<br>TBROK=1;ENOSYS=1 | BLOCKED_RV | musl: wrapper FAIL code=2; musl: TBROK=1; musl: ENOSYS=1; glibc: wrapper FAIL code=2; glibc: TBROK=1; glibc: ENOSYS=1 |
| `openat02` | FAIL<br>code=2<br>1017ms<br>TBROK=2 | FAIL<br>code=2<br>3273ms<br>TBROK=2 | BLOCKED_RV | musl: wrapper FAIL code=2; musl: TBROK=2; glibc: wrapper FAIL code=2; glibc: TBROK=2 |
| `close08` | FAIL<br>code=-1<br>7ms<br>missing-testcase | FAIL<br>code=-1<br>13ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/close08, /musl/ltp/testcases/bin/close08; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `close09` | FAIL<br>code=-1<br>3ms<br>missing-testcase | FAIL<br>code=-1<br>10ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/close09, /musl/ltp/testcases/bin/close09; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `lseek02` | FAIL<br>code=2<br>1008ms<br>TBROK=1;ENOSYS=1 | FAIL<br>code=2<br>3033ms<br>TBROK=1;ENOSYS=1 | BLOCKED_RV | musl: wrapper FAIL code=2; musl: TBROK=1; musl: ENOSYS=1; glibc: wrapper FAIL code=2; glibc: TBROK=1; glibc: ENOSYS=1 |
| `lseek03` | FAIL<br>code=-1<br>19ms<br>missing-testcase | FAIL<br>code=-1<br>37ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/lseek03, /musl/ltp/testcases/bin/lseek03; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `link01` | FAIL<br>code=-1<br>4ms<br>missing-testcase | FAIL<br>code=-1<br>10ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/link01, /musl/ltp/testcases/bin/link01; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `link02` | FAIL<br>code=1<br>1011ms<br>TFAIL=1;ENOSYS=1 | FAIL<br>code=1<br>6343ms<br>TFAIL=1;ENOSYS=1 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=1; musl: ENOSYS=1; glibc: wrapper FAIL code=1; glibc: TFAIL=1; glibc: ENOSYS=1 |
| `link03` | FAIL<br>code=-1<br>7ms<br>missing-testcase | FAIL<br>code=-1<br>25ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/link03, /musl/ltp/testcases/bin/link03; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `linkat01` | FAIL<br>code=2<br>1106ms<br>TBROK=2 | FAIL<br>code=2<br>7028ms<br>TBROK=2 | BLOCKED_RV | musl: wrapper FAIL code=2; musl: TBROK=2; glibc: wrapper FAIL code=2; glibc: TBROK=2 |
| `linkat02` | FAIL<br>code=6<br>938ms<br>TBROK=2 | FAIL<br>code=6<br>5001ms<br>TBROK=2 | BLOCKED_RV | musl: wrapper FAIL code=6; musl: TBROK=2; glibc: wrapper FAIL code=6; glibc: TBROK=2 |
| `rename01` | FAIL<br>code=6<br>1003ms<br>TBROK=1 | FAIL<br>code=6<br>2758ms<br>TBROK=1 | BLOCKED_RV | musl: wrapper FAIL code=6; musl: TBROK=1; glibc: wrapper FAIL code=6; glibc: TBROK=1 |
| `rename02` | FAIL<br>code=-1<br>8ms<br>missing-testcase | FAIL<br>code=-1<br>17ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/rename02, /musl/ltp/testcases/bin/rename02; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `renameat01` | FAIL<br>code=6<br>885ms<br>TBROK=2 | FAIL<br>code=6<br>4538ms<br>TBROK=2 | BLOCKED_RV | musl: wrapper FAIL code=6; musl: TBROK=2; glibc: wrapper FAIL code=6; glibc: TBROK=2 |
| `renameat02` | FAIL<br>code=-1<br>8ms<br>missing-testcase | FAIL<br>code=-1<br>52ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/renameat02, /musl/ltp/testcases/bin/renameat02; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `statfs01` | FAIL<br>code=6<br>939ms<br>TBROK=1 | FAIL<br>code=6<br>7770ms<br>TBROK=1 | BLOCKED_RV | musl: wrapper FAIL code=6; musl: TBROK=1; glibc: wrapper FAIL code=6; glibc: TBROK=1 |
| `statfs02` | FAIL<br>code=1<br>1111ms<br>TFAIL=2 | FAIL<br>code=1<br>6078ms<br>TFAIL=1 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=2; glibc: wrapper FAIL code=1; glibc: TFAIL=1 |
| `fstatfs01` | FAIL<br>code=6<br>957ms<br>TBROK=1 | FAIL<br>code=6<br>4610ms<br>TBROK=1 | BLOCKED_RV | musl: wrapper FAIL code=6; musl: TBROK=1; glibc: wrapper FAIL code=6; glibc: TBROK=1 |
| `fstatfs02` | FAIL<br>code=1<br>958ms<br>TFAIL=1 | PASS<br>code=0<br>6547ms | PARTIAL_RV_CLEAN_ONE_LIBC | musl: wrapper FAIL code=1; musl: TFAIL=1 |
| `statvfs01` | FAIL<br>code=6<br>974ms<br>TBROK=1 | FAIL<br>code=6<br>5087ms<br>TBROK=1 | BLOCKED_RV | musl: wrapper FAIL code=6; musl: TBROK=1; glibc: wrapper FAIL code=6; glibc: TBROK=1 |
| `statvfs02` | FAIL<br>code=1<br>952ms<br>TFAIL=1 | FAIL<br>code=1<br>2972ms<br>TFAIL=1 | BLOCKED_RV | musl: wrapper FAIL code=1; musl: TFAIL=1; glibc: wrapper FAIL code=1; glibc: TFAIL=1 |
| `fstatvfs01` | FAIL<br>code=-1<br>15ms<br>missing-testcase | FAIL<br>code=-1<br>24ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/fstatvfs01, /musl/ltp/testcases/bin/fstatvfs01; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `fstatvfs02` | FAIL<br>code=-1<br>1ms<br>missing-testcase | FAIL<br>code=-1<br>3ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/fstatvfs02, /musl/ltp/testcases/bin/fstatvfs02; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `truncate01` | FAIL<br>code=-1<br>3ms<br>missing-testcase | FAIL<br>code=-1<br>11ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/truncate01, /musl/ltp/testcases/bin/truncate01; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `ftruncate02` | FAIL<br>code=-1<br>4ms<br>missing-testcase | FAIL<br>code=-1<br>4ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/ftruncate02, /musl/ltp/testcases/bin/ftruncate02; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |
| `clock_gettime01` | FAIL<br>code=137<br>20241ms<br>timeout=1 | FAIL<br>code=137<br>20929ms<br>timeout=1 | BLOCKED_RV | musl: wrapper FAIL code=137; musl: timeout=1; glibc: wrapper FAIL code=137; glibc: timeout=1 |
| `clock_getres02` | FAIL<br>code=-1<br>11ms<br>missing-testcase | FAIL<br>code=-1<br>64ms<br>missing-testcase | BLOCKED_RV | missing testcase: /glibc/ltp/testcases/bin/clock_getres02, /musl/ltp/testcases/bin/clock_getres02; musl: wrapper FAIL code=-1; glibc: wrapper FAIL code=-1 |


## Stop conditions for leader

- Do not promote from RV-only evidence. The clean subset above still needs LA follow-up and `scripts/ltp_summary.py` confirmation with zero wrapper failures and zero internal TFAIL/TBROK/TCONF/timeout/ENOSYS/panic rows.
- `PASS` with `TCONF` is intentionally kept out of the clean subset.
- Missing testcase rows and incomplete/unknown parser rows are blockers, not skips.
