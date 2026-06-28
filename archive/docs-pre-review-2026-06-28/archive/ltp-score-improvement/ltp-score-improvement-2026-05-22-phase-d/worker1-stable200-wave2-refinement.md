# Worker 1 STABLE200 wave2 candidate refinement

Scope: read-only refinement while LA clean7 runs. No QEMU/evaluator run, no code edits, no `LTP_STABLE_CASES` edit, and no `.omx/ultragoal` mutation.

## Inputs inspected

- Broad RV pool: `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-22-phase-d/stable200-wave2-broad-rv.cases`
- Stable baseline exclusion: `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-22-phase-d/stable180.cases`
- Prior blocker snapshot: `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-22-phase-d/candidate-matrix.md`
- Wave1 RV summary: `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-22-phase-d/stable200-wave1-rv-summary.json`
- Wave1 clean7 LA summary: `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-22-phase-d/stable200-wave1-clean7-la-summary.json`
- RV sdcard testcase availability checked read-only with `debugfs -R 'ls /musl/ltp/testcases/bin'` and `debugfs -R 'ls /glibc/ltp/testcases/bin'` against `/root/oskernel2026-orays/sdcard-rv.img`.

## Refinement rule

Keep a case only when all of the following are true:

1. It is in the wave2 broad pool.
2. It is not already present in `stable180.cases`.
3. It has both musl and glibc testcase binaries present on the RV sdcard.
4. It is not listed in the Phase-D matrix RV blocked snapshot with TFAIL/TBROK/TCONF/timeout/ENOSYS/code failure evidence.

This is intentionally a highest-probability RV discovery batch, not promotion evidence.

## Output batch

- Output cases file: `docs/ltp-score-improvement-2026-05-22-phase-d/worker1-stable200-wave2-refined-rv.cases`
- Broad pool count: 71
- Already stable180 in broad pool: 0
- RV sdcard available in both libcs: 71 / 71
- Excluded due known RV blocker snapshot: 17
- Recommended RV targeted batch count: 54

```text
gettid02
waitpid11
waitpid12
waitpid13
dup05
pread02
pwrite02
pwrite03
pwrite04
readlink03
link04
link05
rename03
rename04
renameat201
renameat202
mkdir02
mkdir03
mkdir04
mkdir05
rmdir02
rmdir03
fstat02
ftruncate04
utime01
utime02
utimes01
futimesat01
clock_nanosleep01
nanosleep01
kill06
pause02
setitimer02
getitimer02
timer_delete01
timer_gettime01
timer_settime01
mmap02
mmap03
mmap04
mmap05
mmap06
mmap08
munmap01
munmap02
mprotect01
mprotect02
mincore01
mlock01
munlock01
sysinfo03
chroot01
sync01
syncfs01
```

## Recommended split by lane

### process/wait/session (4)

```text
gettid02
waitpid11
waitpid12
waitpid13
```

### fd/io/open (6)

```text
dup05
pread02
pwrite02
pwrite03
pwrite04
readlink03
```

### fs/metadata (18)

```text
link04
link05
rename03
rename04
renameat201
renameat202
mkdir02
mkdir03
mkdir04
mkdir05
rmdir02
rmdir03
fstat02
ftruncate04
utime01
utime02
utimes01
futimesat01
```

### time/signal/timer (9)

```text
clock_nanosleep01
nanosleep01
kill06
pause02
setitimer02
getitimer02
timer_delete01
timer_gettime01
timer_settime01
```

### memory/mm (13)

```text
mmap02
mmap03
mmap04
mmap05
mmap06
mmap08
munmap01
munmap02
mprotect01
mprotect02
mincore01
mlock01
munlock01
```

### misc/system (4)

```text
sysinfo03
chroot01
sync01
syncfs01
```

## Excluded from wave2 refined batch

These are available/non-stable candidates but already carry RV blocked evidence in the matrix, so they should stay in repair lanes instead of the next high-probability discovery batch.

