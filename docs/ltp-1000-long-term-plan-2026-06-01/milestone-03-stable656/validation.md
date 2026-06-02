# Milestone 03 stable656 validation

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Head at first scout: `e9a64d35`
Head during post-fix targeted runs: after `3ee6ee06` plus local `metadata.rs` capacity-reporting, `synthetic_fs.rs` proc-state, and timer-list/periodic-deadline changes

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

## RV `openat02` post-statfs-clamp scout

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=openat02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rv-openat02-post-statfs-scout-20260601T231156Z.derived.sha256`

Parser summary:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 2
Internal TFAIL/TBROK/TCONF: 4 ({'TBROK': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 0
```

Decision: the generic statfs capacity clamp that fixed `fsync02` is not sufficient for `openat02`; both RV musl and RV glibc still hit setup `write(...,7) failed: errno=ENOSPC(28)`. `openat02` remains blocked and is not eligible for LA rerun or promotion accounting until its file-growth/space accounting path is diagnosed separately.

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

## `futex_wait03` procfs sleeping-state proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait03 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait03 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-futex-wait03-proc-sleep-20260601T232011Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-futex-wait03-proc-sleep-20260601T232011Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-futex-wait03-proc-sleep-20260601T232011Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-futex-wait03-proc-sleep-20260601T232011Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-futex-wait03-proc-sleep-20260601T232011Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-futex-wait03-proc-sleep-20260601T232052Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-futex-wait03-proc-sleep-20260601T232052Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-futex-wait03-proc-sleep-20260601T232052Z.summary.json`
- LA promotion report: `target/ltp-1000-milestone-03-stable656/la-futex-wait03-proc-sleep-20260601T232052Z.promotion-candidates.txt`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-futex-wait03-proc-sleep-20260601T232052Z.derived.sha256`

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

Decision: `futex_wait03` is now four-way clean after the generic synthetic `/proc/<pid>/stat` sleeping-state repair and enters the future candidate pool.

## Adjacent futex/proc regression subset

Cases: `futex_wait02,futex_wait04,futex_wake01,proc01,waitpid04`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait02,futex_wait04,futex_wake01,proc01,waitpid04 LTP_CASE_TIMEOUT_SECS=90 timeout 40m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait02,futex_wait04,futex_wake01,proc01,waitpid04 LTP_CASE_TIMEOUT_SECS=90 timeout 40m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-futex-proc-regression-20260601T232144Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-futex-proc-regression-20260601T232144Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-futex-proc-regression-20260601T232144Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-futex-proc-regression-20260601T232144Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-futex-proc-regression-20260601T232144Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-futex-proc-regression-20260601T232232Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-futex-proc-regression-20260601T232232Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-futex-proc-regression-20260601T232232Z.summary.json`
- LA promotion report: `target/ltp-1000-milestone-03-stable656/la-futex-proc-regression-20260601T232232Z.promotion-candidates.txt`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-futex-proc-regression-20260601T232232Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 10
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Decision: the adjacent futex/proc/wait stable subset did not regress.

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

### LA `readlinkat02` root-cause audit

Additional source audit on 2026-06-02:

- Kernel side: `examples/shell/src/uspace/metadata.rs::sys_readlinkat` already returns `EINVAL` when the syscall-visible `bufsiz == 0`.
- LA syscall argument side: `vendor/axcpu/src/loongarch64/context.rs` maps syscall arguments through `a0..a3`, so this is not a generic fourth-argument loss.
- Musl side: upstream `src/unistd/readlinkat.c` (`https://git.musl-libc.org/cgit/musl/plain/src/unistd/readlinkat.c`) rewrites user `bufsize == 0` to a stack dummy buffer with syscall `bufsize = 1`, then converts a positive return back to `0`.

The failing LA musl row is therefore not a safe kernel fix: the kernel cannot distinguish musl's dummy one-byte syscall from a valid direct `readlinkat(..., bufsiz=1)` call, and Linux permits truncation to a one-byte user buffer. Keep `readlinkat02` blocked/non-promotable unless the libc/test boundary changes; do not add a case-specific or `bufsiz=1` kernel special case.

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

## Combined candidate pool before `futex_wait05`

Command:

```bash
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc \
  target/ltp-1000-milestone-03-stable656/rv-futex-wait01-isolated-standalone-20260601T230253Z.log \
  target/ltp-1000-milestone-03-stable656/la-futex-wait01-confirm-20260602T062001Z.log \
  target/ltp-1000-milestone-03-stable656/rv-sched-setaffinity01-postfix-20260601T222738Z.log \
  target/ltp-1000-milestone-03-stable656/la-sched-setaffinity01-postfix-20260601T222823Z.log \
  target/ltp-1000-milestone-03-stable656/rv-fsync02-statfs-clamp-20260601T225748Z.log \
  target/ltp-1000-milestone-03-stable656/la-fsync02-statfs-clamp-20260601T225836Z.log \
  target/ltp-1000-milestone-03-stable656/rv-futex-wait03-proc-sleep-20260601T232011Z.log \
  target/ltp-1000-milestone-03-stable656/la-futex-wait03-proc-sleep-20260601T232052Z.log
```

Artifacts:

- Promotion report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean4-20260601T232334Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean4-20260601T232334Z.derived.sha256`

Parser result:

```text
Required arches: la, rv
Required libcs: glibc, musl
Promotion candidates: 4
Blocked/incomplete cases: 0
Candidates: fsync02, futex_wait01, futex_wait03, sched_setaffinity01
```

Decision: this clean4 report is historical after the later `futex_wait05` update; the current clean5 proof is recorded below.

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

## RV `clone04` singleton rescout

Command captured in run meta:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=clone04 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rv-clone04-singleton-20260602T001435Z.derived.sha256`

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

Decision: `clone04` remains blocked. RV glibc is clean (`NULL stack : EINVAL (22)`), but RV musl is killed by SIGSEGV/TBROK. The raw log's LTP hint points to a musl `clone.c` fix (`https://git.musl-libc.org/cgit/musl/commit/src/linux/clone.c?id=fa4a8abd06a4`), so treat this as a libc-wrapper boundary until proven otherwise; do not promote or run LA confirmation from this failed RV gate.

## Gate outcome

- Targeted RV: clean for `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, and `sched_setaffinity01`; other scout rows remain blocked, including `clone04` due RV musl TBROK/SIGSEGV.
- Adjacent stable regression subset: clean on RV and LA for the scheduler permission fix, statfs capacity clamp, procfs futex-sleeping state repair, and precise timer-list wakeup repair.
- LA confirmation: clean for `fsync02`, `futex_wait01`, `futex_wait03`, `futex_wait05`, and `sched_setaffinity01`; blocked for `readlinkat02` due LA musl `TFAIL` from the audited libc/test zero-size wrapper boundary.
- musl + glibc: clean only for the five candidate rows.
- Parser blockers: still present in scout rows; they are not counted.
- Stable list: unchanged at `606/606/0`.

## Unverified items

- At the earlier sync-SIGSEGV checkpoint there was still no stable656 promotion gate because the candidate pool had only 6/50 required new cases; the later `mmap13` section below supersedes this count with 7/50.
- No new broad all-minus-blacklist sweep in this checkpoint; only closed arch-sweep logs were re-mined, yielding zero non-stable four-way-clean rows.
- At that checkpoint there were no fixes yet for `kill10`, LA `mmap05`, `mmap13`, `shmat1`, `nice04`, or `clone04`; the later `mmap13` section below supersedes that blocker, while LA musl `readlinkat02` remains documented as non-promotable from the kernel side unless the libc/test boundary changes.


## `futex_wait05` precise timer-list proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait05 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait05 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-futex-wait05-periodic-fix-20260601T235234Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-futex-wait05-periodic-fix-20260601T235234Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-futex-wait05-periodic-fix-20260601T235234Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-futex-wait05-periodic-fix-20260601T235234Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-futex-wait05-periodic-fix-20260601T235234Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-futex-wait05-periodic-fix-20260601T235323Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-futex-wait05-periodic-fix-20260601T235323Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-futex-wait05-periodic-fix-20260601T235323Z.summary.json`
- LA promotion report: `target/ltp-1000-milestone-03-stable656/la-futex-wait05-periodic-fix-20260601T235323Z.promotion-candidates.txt`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-futex-wait05-periodic-fix-20260601T235323Z.derived.sha256`

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

Decision: `futex_wait05` is now four-way clean after the generic timer-list precise wakeup and periodic-deadline preservation repair.

## Adjacent timer/futex regression subset

Cases: `futex_wait01,futex_wait02,futex_wait03,futex_wait04,futex_wait05,futex_wake01,proc01,waitpid04,nanosleep01,clock_nanosleep02`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait01,futex_wait02,futex_wait03,futex_wait04,futex_wait05,futex_wake01,proc01,waitpid04,nanosleep01,clock_nanosleep02 LTP_CASE_TIMEOUT_SECS=90 timeout 50m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=futex_wait01,futex_wait02,futex_wait03,futex_wait04,futex_wait05,futex_wake01,proc01,waitpid04,nanosleep01,clock_nanosleep02 LTP_CASE_TIMEOUT_SECS=90 timeout 50m ./run-eval.sh la
```

