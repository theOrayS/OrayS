# Milestone 03 stable656 validation

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Head at first scout: `e9a64d35`
Head during post-fix targeted runs: after `3ee6ee06` plus local `metadata.rs` capacity-reporting change

## Stable count before/after

Command:

```bash
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY
```

Result after this checkpoint: `606 606 0`.

## RV targeted scout: mm/futex blockers

Command captured in run meta:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=mmap05,munmap01,mmap10_1,mmap13,vma02,futex_wait03 \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 60m ./run-eval.sh rv
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.derived.sha256`

Parser summary:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 12
Internal TFAIL/TBROK/TCONF: 8 ({'TBROK': 2, 'TFAIL': 2, 'TCONF': 4})
timeout matches: 2
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 0
```

## RV VFS/process scout

Cases: `openat02,openat03,mknod07,mknodat02,rename03,rename04,rename05,nice04,clone04,sched_rr_get_interval03,sched_setaffinity01,setpriority01,signal01,creat07,fsync02,kill10,nice05`.

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rv-vfs-process-scout-20260602T061408Z.derived.sha256`

Parser summary:

```text
PASS LTP CASE: 3
FAIL LTP CASE: 12
Internal TFAIL/TBROK/TCONF: 17 ({'TBROK': 13, 'TFAIL': 2, 'TCONF': 2})
timeout matches: 1
ENOSYS/not implemented matches: 0
panic/trap matches: 1
Promotion candidates: 0
```

Caveat: `kill10` caused panic/trap and early stop before glibc group; no row from this shard is promotion evidence.

## RV mixed safe scout and LA futex confirmation

Mixed RV cases: `fsync02,nice05,mincore03,shmat1,futex_wait01,futex_wait05`.

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-g009-mixed-safe-scout-20260602T061659Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mixed-safe-scout-20260602T061659Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-g009-mixed-safe-scout-20260602T061659Z.summary.json`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-futex-wait01-confirm-20260602T062001Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-futex-wait01-confirm-20260602T062001Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-futex-wait01-confirm-20260602T062001Z.summary.json`

RV parser summary:

```text
PASS LTP CASE: 3
FAIL LTP CASE: 8
Internal TFAIL/TBROK/TCONF: 11 ({'TBROK': 5, 'TFAIL': 6})
timeout matches: 1
Promotion candidates: 1 (`futex_wait01` on RV)
```

LA `futex_wait01` parser summary:

```text
PASS LTP CASE: 2
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 1
```

Caveat: the RV mixed scout command was terminated with exit code 143 after `shmat1` ran long/hung. It also contains a pre-fix `fsync02` failure. Only completed parser-clean rows are usable, and a later RV isolated futex run is used for the clean current combined candidate pool.

## RV isolated `futex_wait01` proof

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait01 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-futex-wait01-isolated-standalone-20260601T230253Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-futex-wait01-isolated-standalone-20260601T230253Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/rv-futex-wait01-isolated-standalone-20260601T230253Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-futex-wait01-isolated-standalone-20260601T230253Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rv-futex-wait01-isolated-standalone-20260601T230253Z.derived.sha256`

Parser summary:

```text
PASS LTP CASE: 2
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 1
```

## RV divergence scout and LA readlink confirmation

RV cases: `readlinkat02,atof01,fptest01,fptest02,epoll_create02,diotest4,select02,execve05`.

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-divergence-highyield-scout-20260602T062139Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-divergence-highyield-scout-20260602T062139Z.summary.txt`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-confirm-20260602T062321Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-confirm-20260602T062321Z.summary.txt`

RV parser summary:

```text
PASS LTP CASE: 8
FAIL LTP CASE: 8
Internal TFAIL/TBROK/TCONF: 28 ({'TFAIL': 14, 'TCONF': 12, 'TBROK': 2})
ENOSYS/not implemented matches: 2
Promotion candidates: 1 (`readlinkat02` on RV)
```

LA `readlinkat02` parser summary:

```text
PASS LTP CASE: 1
FAIL LTP CASE: 1
Internal TFAIL/TBROK/TCONF: 1 ({'TFAIL': 1})
Promotion candidates: 0
```

Decision: `readlinkat02` is blocked by LA musl `TFAIL`.