| Case | Exclusion reason |
| --- | --- |
| `wait403` | known RV blocked snapshot: glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `waitid01` | known RV blocked snapshot: glibc:ENOSYS, glibc:TBROK=1, glibc:TFAIL=5, glibc:code=3, musl:ENOSYS, musl:TBROK=1, musl:TFAIL=5, musl:code=3 |
| `waitid02` | known RV blocked snapshot: glibc:ENOSYS, glibc:TFAIL=1, glibc:code=1, musl:ENOSYS, musl:TFAIL=1, musl:code=1 |
| `waitid03` | known RV blocked snapshot: glibc:ENOSYS, glibc:TFAIL=1, glibc:code=1, musl:ENOSYS, musl:TFAIL=1, musl:code=1 |
| `waitid04` | known RV blocked snapshot: glibc:ENOSYS, glibc:TFAIL=1, glibc:code=1, musl:ENOSYS, musl:TFAIL=1, musl:code=1 |
| `setsid01` | known RV blocked snapshot: glibc:TFAIL=3, musl:TFAIL=2 |
| `open07` | known RV blocked snapshot: glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `open08` | known RV blocked snapshot: glibc:TFAIL=4, glibc:code=1, musl:TFAIL=4, musl:code=1 |
| `open09` | known RV blocked snapshot: glibc:TFAIL=2, glibc:code=1, musl:TFAIL=2, musl:code=1 |
| `open10` | known RV blocked snapshot: glibc:TFAIL=3, glibc:code=1, musl:TBROK=1, musl:code=2 |
| `open11` | known RV blocked snapshot: glibc:ENOSYS, glibc:TBROK=1, glibc:code=2, musl:ENOSYS, musl:TBROK=1, musl:code=2 |
| `open12` | known RV blocked snapshot: glibc:TBROK=2, glibc:code=2, musl:TBROK=2, musl:code=2 |
| `openat03` | known RV blocked snapshot: glibc:TBROK=2, glibc:code=2, musl:TBROK=2, musl:code=2 |
| `read03` | known RV blocked snapshot: glibc:ENOSYS, glibc:TBROK=1, glibc:code=6, musl:ENOSYS, musl:TBROK=1, musl:code=6 |
| `fstat03` | known RV blocked snapshot: musl:TFAIL=1, musl:code=1 |
| `truncate03` | known RV blocked snapshot: glibc:TFAIL=2, glibc:code=1, musl:TFAIL=2, musl:code=1 |
| `alarm05` | known RV blocked snapshot: musl:TFAIL=1, musl:code=1 |


## Wave1 context used for guardrails

- Wave1 RV summary rows: 134; PASS wrapper rows: 22; FAIL wrapper rows: 112; internal markers: {'TBROK': 39, 'TCONF': 4, 'TFAIL': 1333}; timeouts: 5; ENOSYS: 8.
- Wave1 clean7 LA summary rows: 14; PASS wrapper rows: 13; FAIL wrapper rows: 1; internal markers: {'TFAIL': 1}; timeouts: 0; ENOSYS: 0.
- Important guardrail: `sched_getscheduler02` was RV clean but LA musl failed in clean7; do not infer LA success from RV-only wave2 results.

## Suggested RV-only command shape for leader/runner

Do not run from this worker. If leader chooses this batch, keep it targeted and parser-gated:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=file:docs/ltp-score-improvement-2026-05-22-phase-d/worker1-stable200-wave2-refined-rv.cases \
LTP_CASE_TIMEOUT_SECS=30 \
./run-eval.sh 2>&1 | tee docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable200-wave2-refined-rv.log
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable200-wave2-refined-rv.log \
  > docs/ltp-score-improvement-2026-05-22-phase-d/stable200-wave2-refined-rv-summary.txt
python3 -B scripts/ltp_summary.py --json docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable200-wave2-refined-rv.log \
  > docs/ltp-score-improvement-2026-05-22-phase-d/stable200-wave2-refined-rv-summary.json
```

Promotion stop condition: only cases with RV and LA musl/glibc wrapper PASS plus zero TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap can be considered by the leader for stable-list edits.