Artifacts:

- RV summary: `target/ltp-1000-milestone-03-stable656/rv-timer-futex-regression-periodic-fix-20260601T235036Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-timer-futex-regression-periodic-fix-20260601T235036Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-timer-futex-regression-periodic-fix-20260601T235036Z.derived.sha256`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-timer-futex-regression-periodic-fix-20260601T234827Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-timer-futex-regression-periodic-fix-20260601T234827Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-timer-futex-regression-periodic-fix-20260601T234827Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 20
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Non-countable repair history:

- `target/ltp-1000-milestone-03-stable656/la-timer-futex-regression-20260601T234109Z.log` was terminated with exit code 143 before LTP cases completed because the TTY-launched QEMU process stopped; it is not evidence.
- `target/ltp-1000-milestone-03-stable656/la-timer-futex-regression-20260601T234340Z.log` was terminated with exit code 143 after hanging in pre-fix `futex_wait05`; it exposed the periodic-deadline drift and is not promotion evidence.

## Historical combined clean candidate pool after `futex_wait05`

Artifact:

- `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean5-periodic-fix-20260601T235428Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean5-periodic-fix-20260601T235428Z.derived.sha256`

Parser promotion report:

```text
Promotion candidates: 5
Blocked/incomplete cases: 0
Candidates: fsync02, futex_wait01, futex_wait03, futex_wait05, sched_setaffinity01
```

## Gate outcome after sync-SIGSEGV update

- Live stable list remains `606 total / 606 unique / 0 duplicate`.
- At this sync-SIGSEGV checkpoint the clean candidate pool was 6/50 for stable656.
- No `LTP_STABLE_CASES` edit was made at that point because 44 more four-way-clean unique cases were still required; the later `mmap13` section below updates the pool to 7/50.
- Counted targeted and regression summaries for `futex_wait05`, `munmap01`, timer/futex, and mmap/signal adjacency are parser-clean with zero `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`; `mmap05` remains blocked on LA `TFAIL`.


## `mmap05,munmap01` catchable synchronous `SIGSEGV` proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap05,munmap01 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap05,munmap01 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-mmap05-munmap01-sync-sigsegv-20260602T002516Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.summary.json`
- LA promotion report: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.promotion-candidates.txt`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-sync-sigsegv-20260602T002606Z.derived.sha256`

RV parser summary:

```text
PASS LTP CASE: 4
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 2 (`mmap05`, `munmap01` on RV only)
```

LA parser summary:

```text
PASS LTP CASE: 2
FAIL LTP CASE: 2
Internal TFAIL/TBROK/TCONF: 2 ({'TFAIL': 2})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Promotion candidates: 1 (`munmap01`)
Blocked: `mmap05` LA musl+glibc `TFAIL=1` / SIGSEGV signal not received
```

Decision: `munmap01` is four-way parser-clean and enters the future candidate pool. `mmap05` remains blocked on LA and is not counted.

## Adjacent stable regression for catchable synchronous `SIGSEGV`

Regression cases: `mmap01,mmap02,mmap03,mmap04,mmap09,mmap12,signal03,sigaction01,rt_sigaction01,rt_sigprocmask01,sigprocmask01,waitpid04`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap01,mmap02,mmap03,mmap04,mmap09,mmap12,signal03,sigaction01,rt_sigaction01,rt_sigprocmask01,sigprocmask01,waitpid04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap01,mmap02,mmap03,mmap04,mmap09,mmap12,signal03,sigaction01,rt_sigaction01,rt_sigprocmask01,sigprocmask01,waitpid04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-sync-sigsegv-regression-20260602T002800Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-sync-sigsegv-regression-20260602T002800Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-sync-sigsegv-regression-20260602T002800Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-sync-sigsegv-regression-20260602T002800Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-sync-sigsegv-regression-20260602T003046Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-sync-sigsegv-regression-20260602T003046Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-sync-sigsegv-regression-20260602T003046Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-sync-sigsegv-regression-20260602T003046Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 24
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

## Combined clean6 candidate pool

Combined report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean6-sync-sigsegv-20260602T003243Z.promotion-candidates.txt`
Checksum: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean6-sync-sigsegv-20260602T003243Z.promotion-candidates.sha256`

```text
Promotion candidates: 6
Candidates: fsync02, futex_wait01, futex_wait03, futex_wait05, munmap01, sched_setaffinity01
Blocked/incomplete: mmap05 (LA musl+glibc TFAIL=1)
```


## LA `mmap05` write-protect/TLB experiment (non-promotable)

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap05,munmap01 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap05 LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh la
```

Artifacts:

- Flush experiment raw log: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-tlbflush-20260602T004430Z.log`
- Flush experiment summary: `target/ltp-1000-milestone-03-stable656/la-mmap05-munmap01-tlbflush-20260602T004430Z.summary.txt`
- Debug raw log: `target/ltp-1000-milestone-03-stable656/la-mmap05-debug-20260602T004819Z.log`
- Debug summary: `target/ltp-1000-milestone-03-stable656/la-mmap05-debug-20260602T004819Z.summary.txt`
- Detailed report: `mmap05-la-write-protect-report.md`

Parser result: the flush experiment kept `munmap01` clean (`PASS LTP CASE: 2`) but `mmap05` still failed both LA libcs (`FAIL LTP CASE: 2`, internal `TFAIL=2`). The debug rerun of `mmap05` alone also failed both LA libcs (`FAIL LTP CASE: 2`, internal `TFAIL=2`).

Decision: `mmap05` remains non-promotable. The temporary explicit TLB-flush/debug instrumentation was removed; no production code change is retained from this failed hypothesis.

## `mmap13` file-backed SIGBUS-on-EOF proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap13 LTP_CASE_TIMEOUT_SECS=90 timeout 12m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap13 LTP_CASE_TIMEOUT_SECS=90 timeout 12m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-final-20260602T012111Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-final-20260602T012111Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-final-20260602T012111Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-final-20260602T012111Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-final-20260602T012111Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-final-20260602T012141Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-final-20260602T012141Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-final-20260602T012141Z.summary.json`
- LA promotion report: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-final-20260602T012141Z.promotion-candidates.txt`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-final-20260602T012141Z.derived.sha256`

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