## LA `readlinkat02` rerun after code inspection

Command captured in run meta:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=readlinkat02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-rerun-20260601T223953Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-rerun-20260601T223953Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-rerun-20260601T223953Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-rerun-20260601T223953Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/la-readlinkat02-rerun-20260601T223953Z.derived.sha256`

Parser summary:

```text
PASS LTP CASE: 1
FAIL LTP CASE: 1
Internal TFAIL/TBROK/TCONF: 1 ({'TFAIL': 1})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 0
```

Decision: unchanged blocker. LA glibc is clean, LA musl still fails the zero-size readlink boundary, and this case cannot be promoted.

## RV `fsync02` pre-fix isolated rerun

Command captured in run meta:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fsync02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rv-fsync02-isolated-20260601T224426Z.derived.sha256`

Parser summary:

```text
PASS LTP CASE: 1
FAIL LTP CASE: 1
Internal TFAIL/TBROK/TCONF: 1 ({'TBROK': 1})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 0
```

Decision: this old run remains failed evidence and is not counted. It motivated the `generic_statfs` capacity-reporting inspection.

## `fsync02` post-fix statfs-capacity proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fsync02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fsync02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.summary.json`
- LA promotion report: `target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.promotion-candidates.txt`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 2
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 1
```

Decision: `fsync02` is now four-way clean and enters the future candidate pool.

## Adjacent statfs/fstatfs/statvfs regression subset

Cases: `statfs02,fstatfs02,fstatfs02_64,statfs02_64,statfs03,statfs03_64,statvfs02`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs02,fstatfs02,fstatfs02_64,statfs02_64,statfs03,statfs03_64,statvfs02 LTP_CASE_TIMEOUT_SECS=90 timeout 40m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs02,fstatfs02,fstatfs02_64,statfs02_64,statfs03,statfs03_64,statvfs02 LTP_CASE_TIMEOUT_SECS=90 timeout 40m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-statfs-regression-statfs-clamp-20260601T230028Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-statfs-regression-statfs-clamp-20260601T230028Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-statfs-regression-statfs-clamp-20260601T230028Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-statfs-regression-statfs-clamp-20260601T230028Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-statfs-regression-statfs-clamp-20260601T230028Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-statfs-regression-statfs-clamp-20260601T230122Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-statfs-regression-statfs-clamp-20260601T230122Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-statfs-regression-statfs-clamp-20260601T230122Z.summary.json`
- LA promotion report: `target/ltp-1000-milestone-03-stable656/la-statfs-regression-statfs-clamp-20260601T230122Z.promotion-candidates.txt`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-statfs-regression-statfs-clamp-20260601T230122Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 14
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Decision: the stable statfs/fstatfs/statvfs adjacent subset did not regress.

## `sched_setaffinity01` targeted fix proof

Commands captured in run meta:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_setaffinity01 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_setaffinity01 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-sched-setaffinity01-postfix-20260601T222738Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-sched-setaffinity01-postfix-20260601T222738Z.summary.txt`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-sched-setaffinity01-postfix-20260601T222823Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-sched-setaffinity01-postfix-20260601T222823Z.summary.txt`

Parser result on each arch:

```text
PASS LTP CASE: 2
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 1
```

## Adjacent scheduler regression subset

Cases: `sched_getaffinity01,sched_setparam01,sched_setparam02,sched_setparam03,sched_setparam04,sched_setparam05,sched_setscheduler01,sched_setscheduler02,sched_setscheduler03,setpriority02`.

Commands captured in run meta:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_getaffinity01,sched_setparam01,sched_setparam02,sched_setparam03,sched_setparam04,sched_setparam05,sched_setscheduler01,sched_setscheduler02,sched_setscheduler03,setpriority02 LTP_CASE_TIMEOUT_SECS=90 timeout 40m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_getaffinity01,sched_setparam01,sched_setparam02,sched_setparam03,sched_setparam04,sched_setparam05,sched_setscheduler01,sched_setscheduler02,sched_setscheduler03,setpriority02 LTP_CASE_TIMEOUT_SECS=90 timeout 40m ./run-eval.sh la
```

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-sched-affinity-regression-20260601T222920Z.summary.txt`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-sched-affinity-regression-20260601T223023Z.summary.txt`

Parser result on each arch:

```text
PASS LTP CASE: 20
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

