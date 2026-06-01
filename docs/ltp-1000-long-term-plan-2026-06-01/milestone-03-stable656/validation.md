# Milestone 03 stable656 validation

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Head at first scout: `e9a64d35`
Head during post-fix targeted runs: `840e4a3b` plus local `resource_sched.rs` change

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

Caveat: the RV mixed scout command was terminated with exit code 143 after `shmat1` ran long/hung; only completed parser-clean rows are usable as scout evidence.

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
  target/ltp-1000-milestone-03-stable656/rv-g009-mixed-safe-scout-20260602T061659Z.log \
  target/ltp-1000-milestone-03-stable656/la-futex-wait01-confirm-20260602T062001Z.log \
  target/ltp-1000-milestone-03-stable656/rv-sched-setaffinity01-postfix-20260601T222738Z.log \
  target/ltp-1000-milestone-03-stable656/la-sched-setaffinity01-postfix-20260601T222823Z.log \
  target/ltp-1000-milestone-03-stable656/rv-divergence-highyield-scout-20260602T062139Z.log \
  target/ltp-1000-milestone-03-stable656/la-readlinkat02-confirm-20260602T062321Z.log
```

Artifact:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-20260601T223023Z.promotion-candidates.txt`

Parser result:

```text
Required arches: la, rv
Required libcs: glibc, musl
Promotion candidates: 2
Blocked/incomplete cases: 13
Candidates: futex_wait01, sched_setaffinity01
```

## Gate outcome

- Targeted RV: clean for `futex_wait01` and `sched_setaffinity01`; other scout rows blocked as documented.
- Adjacent stable regression subset: clean on RV and LA for the scheduler permission fix.
- LA confirmation: clean for `futex_wait01` and `sched_setaffinity01`; blocked for `readlinkat02` due LA musl `TFAIL`.
- musl + glibc: clean only for the two candidate rows.
- Parser blockers: still present in scout rows; they are not counted.
- Stable list: unchanged at `606/606/0`.

## Unverified items

- No stable656 promotion gate because the candidate pool has only 2/50 required new cases.
- No broad all-minus-blacklist sweep in this checkpoint.
- No fixes yet for `kill10`, `mmap05`, `munmap01`, `mmap13`, `futex_wait03`, `shmat1`, or LA musl `readlinkat02`.