Decision: `mmap13` is now RV + LA x musl + glibc parser-clean after the generic file-backed mmap beyond-EOF `SIGBUS` repair.

Non-countable repair history:

- `target/ltp-1000-milestone-03-stable656/rv-mmap13-current-20260602T005657Z.log` is the pre-fix blocker evidence: RV musl and glibc both had `TFAIL=1` / `SIGBUS signal not received`.
- `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-20260602T010506Z.log` was a TTY-launched local rerun that stopped before guest output and produced no LTP rows; it is not promotion evidence. The counted RV proof is the non-TTY run above.

## Adjacent mmap/SIGBUS regression subset

Cases: `mmap01,mmap02,mmap03,mmap04,mmap09,mmap12,signal03,sigaction01,rt_sigaction01,rt_sigprocmask01,sigprocmask01,waitpid04`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap01,mmap02,mmap03,mmap04,mmap09,mmap12,signal03,sigaction01,rt_sigaction01,rt_sigprocmask01,sigprocmask01,waitpid04 LTP_CASE_TIMEOUT_SECS=90 timeout 18m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap01,mmap02,mmap03,mmap04,mmap09,mmap12,signal03,sigaction01,rt_sigaction01,rt_sigprocmask01,sigprocmask01,waitpid04 LTP_CASE_TIMEOUT_SECS=90 timeout 18m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-regression-20260602T011329Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-regression-20260602T011329Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-regression-20260602T011329Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-mmap13-sigbus-regression-20260602T011329Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-regression-20260602T011433Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-regression-20260602T011433Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-regression-20260602T011433Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-mmap13-sigbus-regression-20260602T011433Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 24
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Decision: adjacent stable mmap/signal/wait regression did not regress on RV or LA.

## Combined clean7 candidate pool

Combined report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean7-mmap13-sigbus-final-20260602T012225Z.promotion-candidates.txt`
Checksum: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean7-mmap13-sigbus-final-20260602T012225Z.promotion-candidates.sha256`

```text
Promotion candidates: 7
Candidates: fsync02, futex_wait01, futex_wait03, futex_wait05, mmap13, munmap01, sched_setaffinity01
Blocked/incomplete: mmap05 (LA musl+glibc TFAIL=1)
```

## Gate outcome after mmap13 SIGBUS update

- Live stable list remains `606 total / 606 unique / 0 duplicate`.
- At this point in the evidence timeline, the clean candidate pool was 7/50 for stable656.
- No `LTP_STABLE_CASES` edit was made at that historical checkpoint because 43 more four-way-clean unique cases were still required; later `openat02`, `signal01`, `mincore03`, and G009 clean4 evidence superseded that pool count to 14/50 before the LTP device/NAME_MAX clean5 update raised it to 19/50.
- Counted targeted and regression summaries for `mmap13` are parser-clean with zero `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`; the pre-fix and TTY-aborted RV logs remain visible as non-countable history.


## `openat02` sparse large-file proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=openat02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=openat02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-openat02-sparse-largefile-20260602T014202Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-openat02-sparse-largefile-20260602T014202Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-openat02-sparse-largefile-20260602T014202Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-openat02-sparse-largefile-20260602T014202Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-openat02-sparse-largefile-20260602T014245Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-openat02-sparse-largefile-20260602T014245Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-openat02-sparse-largefile-20260602T014245Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-openat02-sparse-largefile-20260602T014245Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 2
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Decision: `openat02` is now RV + LA x musl + glibc parser-clean after generic sparse logical-size/data handling for large-file holes. The older `rv-openat02-post-statfs-scout-20260601T231156Z.log` remains pre-fix blocker history and is not counted.

## Adjacent VFS/FD regression for sparse large-file handling

Clean regression cases: `openat01,lseek01,lseek02,pread02,pwrite02,pwrite04,ftruncate01,truncate02,read01,write01,write03`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=openat01,lseek01,lseek02,pread02,pwrite02,pwrite04,ftruncate01,truncate02,read01,write01,write03 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=openat01,lseek01,lseek02,pread02,pwrite02,pwrite04,ftruncate01,truncate02,read01,write01,write03 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-openat02-adjacent-stable-clean-regression-20260602T014443Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-openat02-adjacent-stable-clean-regression-20260602T014545Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-openat02-adjacent-stable-clean-regression-20260602T014545Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-openat02-adjacent-stable-clean-regression-20260602T014545Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-openat02-adjacent-stable-clean-regression-20260602T014545Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 22
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Non-countable observation: `rv-openat02-adjacent-stable-regression-20260602T014338Z.log` (checksums: `target/ltp-1000-milestone-03-stable656/rv-openat02-adjacent-stable-regression-20260602T014338Z.derived.sha256`) also ran `read02`; wrapper rows passed, but `read02` emitted existing O_DIRECT `TCONF=4` across both libcs, so that 12-case shard is retained only as caveated observation and not counted as parser-clean regression evidence.

## Combined clean8 candidate pool

Combined report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean8-openat02-sparse-largefile-20260602T014245Z.promotion-candidates.txt`
Checksum: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean8-openat02-sparse-largefile-20260602T014245Z.promotion-candidates.sha256`

```text
Promotion candidates: 8
Candidates: fsync02, futex_wait01, futex_wait03, futex_wait05, mmap13, munmap01, openat02, sched_setaffinity01
Blocked/incomplete: mmap05 (LA musl+glibc TFAIL=1)
```

## Gate outcome after openat02 sparse-largefile update

- Live stable list remains `606 total / 606 unique / 0 duplicate`.
- At this point in the evidence timeline, the clean candidate pool was 8/50 for stable656.
- No `LTP_STABLE_CASES` edit was made because 42 more four-way-clean unique cases were still required.
- Counted targeted and regression summaries for `openat02` are parser-clean with zero `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`; the pre-fix RV scout remains visible as non-countable history.

## `openat03` O_TMPFILE unsupported-gate blocker

Context: a larger in-memory `O_TMPFILE`/`linkat` implementation was attempted, but RV `openat03` hit a kernel panic/trap during the testcase's deep nested-directory phase. That patch was rejected and removed. The retained source change is a minimal generic `O_TMPFILE` gate in `open_candidates`: `O_TMPFILE|O_RDONLY` returns `EINVAL`, and `O_TMPFILE` against an existing directory returns `EOPNOTSUPP` instead of accidentally handing out a directory fd.

Build command:

```bash
make A=examples/shell ARCH=riscv64
```

Build result: success; both `kernel-rv` and `kernel-la` were produced, with only pre-existing vendor/axnet warnings.

Rejected implementation evidence:

- RV panic summary: `target/ltp-1000-milestone-03-stable656/rv-openat03-otmpfile-20260602T021349Z.summary.txt`
- RV trace panic summary: `target/ltp-1000-milestone-03-stable656/rv-openat03-trace-20260602T022058Z.summary.txt`

Parser result for each rejected RV run:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 1
```

The trace run reached `openat03` test02 nested directory creation (`tst02_49`) before the supervisor page fault, so this is treated as a VFS/deep-directory robustness blocker, not as promotion evidence.

Retained unsupported-gate validation commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=openat03 LTP_CASE_TIMEOUT_SECS=120 timeout 25m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=openat03 LTP_CASE_TIMEOUT_SECS=120 timeout 25m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-openat03-otmpfile-enotsup-20260602T022658Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-openat03-otmpfile-enotsup-20260602T022658Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-openat03-otmpfile-enotsup-20260602T022658Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-openat03-otmpfile-enotsup-20260602T022658Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-openat03-otmpfile-enotsup-20260602T022748Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-openat03-otmpfile-enotsup-20260602T022748Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-openat03-otmpfile-enotsup-20260602T022748Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-openat03-otmpfile-enotsup-20260602T022748Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 2
Internal TFAIL/TBROK/TCONF: 4 ({'TCONF': 4})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Observed LTP marker: `openat03.c:56: O_TMPFILE not supported`; wrapper status remains FAIL code 32 for both musl and glibc on RV and LA. This evidence is intentionally non-promotable because the parser sees `TCONF`, but it closes the safety claim that unsupported `O_TMPFILE` no longer causes panic/trap in the targeted RV/LA runs.

Decision: `openat03` is not added to the candidate pool. At this point in the evidence timeline, the candidate pool remained 8/50 for stable656; after later `signal01`, `mincore03`, and G009 clean4 evidence, the pool was 14/50 before the LTP device/NAME_MAX clean5 update raised it to 19/50. `LTP_STABLE_CASES` remains `606 total / 606 unique / 0 duplicate`.

## `signal01` signal/poll sleeping-state proof

Rejected intermediate evidence:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-signal01-proc-sleep-20260602T024336Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-signal01-proc-sleep-20260602T024336Z.summary.txt`