## Combined candidate pool

Command:

```bash
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc \
  target/ltp-1000-milestone-03-stable656/rv-futex-wait01-isolated-standalone-20260601T230253Z.log \
  target/ltp-1000-milestone-03-stable656/la-futex-wait01-confirm-20260602T062001Z.log \
  target/ltp-1000-milestone-03-stable656/rv-sched-setaffinity01-postfix-20260601T222738Z.log \
  target/ltp-1000-milestone-03-stable656/la-sched-setaffinity01-postfix-20260601T222823Z.log \
  target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.log \
  target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.log
```

Artifacts:

- Promotion report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean3-20260601T230334Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean3-20260601T230334Z.derived.sha256`

Parser result:

```text
Required arches: la, rv
Required libcs: glibc, musl
Promotion candidates: 3
Blocked/incomplete cases: 0
Candidates: fsync02, futex_wait01, sched_setaffinity01
```

## Closed arch full-sweep mining against live stable606

Command shape:

```bash
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc \
  target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/rv-arch002.log \
  target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch012.log
```

Artifacts:

- Candidate report: `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.promotion-candidates.txt`
- Not-stable filter: `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.not-stable.txt`
- RV matrix JSON: `target/ltp-1000-milestone-03-stable656/rv-arch002-full-matrix-20260601T224223Z.json`
- LA matrix JSON: `target/ltp-1000-milestone-03-stable656/la-arch012-full-matrix-20260601T224223Z.json`
- Checksums: `target/ltp-1000-milestone-03-stable656/arch-sweep-rv002-la012-not-stable606-20260601T224223Z.derived.sha256`

Parser/mining summary:

```text
Raw arch-sweep four-way clean candidates: 563
Four-way clean candidates not already in live stable606: 0
RV matrix: PASS 1204, FAIL 3453, internal {'TBROK': 1043, 'TCONF': 2663, 'TFAIL': 4058}, timeout 55, ENOSYS 1280, panic/trap 0
LA matrix: PASS 1207, FAIL 2698, internal {'TBROK': 1031, 'TCONF': 1936, 'TFAIL': 4041}, timeout 53, ENOSYS 1279, panic/trap 0
```

Decision: the closed arch sweep is exhausted for immediate post-stable606 promotion. Its remaining value is blocker triage, not stable656 counting.

## `nice04` source/errno-boundary note

Source inspected: `/root/oskernel2026-orays-clean-stable520/docs/ltp-score-improvement-2026-05-28-phase-b/raw/ltp-source/nice_nice04.c`.

Observed requirement: after switching to user `nobody`, `nice(-10)` expects failure with `errno == EPERM`. Current kernel `setpriority` lowering path returns Linux `EACCES` semantics for non-root callers that try to lower nice, and stable `setpriority02` explicitly protects that syscall-level behavior. Detailed report: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-03-stable656/nice04-errno-boundary-report.md`. Therefore `nice04` is not changed in this checkpoint; it remains a libc-wrapper/errno-boundary investigation rather than a safe kernel errno flip.

## Gate outcome

- Targeted RV: clean for `fsync02`, `futex_wait01`, and `sched_setaffinity01`; other scout rows blocked as documented.
- Adjacent stable regression subset: clean on RV and LA for the scheduler permission fix and the statfs capacity clamp.
- LA confirmation: clean for `fsync02`, `futex_wait01`, and `sched_setaffinity01`; blocked for `readlinkat02` due LA musl `TFAIL`.
- musl + glibc: clean only for the three candidate rows.
- Parser blockers: still present in scout rows; they are not counted.
- Stable list: unchanged at `606/606/0`.

## Unverified items

- No stable656 promotion gate because the candidate pool has only 3/50 required new cases.
- No new broad all-minus-blacklist sweep in this checkpoint; only closed arch-sweep logs were re-mined, yielding zero non-stable four-way-clean rows.
- No fixes yet for `kill10`, `mmap05`, `munmap01`, `mmap13`, `futex_wait03`, `futex_wait05`, `shmat1`, `nice04`, or LA musl `readlinkat02`.