This run was terminated after the first, `rt_sigsuspend`-only proc-state repair still left RV musl in timeout. It is repair-history evidence only and is not counted toward promotion.

Final targeted commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=signal01 LTP_CASE_TIMEOUT_SECS=120 timeout 25m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=signal01 LTP_CASE_TIMEOUT_SECS=120 timeout 25m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-signal01-poll-wait-20260602T024843Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-signal01-poll-wait-20260602T024843Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-signal01-poll-wait-20260602T024843Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-signal01-poll-wait-20260602T024843Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-signal01-poll-wait-20260602T024926Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-signal01-poll-wait-20260602T024926Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-signal01-poll-wait-20260602T024926Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-signal01-poll-wait-20260602T024926Z.derived.sha256`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean9-signal01-poll-wait-20260602T025432Z.promotion-candidates.txt`

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

Decision: `signal01` is now RV + LA x musl + glibc parser-clean after the generic synthetic `/proc/<pid>/stat` sleeping-state repair covers both `rt_sigsuspend` and libc `pause()`/`ppoll` wait paths. It enters the future candidate pool; stable list remains unchanged until the +50 milestone gate is met.

## Adjacent signal/poll/proc regression subset

Cases: `signal02,signal03,signal04,signal05,sigaction01,rt_sigaction01,sigprocmask01,rt_sigprocmask01,ppoll01,pselect01,poll02,waitpid04`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=signal02,signal03,signal04,signal05,sigaction01,rt_sigaction01,sigprocmask01,rt_sigprocmask01,ppoll01,pselect01,poll02,waitpid04 LTP_CASE_TIMEOUT_SECS=120 timeout 35m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=signal02,signal03,signal04,signal05,sigaction01,rt_sigaction01,sigprocmask01,rt_sigprocmask01,ppoll01,pselect01,poll02,waitpid04 LTP_CASE_TIMEOUT_SECS=120 timeout 35m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-signal-poll-regression-20260602T025025Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-signal-poll-regression-20260602T025025Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-signal-poll-regression-20260602T025025Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-signal-poll-regression-20260602T025025Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-signal-poll-regression-20260602T025204Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-signal-poll-regression-20260602T025204Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-signal-poll-regression-20260602T025204Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-signal-poll-regression-20260602T025204Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 24
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Decision: the adjacent stable signal/poll/proc subset did not regress on RV or LA.

## RV `kill10` isolated severe-blocker confirmation

Date: 2026-06-02.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=kill10 LTP_CASE_TIMEOUT_SECS=120 timeout 25m ./run-eval.sh rv
# after testing and rejecting a temporary poll/exit-group cleanup hypothesis:
OSCOMP_TEST_GROUPS=ltp LTP_CASES=kill10 LTP_CASE_TIMEOUT_SECS=120 timeout 25m ./run-eval.sh rv
```

Artifacts:

- Pre-hypothesis raw log: `target/ltp-1000-milestone-03-stable656/rv-kill10-singleton-20260602T030343Z.log`
- Pre-hypothesis summary: `target/ltp-1000-milestone-03-stable656/rv-kill10-singleton-20260602T030343Z.summary.txt`
- Pre-hypothesis JSON: `target/ltp-1000-milestone-03-stable656/rv-kill10-singleton-20260602T030343Z.summary.json`
- Pre-hypothesis checksums: `target/ltp-1000-milestone-03-stable656/rv-kill10-singleton-20260602T030343Z.derived.sha256`
- Poll/exit cleanup hypothesis raw log: `target/ltp-1000-milestone-03-stable656/rv-kill10-poll-exit-cleanup-20260602T031039Z.log`
- Poll/exit cleanup hypothesis summary: `target/ltp-1000-milestone-03-stable656/rv-kill10-poll-exit-cleanup-20260602T031039Z.summary.txt`
- Poll/exit cleanup hypothesis JSON: `target/ltp-1000-milestone-03-stable656/rv-kill10-poll-exit-cleanup-20260602T031039Z.summary.json`
- Poll/exit cleanup hypothesis checksums: `target/ltp-1000-milestone-03-stable656/rv-kill10-poll-exit-cleanup-20260602T031039Z.derived.sha256`

Parser summary for both singleton runs:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 1
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 1
ENOSYS/not implemented matches: 0
panic/trap matches: 1
```

Case-matrix observation:

- RV musl `kill10` returns wrapper FAIL 137 after the 120s case timeout.
- Free frames fall from about `258812` before the musl run to `129627` after cleanup, leaving a persistent negative delta of roughly `129185` frames.
- The following RV glibc group starts with the reduced free-frame count and immediately panics in the allocator: `memory allocation of 262144 bytes failed`.

Decision: `kill10` is isolated as a severe cleanup/resource-lifetime blocker. A temporary generic `poll`/`ppoll` exit-group cleanup hypothesis was tested and rejected because the same timeout, persistent frame leak, and glibc allocator panic remained. The temporary source edit was removed, and no `kill10` evidence is eligible for promotion.


## `mincore03` mincore/mlock residency proof

The earlier mixed RV scout showed `mincore03` as non-promotable because both libcs hit `TBROK` with `mincore failed: ENOMEM`. The retained repair is generic: `mincore(2)` now treats pages inside an existing lazy VMA as valid but non-resident until a PTE/shared mapping exists, while `mlock(2)` validates and prefaults mapped ranges.

Targeted commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mincore03 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mincore03 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-mincore03-mincore-mlock-20260602T032124Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-mincore03-mincore-mlock-20260602T032124Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-mincore03-mincore-mlock-20260602T032124Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-mincore03-mincore-mlock-20260602T032124Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-mincore03-mincore-mlock-20260602T032208Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-mincore03-mincore-mlock-20260602T032208Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-mincore03-mincore-mlock-20260602T032208Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-mincore03-mincore-mlock-20260602T032208Z.derived.sha256`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean10-mincore03-mincore-mlock-20260602T032401Z.promotion-candidates.txt`

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

Decision: `mincore03` is now RV + LA x musl + glibc parser-clean and enters the future candidate pool. Stable list remains unchanged until the +50 milestone gate is met.

Evidence hygiene note: the raw `.log` files include an automatically appended `# LTP summary` block after the guest output. Fresh parser spot-checks use the pre-summary guest-output segment (or the generated `.summary.txt`) so summary-table headings are not double-counted as raw testcase output.

## Adjacent mincore/mlock/mmap regression subset

Cases: `mincore01,mlock01,mlock03,mlock04,munlock01,mlockall01,mmap01,mmap02,mmap03,mmap04`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mincore01,mlock01,mlock03,mlock04,munlock01,mlockall01,mmap01,mmap02,mmap03,mmap04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mincore01,mlock01,mlock03,mlock04,munlock01,mlockall01,mmap01,mmap02,mmap03,mmap04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-mincore03-adjacent-regression-20260602T032259Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-mincore03-adjacent-regression-20260602T032259Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-mincore03-adjacent-regression-20260602T032259Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-mincore03-adjacent-regression-20260602T032259Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-mincore03-adjacent-regression-20260602T032401Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-mincore03-adjacent-regression-20260602T032401Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-mincore03-adjacent-regression-20260602T032401Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-mincore03-adjacent-regression-20260602T032401Z.derived.sha256`

Parser result on each arch:

```text
PASS LTP CASE: 20
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Decision: the adjacent mincore/mlock/mmap subset did not regress on RV or LA. At the mincore03 checkpoint the clean candidate pool was 10/50; after the later G009 clean4 confirmation it was 14/50 before the LTP device/NAME_MAX clean5 update raised it to 19/50. `LTP_STABLE_CASES` remains `606 total / 606 unique / 0 duplicate`.

## `epoll_create02` singleton boundary confirmation

Date: 2026-06-02.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=epoll_create02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=epoll_create02 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-epoll-create02-singleton-20260602T033549Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-epoll-create02-singleton-20260602T033549Z.derived.sha256`

Parser result:

- RV: 1 wrapper PASS / 1 wrapper FAIL. RV glibc wrapper PASSes but includes one architecture-level `TCONF`; RV musl FAILs with `TFAIL=2` and `ENOSYS=2` because libc `epoll_create(0/-1)` reaches a missing old-ABI path instead of returning `EINVAL`.
- LA: 2 wrapper PASS / 0 wrapper FAIL, but both libcs include the same old-ABI `TCONF`; there is no timeout, ENOSYS, panic, or trap on LA.

Decision: `epoll_create02` is not promotion evidence. The RV musl ENOSYS path and the parser-visible old-ABI `TCONF` prevent a four-way parser-clean stable gate. No `LTP_STABLE_CASES` edit is made.


## G009 mm/mlock/mmap RV scout and LA clean4 confirmation

Date: 2026-06-02. This was an evidence-only checkpoint after the prior generic `mincore`/`mlock` and existing `mprotect` behavior; no stable-list edit was made.

RV scout command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mincore02,mincore04,mlock02,mlock05,mlock201,mlock202,mlock203,mlockall02,mlockall03,munlock02,munlockall01,mprotect01,mprotect02,mprotect03,mprotect04,mmap08,mmap16,mmap18,mmap20 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
```

LA confirmation command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mincore02,mincore04,mprotect02,mprotect04 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.summary.json`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-mlock-mmap-scout-20260602T034405Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-g009-mincore-mprotect-clean4-confirm-20260602T034707Z.derived.sha256`
- Combined candidate report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean14-g009-mm-mprotect-20260602T034707Z.promotion-candidates.txt`

RV parser summary:

```text
PASS LTP CASE: 8
FAIL LTP CASE: 30
Internal TFAIL/TBROK/TCONF: 60 ({'TFAIL': 50, 'TBROK': 4, 'TCONF': 6})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Suite summaries: ltp-musl 4 passed / 15 failed; ltp-glibc 4 passed / 15 failed
```

LA clean4 parser summary:

```text
PASS LTP CASE: 8
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Suite summaries: ltp-musl 4 passed / 0 failed; ltp-glibc 4 passed / 0 failed
```

Decision: `mincore02`, `mincore04`, `mprotect02`, and `mprotect04` are now RV + LA x musl + glibc parser-clean and enter the future candidate pool. The other RV scout rows remain blocker evidence because they retain parser-visible `TFAIL/TBROK/TCONF` and were not LA-confirmed. Candidate pool was 14/50 at that checkpoint; `LTP_STABLE_CASES` remained `606 total / 606 unique / 0 duplicate`.

## RV statfs01-family device-acquire blocker scout

Date: 2026-06-02. This was a VFS/statfs setup-boundary scout after the earlier `generic_statfs` capacity-clamp regression passed adjacent stable rows. No source or stable-list edit was made.

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs01,fstatfs01,fstatfs01_64,statvfs01 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.summary.json`
- RV promotion-candidate report: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-statfs01-family-scout-20260602T035624Z.derived.sha256`

Parser result:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 8
Internal TFAIL/TBROK/TCONF: 8 ({'TBROK': 8})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Suite summaries: ltp-musl 0 passed / 4 failed; ltp-glibc 0 passed / 4 failed
Promotion candidates: 0; blocked/incomplete cases: 4
```

Raw-log diagnostic: `strings` over the RV log shows every case entering LTP setup and failing with `No free devices found` / `Failed to acquire device`. This is a visible setup-device blocker, not promotion-clean statfs/statvfs behavior evidence.

Decision: no LA confirmation was run, no blacklist change was made, and no stable promotion is allowed from this shard. Candidate pool remained 14/50 at that checkpoint and stable remained `606 total / 606 unique / 0 duplicate`.

## RV VFS-C mknod/rename device-acquire blocker scout

Date: 2026-06-02. This was a VFS create/rename/path setup-boundary scout for remaining not-stable VFS-C rows. No source or stable-list edit was made.

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mknod07,mknodat02,rename03,rename04,rename05 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.summary.json`
- RV promotion-candidate report: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-vfs-c-mknod-rename-scout-20260602T040413Z.derived.sha256`

Parser result:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 10
Internal TFAIL/TBROK/TCONF: 14 ({'TBROK': 14})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Suite summaries: ltp-musl 0 passed / 5 failed; ltp-glibc 0 passed / 5 failed
Promotion candidates: 0; blocked/incomplete cases: 5
```

Raw-log diagnostic: `strings` over the RV log shows every case entering LTP setup and failing with `No free devices found` / `Failed to acquire device`. This is a visible setup-device blocker, not promotion-clean mknod/rename behavior evidence.

Decision: no LA confirmation was run, no blacklist change was made, and no stable promotion is allowed from this shard. Candidate pool remained 14/50 at that checkpoint and stable remained `606 total / 606 unique / 0 duplicate`.

## LTP device/NAME_MAX repair and clean5 confirmation

Date: 2026-06-02. This closes the earlier setup-device blocker for the statfs family and `rename05` without promoting stable cases yet.

### Commands

Enumeration-only RV retest (insufficient, blocker history):

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs01,fstatfs01,fstatfs01_64,statvfs01,mknod07,mknodat02,rename03,rename04,rename05 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
```

RV retest after global `LTP_DEV=/dev/vda` but before true NAME_MAX reporting (repair history; not countable because of panic/trap):

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs01,fstatfs01,fstatfs01_64,statvfs01,mknod07,mknodat02,rename03,rename04,rename05 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
```

Final RV retest after `LTP_DEV=/dev/vda`, `/dev` block-device enumeration/stat, and `NAME_MAX=63` reporting:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs01,fstatfs01,fstatfs01_64,statvfs01,mknod07,mknodat02,rename03,rename04,rename05 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
```

LA confirmation for the RV-clean subset:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs01,fstatfs01,fstatfs01_64,statvfs01,rename05 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Adjacent regression subset after global `LTP_DEV` and NAME_MAX changes:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=chdir01,pathconf01,fpathconf01 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=chdir01,pathconf01,fpathconf01 LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

### Artifacts

- RV enumeration-only retest: `target/ltp-1000-milestone-03-stable656/rv-device-enumeration-retest-20260602T041227Z.log`
- RV enumeration-only summary: `target/ltp-1000-milestone-03-stable656/rv-device-enumeration-retest-20260602T041227Z.summary.txt`
- RV enumeration-only checksums: `target/ltp-1000-milestone-03-stable656/rv-device-enumeration-retest-20260602T041227Z.derived.sha256`
- RV pre-NAME_MAX panic retest: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-vda-device-retest-20260602T041431Z.log`
- RV pre-NAME_MAX summary: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-vda-device-retest-20260602T041431Z.summary.txt`
- RV pre-NAME_MAX checksums: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-vda-device-retest-20260602T041431Z.derived.sha256`
- RV `statvfs01` singleton after NAME_MAX fix: `target/ltp-1000-milestone-03-stable656/rv-statvfs01-ltpdev-namemax-retest-20260602T041604Z.summary.txt`
- RV final 9-case retest summary: `target/ltp-1000-milestone-03-stable656/rv-device-cases-ltpdev-namemax-retest-20260602T041654Z.summary.txt`
- RV final 9-case checksums: `target/ltp-1000-milestone-03-stable656/rv-device-cases-ltpdev-namemax-retest-20260602T041654Z.derived.sha256`
- LA clean5 confirmation summary: `target/ltp-1000-milestone-03-stable656/la-device-clean5-ltpdev-namemax-retest-20260602T041803Z.summary.txt`
- LA clean5 checksums: `target/ltp-1000-milestone-03-stable656/la-device-clean5-ltpdev-namemax-retest-20260602T041803Z.derived.sha256`
- RV regression subset summary: `target/ltp-1000-milestone-03-stable656/rv-ltpdev-namemax-regression-subset-20260602T041926Z.summary.txt`
- LA regression subset summary: `target/ltp-1000-milestone-03-stable656/la-ltpdev-namemax-regression-subset-20260602T042012Z.summary.txt`
- Combined clean19 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean19-ltpdev-namemax-20260602T041803Z.promotion-candidates.txt`

### Parser results

Enumeration-only RV retest:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 18
Internal TFAIL/TBROK/TCONF: 22 ({'TBROK': 22})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Pre-NAME_MAX RV retest:

```text
PASS LTP CASE: 3
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 1
```

Final RV 9-case retest:

```text
PASS LTP CASE: 10
FAIL LTP CASE: 8
Internal TFAIL/TBROK/TCONF: 18 ({'TCONF': 8, 'TFAIL': 10})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

LA clean5 confirmation:

```text
PASS LTP CASE: 10
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Regression subset after the global wrapper/NAME_MAX change:

```text
RV: PASS LTP CASE 6, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 6, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

### Decision

`statfs01`, `fstatfs01`, `fstatfs01_64`, `statvfs01`, and `rename05` are now RV + LA x musl + glibc parser-clean and enter the future candidate pool. The pool becomes 19/50, still below the stable656 +50 gate, so `LTP_STABLE_CASES` remains `606 total / 606 unique / 0 duplicate`.

`mknod07` and `mknodat02` remain non-promotable because the final RV run reaches setup but emits parser-visible `TCONF` for missing `mkfs.ext2`. `rename03` and `rename04` remain non-promotable because they now reach real assertions but retain parser-visible `TFAIL`. No blacklist/SKIP/status0/full-sweep row is counted.

## RV FD/fcntl scout and LA clean2 confirmation

Date: 2026-06-02. This was an evidence-only FD/fcntl scout; no source or stable-list edit was made.

RV scout command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fcntl15,fcntl17,fcntl24,fcntl25,fcntl26,fcntl27,fcntl31,fcntl34,fcntl37,fcntl38,fcntl39,fcntl11_64 LTP_CASE_TIMEOUT_SECS=90 timeout 40m ./run-eval.sh rv
```

LA confirmation command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fcntl15,fcntl11_64 LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.log`
- RV summary: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.summary.txt`
- RV JSON: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.summary.json`
- RV promotion report: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.promotion-candidates.txt`
- RV checksums: `target/ltp-1000-milestone-03-stable656/rv-fcntl-fd-scout-20260602T043210Z.derived.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.log`
- LA summary: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.summary.txt`
- LA JSON: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.summary.json`
- LA checksums: `target/ltp-1000-milestone-03-stable656/la-fcntl-clean2-confirm-20260602T043619Z.derived.sha256`
- Combined clean21 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean21-fcntl-fd-20260602T043619Z.promotion-candidates.txt`

RV parser summary:

```text
PASS LTP CASE: 4
FAIL LTP CASE: 20
Internal TFAIL/TBROK/TCONF: 26 ({'TCONF': 14, 'TFAIL': 6, 'TBROK': 6})
timeout matches: 2
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Suite summaries: ltp-musl 2 passed / 10 failed; ltp-glibc 2 passed / 10 failed
```

LA clean2 parser summary:

```text
PASS LTP CASE: 4
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
Suite summaries: ltp-musl 2 passed / 0 failed; ltp-glibc 2 passed / 0 failed
```

Decision: `fcntl15` and `fcntl11_64` are now RV + LA x musl + glibc parser-clean and enter the future candidate pool. The pool becomes 21/50, still below the stable656 +50 gate, so `LTP_STABLE_CASES` remains `606 total / 606 unique / 0 duplicate`.

The rest of the RV scout is blocker evidence only: `fcntl17` timed out for both libcs; `fcntl24`, `fcntl25`, `fcntl26`, and `fcntl37` retain `TCONF`; `fcntl27` and `fcntl31` retain `TFAIL`; `fcntl34`, `fcntl38`, and `fcntl39` retain `TBROK`. These rows were not LA-confirmed and are not counted.

## VFS/path scout and rename inode confirmation

Date: 2026-06-02. This checkpoint first ran a broad RV VFS/path scout, then retained a generic rename metadata fix after `rename01` exposed non-preserved inode numbers across successful `rename()`.

RV scout command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=link01,link02,link03,link04,link05,linkat01,linkat02,rename01,rename02,renameat01,renameat02,stat03,stat03_64,statx01,statx04,statx05,getdents01,getdents02,unlink01,readlink03,writev03,chmod02,readlink02 LTP_CASE_TIMEOUT_SECS=90 timeout 50m ./run-eval.sh rv
```

Targeted confirmation commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=rename01,rename05 LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=rename01,rename05 LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES=rename01 LTP_CASE_TIMEOUT_SECS=90 timeout 20m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=rename01 LTP_CASE_TIMEOUT_SECS=90 timeout 20m ./run-eval.sh la
```

Artifacts:

- RV scout raw log: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-link-statx-scout-20260602T044314Z.log`
- RV scout summary: `target/ltp-1000-milestone-03-stable656/rv-vfs-path-link-statx-scout-20260602T044314Z.summary.txt`
- RV scout JSON/checksums/promotion report: `rv-vfs-path-link-statx-scout-20260602T044314Z.summary.json`, `rv-vfs-path-link-statx-scout-20260602T044314Z.derived.sha256`, `rv-vfs-path-link-statx-scout-20260602T044314Z.promotion-candidates.txt`
- RV rename01+rename05 regression summary: `target/ltp-1000-milestone-03-stable656/rv-rename-inode-retarget-20260602T044708Z.summary.txt`
- LA rename01+rename05 regression summary: `target/ltp-1000-milestone-03-stable656/la-rename-inode-retarget-20260602T044751Z.summary.txt`
- RV rename01 singleton summary: `target/ltp-1000-milestone-03-stable656/rv-rename01-inode-confirm-20260602T044855Z.summary.txt`
- LA rename01 singleton summary: `target/ltp-1000-milestone-03-stable656/la-rename01-inode-confirm-20260602T044855Z.summary.txt`
- Combined clean22 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean22-rename01-inode-20260602T044855Z.promotion-candidates.txt`

RV scout parser summary:

```text
PASS LTP CASE: 4
FAIL LTP CASE: 42
Internal TFAIL/TBROK/TCONF: 79 ({'TFAIL': 53, 'TCONF': 26})
timeout matches: 0
ENOSYS/not implemented matches: 34
panic/trap matches: 0
Suite summaries: ltp-musl 2 passed / 21 failed; ltp-glibc 2 passed / 21 failed
```

RV rename01+rename05 regression parser summary:

```text
PASS LTP CASE: 4
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

LA rename01+rename05 regression parser summary:

```text
PASS LTP CASE: 4
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

RV/LA rename01 singleton summaries:

```text
RV: PASS LTP CASE 2, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 2, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

Decision: `rename01` is now RV + LA x musl + glibc parser-clean and enters the future candidate pool. The pool becomes 22/50, still below the stable656 +50 gate, so `LTP_STABLE_CASES` remains `606 total / 606 unique / 0 duplicate`.

The RV scout's other rows are blocker evidence only. `statx01` and `getdents02` wrapper-PASS on RV but contain parser-visible `TCONF`, so they are not candidates. Hard-link/linkat rows retain visible ENOSYS/TCONF/setup blockers; `stat03`, `stat03_64`, `getdents01`, and `readlink03` retain real semantic `TFAIL` blockers; missing guest binaries are not counted.

## `rename03`/`rename04` directory replacement proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=rename03,rename04,rename05,rename01 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=rename03,rename04,rename05,rename01 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES=statfs01,fstatfs01,fstatfs01_64,statvfs01,rename05 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
python3 scripts/ltp_summary.py --promotion-candidates <clean-only evidence logs>
```

Artifacts:

- RV rename raw log: `target/ltp-1000-milestone-03-stable656/rv-rename-dir-overwrite-20260602T050256Z.log`
- RV rename summary: `target/ltp-1000-milestone-03-stable656/rv-rename-dir-overwrite-20260602T050256Z.summary.txt`
- RV rename JSON: `target/ltp-1000-milestone-03-stable656/rv-rename-dir-overwrite-20260602T050256Z.summary.json`
- LA rename raw log: `target/ltp-1000-milestone-03-stable656/la-rename-dir-overwrite-20260602T050346Z.log`
- LA rename summary: `target/ltp-1000-milestone-03-stable656/la-rename-dir-overwrite-20260602T050346Z.summary.txt`
- LA rename JSON: `target/ltp-1000-milestone-03-stable656/la-rename-dir-overwrite-20260602T050346Z.summary.json`
- RV clean-only statfs/rename05 retarget raw log: `target/ltp-1000-milestone-03-stable656/rv-statfs-rename05-clean-retarget-20260602T050521Z.log`
- RV clean-only statfs/rename05 retarget summary: `target/ltp-1000-milestone-03-stable656/rv-statfs-rename05-clean-retarget-20260602T050521Z.summary.txt`
- Combined clean24 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean24-rename03-04-20260602T050630Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rename03-04-clean24-20260602T050630Z.derived.sha256`

Parser result on RV rename targeted proof:

```text
PASS LTP CASE: 8
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Parser result on LA rename targeted proof:

```text
PASS LTP CASE: 8
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Parser result on RV clean-only statfs/rename05 retarget proof:

```text
PASS LTP CASE: 10
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Combined clean-only report result: 24 promotion candidates, 26 blocked/incomplete. `rename03` and `rename04` are the only newly added clean cases in this step. The stable list remains unchanged because the pool is still 24/50.

## `stat03`/`stat03_64` path traversal proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=readlink03,stat03,stat03_64 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=readlink03,stat03,stat03_64 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=readlink03,stat03,stat03_64 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stat01,stat02,stat01_64,stat02_64,lstat01,lstat01_64,fstatat01,readlink01,readlinkat01,openat01,rename14 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stat01,stat02,stat01_64,stat02_64,lstat01,lstat01_64,fstatat01,readlink01,readlinkat01,openat01,rename14 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh la
python3 scripts/ltp_summary.py <log>
python3 scripts/ltp_summary.py --json <log>
python3 scripts/ltp_summary.py --promotion-candidates <clean-only evidence logs>
```

Artifacts:

- Initial RV repair-history raw log: `target/ltp-1000-milestone-03-stable656/rv-readlink-stat-path-20260602T051956Z.log`
- Initial RV summary/JSON/checksum: `rv-readlink-stat-path-20260602T051956Z.summary.txt`, `rv-readlink-stat-path-20260602T051956Z.summary.json`, `rv-readlink-stat-path-20260602T051956Z.sha256`
- Fixed RV raw log: `target/ltp-1000-milestone-03-stable656/rv-readlink-stat-path-nonrecursive-20260602T052206Z.log`
- Fixed RV summary/JSON/checksum: `rv-readlink-stat-path-nonrecursive-20260602T052206Z.summary.txt`, `rv-readlink-stat-path-nonrecursive-20260602T052206Z.summary.json`, `rv-readlink-stat-path-nonrecursive-20260602T052206Z.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-readlink-stat-path-nonrecursive-20260602T052251Z.log`
- LA summary/JSON/checksum: `la-readlink-stat-path-nonrecursive-20260602T052251Z.summary.txt`, `la-readlink-stat-path-nonrecursive-20260602T052251Z.summary.json`, `la-readlink-stat-path-nonrecursive-20260602T052251Z.sha256`
- RV adjacent regression: `target/ltp-1000-milestone-03-stable656/rv-stat-readlink-stable-regression-20260602T052501Z.summary.txt`, `.summary.json`, `.sha256`
- LA adjacent regression: `target/ltp-1000-milestone-03-stable656/la-stat-readlink-stable-regression-20260602T052706Z.summary.txt`, `.summary.json`, `.sha256`
- Combined clean26 report: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean26-stat03-path-20260602T052251Z.promotion-candidates.txt`
- Combined report checksum: `target/ltp-1000-milestone-03-stable656/combined-candidate-pool-clean26-stat03-path-20260602T052251Z.promotion-candidates.derived.sha256`

Initial RV repair-history parser summary:

```text
readlink03 became RV-clean, but stat03/stat03_64 triggered a parser-visible panic/trap after recursive parent-directory search checking. This log is explicitly non-countable repair history.
```

Fixed RV targeted parser summary:

```text
PASS LTP CASE: 6
FAIL LTP CASE: 0
Internal TFAIL/TBROK/TCONF: 0 ({})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

LA targeted parser summary:

```text
PASS LTP CASE: 5
FAIL LTP CASE: 1
Internal TFAIL/TBROK/TCONF: 1 ({'TFAIL': 1})
timeout matches: 0
ENOSYS/not implemented matches: 0
panic/trap matches: 0
```

Decision: `stat03` and `stat03_64` are RV + LA x musl + glibc parser-clean and enter the future candidate pool. `readlink03` remains blocked because LA musl is parser-visible `TFAIL`; the RV clean row and LA glibc clean row are not enough for promotion.

Adjacent stable regression parser summaries:

```text
RV: PASS LTP CASE 22, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 22, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

Combined clean-only report result: 26 promotion candidates, 27 blocked/incomplete. `stat03` and `stat03_64` are the only newly added clean cases in this step. The stable list remains unchanged because the pool is still 26/50.

## `mmap20`/`munlock02` targeted proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap08,mmap20,mlock02,munlock02 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap20,munlock02 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap20,munlock02 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap01,mmap02,mmap03,mmap04,mmap09,mmap12,mmap13,munmap01,munlock01,mincore02,mincore03,mincore04,mprotect02,mprotect04 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap01,mmap02,mmap03,mmap04,mmap09,mmap12,mmap13,munmap01,munlock01,mincore02,mincore03,mincore04,mprotect02,mprotect04 LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh la
python3 scripts/ltp_summary.py <log>
python3 scripts/ltp_summary.py --json <log>
python3 scripts/ltp_summary.py --promotion-candidates target/ltp-1000-milestone-03-stable656/rv-mmap20-munlock02-targeted-20260602T054424Z.log target/ltp-1000-milestone-03-stable656/la-mmap20-munlock02-targeted-20260602T054508Z.log
```

Artifacts:

- Initial RV repair-history log/summary: `target/ltp-1000-milestone-03-stable656/rv-mmap-munlock-errno-targeted-20260602T053636Z.log`, `.summary.txt`, `.summary.json`, `.sha256` — not countable because `mmap08` and `mlock02` remain parser-visible failures.
- RV targeted raw log: `target/ltp-1000-milestone-03-stable656/rv-mmap20-munlock02-targeted-20260602T054424Z.log`
- RV targeted summary/JSON/checksum: `rv-mmap20-munlock02-targeted-20260602T054424Z.summary.txt`, `rv-mmap20-munlock02-targeted-20260602T054424Z.summary.json`, `rv-mmap20-munlock02-targeted-20260602T054424Z.sha256`
- LA targeted raw log: `target/ltp-1000-milestone-03-stable656/la-mmap20-munlock02-targeted-20260602T054508Z.log`
- LA targeted summary/JSON/checksum: `la-mmap20-munlock02-targeted-20260602T054508Z.summary.txt`, `la-mmap20-munlock02-targeted-20260602T054508Z.summary.json`, `la-mmap20-munlock02-targeted-20260602T054508Z.sha256`
- Incremental clean2 report/checksum: `target/ltp-1000-milestone-03-stable656/mmap20-munlock02-clean2-20260602T054508Z.promotion-candidates.txt`, `.sha256`
- RV regression summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-mmap-munlock-regression-20260602T054554Z.summary.txt`, `.summary.json`, `.sha256`
- LA regression summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-mmap-munlock-regression-20260602T054705Z.summary.txt`, `.summary.json`, `.sha256`
- Diagnostic-only `mmap08` fd logs: `rv-mmap08-debug-20260602T054205Z.log` and `rv-mmap08-debug-fdpath-20260602T054313Z.log`; these are not promotion evidence.

Targeted parser summaries:

```text
RV: PASS LTP CASE 4, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 4, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

Regression parser summaries:

```text
RV: PASS LTP CASE 28, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 28, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

Promotion-candidate report result: 2 promotion candidates (`mmap20`, `munlock02`), 0 blocked/incomplete rows for the incremental proof. Combined with the previous clean26 audit, the current not-yet-promoted pool is 28/50; stable list remains unchanged at `606/606/0`.


## `epoll_create1_01`/`epoll_create1_02` targeted proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES="epoll_create1_01 epoll_create1_02" LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES="epoll_create1_01 epoll_create1_02" LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 25m ./run-eval.sh la
python3 scripts/ltp_summary.py <log>
python3 scripts/ltp_summary.py --json <log>
python3 scripts/ltp_summary.py --promotion-candidates target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.log target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.log
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.log`
- RV summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.txt`, `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.summary.json`, `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-final-20260602T061430Z.sha256`
- LA raw log: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.log`
- LA summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.summary.json`, `target/ltp-1000-milestone-03-stable656/la-epoll-create1-final-20260602T061430Z.sha256`
- Incremental clean2 report/checksum: `target/ltp-1000-milestone-03-stable656/epoll-create1-clean2-20260602T061430Z.promotion-candidates.txt`, `target/ltp-1000-milestone-03-stable656/epoll-create1-clean2-20260602T061430Z.promotion-candidates.sha256`

Targeted parser summaries:

```text
RV: PASS LTP CASE 4, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 4, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

Adjacent FD/flag regression commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES="close01 fcntl01 fcntl05 dup01 pipe2_01 poll01" LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES="close01 fcntl01 fcntl05 dup01 pipe2_01 poll01" LTP_DEV=/dev/vda LTP_CASE_TIMEOUT_SECS=90 timeout 30m ./run-eval.sh la
```

Regression artifacts:

- RV summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.summary.txt`, `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.summary.json`, `target/ltp-1000-milestone-03-stable656/rv-epoll-create1-fd-regression-20260602T060838Z.sha256`
- LA summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.summary.json`, `target/ltp-1000-milestone-03-stable656/la-epoll-create1-fd-regression-20260602T061054Z.sha256`

Regression parser summaries:

```text
RV: PASS LTP CASE 12, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 12, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

Decision: `epoll_create1_01` and `epoll_create1_02` are RV + LA x musl + glibc parser-clean and enter the future candidate pool. Combined with the previous clean28 audit, the current not-yet-promoted pool is 30/50; stable list remains unchanged at `606/606/0`.

`epoll_create02` remains blocker evidence only. `rv-epoll-create02-create1-20260602T060510Z.summary.txt` shows the kernel now exposes `epoll_create1(0)`, but musl's `epoll_create(size)` wrapper converts the invalid old-size call into valid `epoll_create1(0)`, so invalid `size` is not visible at the syscall boundary. Do not make `epoll_create1(0)` invalid to satisfy that old-wrapper row.


## `adjtimex01`/`adjtimex03`/`sigaltstack02`/`shmt04` targeted proof

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=adjtimex01,adjtimex03,sigaltstack02,shmt04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=adjtimex01,adjtimex03,sigaltstack02,shmt04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py <log>
python3 scripts/ltp_summary.py --json <log>
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs musl,glibc target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.log target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.log
```

Artifacts:

- RV raw/meta: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.log`, `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.log.meta`
- RV summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-shmt04-targeted-20260602T143608+0800.derived.sha256`
- LA raw/meta: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.log`, `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.log.meta`
- LA summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-shmt04-targeted-20260602T143702+0800.derived.sha256`
- Incremental clean4 report/checksum: `target/ltp-1000-milestone-03-stable656/combined-clock-sigaltstack-shmt04-20260602T143805+0800.promotion-candidates.txt`, `target/ltp-1000-milestone-03-stable656/combined-clock-sigaltstack-shmt04-20260602T143805+0800.promotion-candidates.txt.sha256`

Targeted parser summaries:

```text
RV: PASS LTP CASE 8, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 8, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

Adjacent time/signal regression commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=clock_gettime02,clock_nanosleep02,nanosleep01,rt_sigaction01,rt_sigprocmask01,sigaction01,sigprocmask01 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=clock_gettime02,clock_nanosleep02,nanosleep01,rt_sigaction01,rt_sigprocmask01,sigaction01,sigprocmask01 LTP_CASE_TIMEOUT_SECS=90 timeout 35m ./run-eval.sh la
```

Regression artifacts:

- RV summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-adjacent-regression-20260602T143818+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-adjacent-regression-20260602T143818+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/rv-clock-sigaltstack-adjacent-regression-20260602T143818+0800.derived.sha256`
- LA summary/JSON/checksum: `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-adjacent-regression-20260602T143950+0800.summary.txt`, `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-adjacent-regression-20260602T143950+0800.summary.json`, `target/ltp-1000-milestone-03-stable656/la-clock-sigaltstack-adjacent-regression-20260602T143950+0800.derived.sha256`

Regression parser summaries:

```text
RV: PASS LTP CASE 14, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
LA: PASS LTP CASE 14, FAIL LTP CASE 0, Internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
```

Decision: `adjtimex01`, `adjtimex03`, `shmt04`, and `sigaltstack02` are RV + LA x musl + glibc parser-clean and enter the future candidate pool. Combined with the previous clean30 audit, the current not-yet-promoted pool is 34/50; stable list remains unchanged at `606/606/0`.

Boundary: `sigaltstack` now records and reports alternate-stack state and errno validation, but signal delivery still uses the existing signal-frame path rather than switching to the alternate stack. Future signal-delivery work must not count this syscall-only proof as full alternate-stack delivery semantics.
