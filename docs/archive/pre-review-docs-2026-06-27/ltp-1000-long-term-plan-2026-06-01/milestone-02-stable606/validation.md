# milestone-02-stable606 validation preflight

## Baseline / team reconciliation

```bash
omx team status complete-dev-1000ltp-c632b4a0
# No team state found for complete-dev-1000ltp-c632b4a0
```

The requested worker-4 mailbox path was absent, and the session team state reported `active=False`, `current_phase=cancelled`, `lifecycle_outcome=finished`, `run_outcome=finish`, `completed_at=2026-06-01T13:55:34.273Z`. The nudge was treated as stale; no worker was assigned.

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
# 556 556 0
```

Disk preflight before long QEMU runs: `/dev/vda2` 59G size / 24G used / 33G available / 43% on `/` and `/root`.

## RV 80-case scout

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=<80-case-inline> LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.log
python3 scripts/ltp_summary.py --json target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv --promotion-libcs glibc,musl target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.log
```

Artifacts:

- Raw: `target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.log`
- Meta: `target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.log.meta`
- Raw checksum: `target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.sha256`
- Summary: `target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.summary.txt`
- JSON: `target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.promotion-candidates.txt`
- Derived checksum: `target/ltp-1000-milestone-02-stable606/rv-m02-scout-001-20260601T154726Z.derived.sha256`

Parser summary:

- PASS LTP CASE: 51
- FAIL LTP CASE: 109
- Internal TFAIL/TBROK/TCONF: 219 (`TBROK=73`, `TFAIL=122`, `TCONF=24`)
- timeout matches: 4
- ENOSYS/not implemented matches: 6
- panic/trap matches: 0
- Suite summaries: `ltp-musl` 24 passed / 56 failed; `ltp-glibc` 27 passed / 53 failed.

## socket01 errno fix validation

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=socket01 LTP_CASE_TIMEOUT_SECS=60 timeout 40m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=socket01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-socket01-postfix-20260601T160003Z.log \
  target/ltp-1000-milestone-02-stable606/la-socket01-postfix-20260601T160247Z.log
```

Artifacts:

- RV raw/summary: `target/ltp-1000-milestone-02-stable606/rv-socket01-postfix-20260601T160003Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.log.sha256`, `.derived.sha256`
- LA raw/summary: `target/ltp-1000-milestone-02-stable606/la-socket01-postfix-20260601T160247Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.log.sha256`, `.derived.sha256`
- Four-way candidate report: `target/ltp-1000-milestone-02-stable606/socket01-rv-la-postfix.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Four-way candidate report: 1 candidate, `socket01`.

## Adjacent socket regression subset

Command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=socket01,socket02,socketpair02,accept01,listen01 LTP_CASE_TIMEOUT_SECS=60 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=socket01,socket02,socketpair02,accept01,listen01 LTP_CASE_TIMEOUT_SECS=60 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV raw/summary: `target/ltp-1000-milestone-02-stable606/rv-socket-adjacent-postfix-20260601T160853Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.log.sha256`, `.derived.sha256`
- LA raw/summary: `target/ltp-1000-milestone-02-stable606/la-socket-adjacent-postfix-20260601T160953Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.log.sha256`, `.derived.sha256`
- Four-way report: `target/ltp-1000-milestone-02-stable606/socket-adjacent-rv-la-postfix.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV: 10 PASS / 0 FAIL, no internal failures/caveats.
- LA: 10 PASS / 0 FAIL, no internal failures/caveats.
- Four-way report: `accept01`, `listen01`, `socket01`, `socket02`, `socketpair02` clean on RV + LA x musl + glibc.

## nanosleep01 rescout

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=nanosleep01 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=nanosleep01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV raw/summary: `target/ltp-1000-milestone-02-stable606/rv-nanosleep01-rescout-20260601T160605Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.log.sha256`, `.derived.sha256`
- LA raw/summary: `target/ltp-1000-milestone-02-stable606/la-nanosleep01-rescout-20260601T160721Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.log.sha256`, `.derived.sha256`
- Four-way report: `target/ltp-1000-milestone-02-stable606/nanosleep01-rv-la-rescout.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV: 2 PASS / 0 FAIL, no internal failures/caveats.
- LA: 2 PASS / 0 FAIL, no internal failures/caveats.
- Four-way candidate report: 1 candidate, `nanosleep01`.
- Caveat: earlier grouped RV scout had one musl timing TFAIL; do not rely on this isolated run alone for final stable606 promotion.

## Unverified / not yet closed

- No final `LTP_CASES=stable` RV/LA gate for stable606 was run.
- No LA full-sweep shard evidence was generated in this preflight.
- No `LTP_STABLE_CASES` update was made.

## proc-self-maps mmap/vma fix validation

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap04,vma01 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap04,vma01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap04,vma01,mmap01,mmap02,mmap03,mmap06,mmap09,mmap10,mmap11,mincore01,mprotect05 LTP_CASE_TIMEOUT_SECS=90 timeout 75m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap04,vma01,mmap01,mmap02,mmap03,mmap06,mmap09,mmap10,mmap11,mincore01,mprotect05 LTP_CASE_TIMEOUT_SECS=90 timeout 75m ./run-eval.sh la
```

Artifacts:

- RV singleton raw/summary: `target/ltp-1000-milestone-02-stable606/rv-proc-maps-mmap-vma-postfix2-20260601T162318Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA singleton raw/summary: `target/ltp-1000-milestone-02-stable606/la-proc-maps-mmap-vma-postfix-20260601T162441Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-proc-maps-mmap-regression-20260601T162607Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-proc-maps-mmap-regression-20260601T162755Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Four-way report: `target/ltp-1000-milestone-02-stable606/proc-maps-mmap-regression-rv-la.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV singleton: 4 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA singleton: 4 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- RV regression subset: 22 PASS / 0 FAIL, no parser caveats.
- LA regression subset: 22 PASS / 0 FAIL, no parser caveats.
- New four-way not-yet-stable candidates from this fix: `mmap04`, `vma01`.


## times03 CPU accounting validation

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=times03 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=times03 LTP_CASE_TIMEOUT_SECS=120 timeout 60m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-times03-postfix-20260601T164216Z.log \
  target/ltp-1000-milestone-02-stable606/la-times03-postfix-20260601T164436Z.log
```

Artifacts:

- RV singleton raw/summary: `target/ltp-1000-milestone-02-stable606/rv-times03-postfix-20260601T164216Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA singleton raw/summary: `target/ltp-1000-milestone-02-stable606/la-times03-postfix-20260601T164436Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Four-way report: `target/ltp-1000-milestone-02-stable606/times03-rv-la-postfix.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV singleton: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA singleton: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Four-way candidate report: 1 candidate, `times03`.

## times03 adjacent time regression subset

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=times01,times03,gettimeofday01,gettimeofday02,clock_gettime02 LTP_CASE_TIMEOUT_SECS=60 timeout 50m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=times01,times03,gettimeofday01,gettimeofday02,clock_gettime02 LTP_CASE_TIMEOUT_SECS=120 timeout 70m ./run-eval.sh la
```

Artifacts:

- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-times03-regression-20260601T164708Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-times03-regression-20260601T164956Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Four-way report: `target/ltp-1000-milestone-02-stable606/times03-regression-rv-la.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV regression subset: 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA regression subset: 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `times01`, `times03`, `gettimeofday01`, `gettimeofday02`, and `clock_gettime02` all clean across RV + LA x musl + glibc; only `times03` is new relative to current stable list.

## mmap14 MAP_LOCKED / VmLck validation

Pre-fix evidence from the 80-case RV scout:

- `mmap14` failed in both musl and glibc with `Expected 1024K locked, get 0K locked`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap14 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap14 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-mmap14-postfix-20260601T170355Z.log \
  target/ltp-1000-milestone-02-stable606/la-mmap14-postfix-20260601T170553Z.log
```

Artifacts:

- RV singleton raw/summary: `target/ltp-1000-milestone-02-stable606/rv-mmap14-postfix-20260601T170355Z.log`, `.summary.txt`, `.summary.json`, `.sha256`, `.derived.sha256`
- LA singleton raw/summary: `target/ltp-1000-milestone-02-stable606/la-mmap14-postfix-20260601T170553Z.log`, `.summary.txt`, `.summary.json`, `.sha256`, `.derived.sha256`
- Four-way report: `target/ltp-1000-milestone-02-stable606/mmap14-rv-la-postfix.promotion-candidates.txt`, `target/ltp-1000-milestone-02-stable606/mmap14-promotion-reports.sha256`

Parser summary:

- RV singleton: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA singleton: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Four-way candidate report: 1 candidate, `mmap14`.

## mmap14 adjacent mmap/proc regression subset

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap04,vma01,mmap14,mmap01,mmap02,mmap03,mmap06,mmap09,mmap10,mmap11,mincore01,mprotect05 LTP_CASE_TIMEOUT_SECS=90 timeout 75m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap04,vma01,mmap14,mmap01,mmap02,mmap03,mmap06,mmap09,mmap10,mmap11,mincore01,mprotect05 LTP_CASE_TIMEOUT_SECS=90 timeout 75m ./run-eval.sh la
```

Artifacts:

- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-mmap14-regression-20260601T170753Z.log`, `.summary.txt`, `.summary.json`, `.sha256`, `.derived.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-mmap14-regression-20260601T171057Z.log`, `.summary.txt`, `.summary.json`, `.sha256`, `.derived.sha256`
- Four-way report: `target/ltp-1000-milestone-02-stable606/mmap14-regression-rv-la.promotion-candidates.txt`, `target/ltp-1000-milestone-02-stable606/mmap14-promotion-reports.sha256`

Parser summary:

- RV regression subset: 24 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA regression subset: 24 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: all twelve rows clean across RV + LA x musl + glibc; only `mmap14` is new relative to the current stable list and already-banked `mmap04`/`vma01`.

## mmap12 /proc/self/pagemap validation

Pre-fix evidence from the 80-case RV scout:

- `mmap12` failed in both musl and glibc with `pagemap failed: ENOENT (2)` because `/proc/self/pagemap` did not exist.
- The relevant upstream LTP case (`mmap12.c`, LTP 20240524) opens `/proc/self/pagemap`, seeks to one pagemap entry per mapped page, and requires bit 63 (`present`) to be set after a `MAP_POPULATE` file mapping: https://raw.githubusercontent.com/linux-test-project/ltp/20240524/testcases/kernel/syscalls/mmap/mmap12.c

Commands:

```bash
LTP_CASES=mmap12 ./run-eval.sh rv
LTP_CASES=mmap12 ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates \
  target/ltp-1000-milestone-02-stable606/rv-mmap12-postfix-20260601T173127Z.log \
  target/ltp-1000-milestone-02-stable606/la-mmap12-postfix-20260601T173441Z.log
sha256sum <mmap12 singleton raw/summary/report files> > \
  target/ltp-1000-milestone-02-stable606/mmap12-postfix-evidence.sha256
```

Artifacts:

- RV singleton raw/summary: `target/ltp-1000-milestone-02-stable606/rv-mmap12-postfix-20260601T173127Z.log`, `.summary.txt`
- LA singleton raw/summary: `target/ltp-1000-milestone-02-stable606/la-mmap12-postfix-20260601T173441Z.log`, `.summary.txt`
- Four-way singleton report: `target/ltp-1000-milestone-02-stable606/mmap12-rv-la-postfix.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-02-stable606/mmap12-postfix-evidence.sha256`

Parser summary:

- RV singleton: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA singleton: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Four-way candidate report: 1 candidate, `mmap12`.

## mmap12 adjacent mmap/proc regression subset

Commands:

```bash
LTP_CASES=mmap01,mmap02,mmap03,mmap04,mmap06,mmap09,mmap11,mmap12,mmap14,mincore01,mprotect05,vma01 ./run-eval.sh rv
LTP_CASES=mmap01,mmap02,mmap03,mmap04,mmap06,mmap09,mmap11,mmap12,mmap14,mincore01,mprotect05,vma01 ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-mmap12-regression-20260601T174051Z.log \
  target/ltp-1000-milestone-02-stable606/la-mmap12-regression-20260601T174435Z.log
sha256sum <mmap12 regression raw/summary/report files> > \
  target/ltp-1000-milestone-02-stable606/mmap12-regression-evidence.sha256
```

Artifacts:

- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-mmap12-regression-20260601T174051Z.log`, `.summary.txt`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-mmap12-regression-20260601T174435Z.log`, `.summary.txt`
- Four-way regression report: `target/ltp-1000-milestone-02-stable606/mmap12-regression-rv-la.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-02-stable606/mmap12-regression-evidence.sha256`

Parser summary:

- RV regression subset: 24 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA regression subset: 24 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: all twelve rows clean across RV + LA x musl + glibc; `mmap12` is the new not-yet-stable candidate from this fix.

Non-LTP caveat: the full evaluator still reports existing `iperf-glibc` failures in these QEMU runs. They are outside the LTP parser gate and are not counted as LTP promotion evidence.

## open10 / creat08 setgid inheritance validation

Pre-fix evidence from the 80-case RV scout:

- `open10` and `creat08` failed in both musl and glibc when files were created under the setgid `dir_b`; the new files reported gid `65534` instead of the parent directory gid `1`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=open10,creat08 LTP_CASE_TIMEOUT_SECS=90 timeout 50m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=open10,creat08 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-open-creat-setgid-postfix-20260601T180048Z.log \
  target/ltp-1000-milestone-02-stable606/la-open-creat-setgid-postfix-20260601T180132Z.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=open10,creat08,open01,open03,open08,open09,creat01,creat03,creat04,creat05,chmod05,chown01,chown02,chown03,mkdir04,mknod02 LTP_CASE_TIMEOUT_SECS=90 timeout 70m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=open10,creat08,open01,open03,open08,open09,creat01,creat03,creat04,creat05,chmod05,chown01,chown02,chown03,mkdir04,mknod02 LTP_CASE_TIMEOUT_SECS=90 timeout 80m ./run-eval.sh la
```

Artifacts:

- RV singleton raw/summary: `target/ltp-1000-milestone-02-stable606/rv-open-creat-setgid-postfix-20260601T180048Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA singleton raw/summary: `target/ltp-1000-milestone-02-stable606/la-open-creat-setgid-postfix-20260601T180132Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Four-way singleton report: `target/ltp-1000-milestone-02-stable606/open-creat-setgid-rv-la-postfix.promotion-candidates.txt`, `.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-open-creat-setgid-regression-20260601T180236Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-open-creat-setgid-regression-20260601T180348Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Four-way regression report: `target/ltp-1000-milestone-02-stable606/open-creat-setgid-regression-rv-la.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV singleton: 4 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA singleton: 4 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- RV regression subset: 32 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA regression subset: 32 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined regression report: all sixteen rows clean across RV + LA x musl + glibc; only `open10` and `creat08` are new relative to current stable list.


## chmod07 / fchmod02 group database validation

Pre-fix evidence from the 80-case RV scout:

- `chmod07` and `fchmod02` failed in both musl and glibc during test setup because `getgrnam(users)` failed and the fallback `getgrnam(daemon)` also failed.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=chmod07,fchmod02 LTP_CASE_TIMEOUT_SECS=90 timeout 50m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=chmod07,fchmod02 LTP_CASE_TIMEOUT_SECS=90 timeout 50m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl   target/ltp-1000-milestone-02-stable606/rv-groupdb-chmod-fchmod-20260601T181203Z.log   target/ltp-1000-milestone-02-stable606/la-groupdb-chmod-fchmod-20260601T181243Z.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=chmod05,chmod07,fchmod02,chown01,chown02,chown03,open01,creat01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=chmod05,chmod07,fchmod02,chown01,chown02,chown03,open01,creat01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV singleton raw/summary: `target/ltp-1000-milestone-02-stable606/rv-groupdb-chmod-fchmod-20260601T181203Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA singleton raw/summary: `target/ltp-1000-milestone-02-stable606/la-groupdb-chmod-fchmod-20260601T181243Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Four-way singleton report: `target/ltp-1000-milestone-02-stable606/groupdb-chmod-fchmod-rv-la.promotion-candidates.txt`, `.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-groupdb-chmod-regression-20260601T181338Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-groupdb-chmod-regression-20260601T181429Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Four-way regression report: `target/ltp-1000-milestone-02-stable606/groupdb-chmod-regression-rv-la.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV singleton: 4 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA singleton: 4 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- RV regression subset: 16 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA regression subset: 16 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined regression report: all eight rows clean across RV + LA x musl + glibc; only `chmod07` and `fchmod02` are new relative to current stable list.

## tmpfs read-only remount metadata validation

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=access04,chmod06,chown04,fchmod06,fchown04 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=access04,chmod06,chown04,fchmod06,fchown04 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES=access04,chmod06,chown04,fchmod06,fchown04,access01,access02,chmod05,chmod07,fchmod02,chown01,chown02,chown03,open01,creat01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=access04,chmod06,chown04,fchmod06,fchown04,access01,access02,chmod05,chmod07,fchmod02,chown01,chown02,chown03,open01,creat01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV targeted raw/summary: `target/ltp-1000-milestone-02-stable606/rv-tmpfs-readonly-metadata-20260601T182849Z.log`, `.summary.txt`, `.summary.json`, `.sha256`, `.derived.sha256`
- LA targeted raw/summary: `target/ltp-1000-milestone-02-stable606/la-tmpfs-readonly-metadata-20260601T182942Z.log`, `.summary.txt`, `.summary.json`, `.sha256`, `.derived.sha256`
- Targeted four-way report: `target/ltp-1000-milestone-02-stable606/tmpfs-readonly-rv-la.promotion-candidates.txt`, `.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-tmpfs-readonly-regression-20260601T183034Z.log`, `.summary.txt`, `.summary.json`, `.sha256`, `.derived.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-tmpfs-readonly-regression-20260601T183152Z.log`, `.summary.txt`, `.summary.json`, `.sha256`, `.derived.sha256`
- Regression four-way report: `target/ltp-1000-milestone-02-stable606/tmpfs-readonly-regression-rv-la.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV targeted: 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA targeted: 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Targeted four-way report: 5 candidates: `access04`, `chmod06`, `chown04`, `fchmod06`, `fchown04`.
- RV regression subset: 30 PASS / 0 FAIL, no parser caveats.
- LA regression subset: 30 PASS / 0 FAIL, no parser caveats.
- Regression four-way report: 15 clean rows across RV + LA x musl + glibc.

The first attempted RV rebuild used the nonexistent `LinuxError::ReadOnlyFilesystem` variant and stopped at compile time before any LTP cases ran. It was corrected to the generated errno variant `LinuxError::EROFS`; that failed compile log is not promotion evidence.


## Team nudge reconciliation refresh on 2026-06-02

Commands:

```bash
omx team status complete-dev-1000ltp-c632b4a0
# No team state found for complete-dev-1000ltp-c632b4a0
OMX_TEAM_STATE_ROOT=/root/.omx-runs/run-20260601071305-5c04/.omx/state   omx team status complete-dev-1000ltp-c632b4a0
# No team state found for complete-dev-1000ltp-c632b4a0
```

The path named by the worker-2/worker-4 Stop nudges (`/root/.omx-runs/run-20260601071305-5c04/.omx/state/team/complete-dev-1000ltp-c632b4a0/mailbox/leader-fixed.json`) was absent. The remaining canonical evidence is the previously generated commit-hygiene report at `/root/oskernel2026-orays-1000ltp-leader-20260601-1336/.omx/reports/team-commit-hygiene/complete-dev-1000ltp-c632b4a0.md`, which records all seven phase0 report-only tasks completed and shutdown merges at `2026-06-01T13:55:33.785Z`. No new worker assignment was possible or needed; current work continued in leader-owned solo mode.

## /proc/self/fd / pipe07 validation

Pre-fix evidence from the 80-case RV scout:

- `pipe07` failed in both musl and glibc because `opendir(/proc/self/fd)` returned `ENOENT`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=pipe07 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=pipe07 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl   target/ltp-1000-milestone-02-stable606/rv-proc-fd-pipe07-20260601T184539Z.log   target/ltp-1000-milestone-02-stable606/la-proc-fd-pipe07-20260601T184915Z.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=pipe07,pipe01,pipe02,pipe03,pipe04,pipe05,pipe06,pipe08,pipe09,pipe10,pipe14,pipe2_01,pipe2_02,pipe2_04,proc01,readlink01,readlinkat01,fcntl01,fcntl02,fcntl03 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=pipe07,pipe01,pipe02,pipe03,pipe04,pipe05,pipe06,pipe08,pipe09,pipe10,pipe14,pipe2_01,pipe2_02,pipe2_04,proc01,readlink01,readlinkat01,fcntl01,fcntl02,fcntl03 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV targeted raw/summary: `target/ltp-1000-milestone-02-stable606/rv-proc-fd-pipe07-20260601T184539Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA targeted raw/summary: `target/ltp-1000-milestone-02-stable606/la-proc-fd-pipe07-20260601T184915Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Targeted four-way report: `target/ltp-1000-milestone-02-stable606/proc-fd-pipe07-rv-la.promotion-candidates.txt`, `.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-proc-fd-regression-20260601T185013Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-proc-fd-regression-20260601T185013Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`, `.derived.sha256`
- Regression four-way report: `target/ltp-1000-milestone-02-stable606/proc-fd-regression-rv-la.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV targeted: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA targeted: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Targeted four-way report: 1 candidate, `pipe07`.
- RV regression subset: 40 PASS / 0 FAIL, no parser caveats.
- LA regression subset: 40 PASS / 0 FAIL, no parser caveats.
- Regression four-way report: 20 clean rows across RV + LA x musl + glibc; `pipe07` is the only new row relative to current stable.

Non-LTP caveat: these are targeted LTP gates only. Full stable606 promotion and full all-minus-blacklist sweep were not run.

## mknod mode errno validation

Pre-fix evidence:

- In the post-proc-fd RV rescout, `mknod09` failed because invalid type bits returned `EPERM` instead of the expected `EINVAL`.
- `mknod03` and `mknod04` were clean in that RV-only scout, but still required fresh LA confirmation before they could enter the four-way candidate bank.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mknod03,mknod04,mknod09 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mknod03,mknod04,mknod09 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-mknod-mode-rescout-20260601T190332Z.log \
  target/ltp-1000-milestone-02-stable606/la-mknod-mode-rescout-20260601T190415Z.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mknod03,mknod04,mknod09,mknod02,open10,creat08,chmod07,fchmod02,access04,chmod06,chown04,fchmod06,fchown04 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=mknod03,mknod04,mknod09,mknod02,open10,creat08,chmod07,fchmod02,access04,chmod06,chown04,fchmod06,fchown04 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV targeted raw/summary: `target/ltp-1000-milestone-02-stable606/rv-mknod-mode-rescout-20260601T190332Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA targeted raw/summary: `target/ltp-1000-milestone-02-stable606/la-mknod-mode-rescout-20260601T190415Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- Targeted four-way report: `target/ltp-1000-milestone-02-stable606/mknod-mode-rv-la.promotion-candidates.txt`, `.derived.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-mknod-vfs-regression-20260601T190520Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-mknod-vfs-regression-20260601T190623Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV targeted: 6 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA targeted: 6 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Targeted four-way report: 3 candidates: `mknod03`, `mknod04`, `mknod09`.
- RV regression subset: 26 PASS / 0 FAIL, no parser caveats.
- LA regression subset: 26 PASS / 0 FAIL, no parser caveats.

Non-LTP caveat: these are targeted LTP gates only. Full stable606 promotion and full all-minus-blacklist sweep were not run.

## fchownat symlink nofollow validation

Pre-fix evidence:

- In the post-proc-fd RV rescout, `fchownat02` failed both libcs because `AT_SYMLINK_NOFOLLOW` ownership changes were not visible through symlink `lstat`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fchownat02 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fchownat02 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-fchownat02-nofollow-20260601T191133Z.log \
  target/ltp-1000-milestone-02-stable606/la-fchownat02-nofollow-20260601T191212Z.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fchownat02,symlink01,symlink02,readlink01,readlinkat01,lstat01,lstat01_64,chown01,chown02,chown03,fchownat01,fchmodat01,chown04,fchown04,chmod07,fchmod02 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=fchownat02,symlink01,symlink02,readlink01,readlinkat01,lstat01,lstat01_64,chown01,chown02,chown03,fchownat01,fchmodat01,chown04,fchown04,chmod07,fchmod02 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV targeted raw/summary: `target/ltp-1000-milestone-02-stable606/rv-fchownat02-nofollow-20260601T191133Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA targeted raw/summary: `target/ltp-1000-milestone-02-stable606/la-fchownat02-nofollow-20260601T191212Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- Targeted four-way report: `target/ltp-1000-milestone-02-stable606/fchownat02-nofollow-rv-la.promotion-candidates.txt`, `.derived.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-fchownat-symlink-regression-20260601T191310Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-fchownat-symlink-regression-20260601T191417Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- Regression four-way report: `target/ltp-1000-milestone-02-stable606/fchownat-symlink-regression-rv-la.promotion-candidates.txt`, `.derived.sha256`

Parser summary:

- RV targeted: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA targeted: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Targeted four-way report: 1 candidate, `fchownat02`.
- RV regression subset: 32 PASS / 0 FAIL, no parser caveats.
- LA regression subset: 32 PASS / 0 FAIL, no parser caveats.
- Regression four-way report: 16 clean rows across RV + LA x musl + glibc; `fchownat02` is the only newly banked row from this follow-up.

Non-LTP caveat: these are targeted LTP gates only. Full stable606 promotion and full all-minus-blacklist sweep were not run.

## setrlimit04 busybox applet exec validation

Pre-fix evidence:

- In the post-proc-fd RV rescout, `setrlimit04` failed both libcs because `execlp(/bin/true, /bin/true, ...)` returned `ENOENT`.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=setrlimit04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=setrlimit04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-setrlimit04-bin-true-20260601T191920Z.log \
  target/ltp-1000-milestone-02-stable606/la-setrlimit04-bin-true-20260601T191959Z.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=setrlimit01,setrlimit02,setrlimit03,setrlimit04,getrlimit01,getrlimit02,fork01,waitpid01,wait401,wait402,waitid01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=setrlimit01,setrlimit02,setrlimit03,setrlimit04,getrlimit01,getrlimit02,fork01,waitpid01,wait401,wait402,waitid01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV targeted raw/summary: `target/ltp-1000-milestone-02-stable606/rv-setrlimit04-bin-true-20260601T191920Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA targeted raw/summary: `target/ltp-1000-milestone-02-stable606/la-setrlimit04-bin-true-20260601T191959Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- Targeted four-way report: `target/ltp-1000-milestone-02-stable606/setrlimit04-bin-true-rv-la.promotion-candidates.txt`, `.derived.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-setrlimit-exec-regression-20260601T192057Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-setrlimit-exec-regression-20260601T192159Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- Regression four-way report: `target/ltp-1000-milestone-02-stable606/setrlimit-exec-regression-rv-la.promotion-candidates.txt`, `.derived.sha256`

Parser summary:

- RV targeted: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA targeted: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Targeted four-way report: 1 candidate, `setrlimit04`.
- RV regression subset: 22 PASS / 0 FAIL, no parser caveats.
- LA regression subset: 22 PASS / 0 FAIL, no parser caveats.
- Regression four-way report: 11 clean rows across RV + LA x musl + glibc; `setrlimit04` is the only newly banked row from this follow-up.

Non-LTP caveat: these are targeted LTP gates only. Full stable606 promotion and full all-minus-blacklist sweep were not run.


## clock_gettime04 evidence-only validation

Pre-follow-up evidence:

- `clock_gettime04` was clean for RV musl+glibc inside `rv-mm-time-followup-scout-20260601T192613Z.log`, while the same mixed scout retained real failures/TFAIL/TBROK/TCONF rows for other mm/wait/getcwd cases. Only the clean `clock_gettime04` row was eligible for further confirmation; the failed rows remain non-countable.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=clock_gettime04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES=clock_gettime04 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-clock-gettime04-rescout-20260601T193254Z.log \
  target/ltp-1000-milestone-02-stable606/la-clock-gettime04-rescout-20260601T192915Z.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES=clock_gettime02,clock_gettime04,gettimeofday01,gettimeofday02,times01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=clock_gettime02,clock_gettime04,gettimeofday01,gettimeofday02,times01 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV targeted raw/summary: `target/ltp-1000-milestone-02-stable606/rv-clock-gettime04-rescout-20260601T193254Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA targeted raw/summary: `target/ltp-1000-milestone-02-stable606/la-clock-gettime04-rescout-20260601T192915Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- Isolated four-way report: `target/ltp-1000-milestone-02-stable606/clock-gettime04-isolated-rv-la.promotion-candidates.txt`, `.sha256`
- RV regression raw/summary: `target/ltp-1000-milestone-02-stable606/rv-clock-time-regression-20260601T193006Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA regression raw/summary: `target/ltp-1000-milestone-02-stable606/la-clock-time-regression-20260601T193006Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- Regression four-way report: `target/ltp-1000-milestone-02-stable606/clock-time-regression-rv-la.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV targeted: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA targeted: 2 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Isolated four-way report: 1 candidate, `clock_gettime04`.
- RV regression subset: 10 PASS / 0 FAIL, no parser caveats.
- LA regression subset: 10 PASS / 0 FAIL, no parser caveats.
- Regression four-way report: 5 clean rows across RV + LA x musl + glibc; `clock_gettime04` is the only newly banked row from this follow-up.

Non-LTP caveat: these are targeted LTP gates only. Full stable606 promotion and full all-minus-blacklist sweep were not run.


## legacy clean-tail evidence-only validation

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=locktests,ltpServer,stress LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=locktests,ltpServer,stress LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl \
  target/ltp-1000-milestone-02-stable606/rv-legacy-clean-tail-scout-20260601T194031Z.log \
  target/ltp-1000-milestone-02-stable606/la-legacy-clean-tail-scout-20260601T194116Z.log
```

Artifacts:

- RV raw/summary: `target/ltp-1000-milestone-02-stable606/rv-legacy-clean-tail-scout-20260601T194031Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA raw/summary: `target/ltp-1000-milestone-02-stable606/la-legacy-clean-tail-scout-20260601T194116Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- Four-way report: `target/ltp-1000-milestone-02-stable606/legacy-clean-tail-rv-la.promotion-candidates.txt`, `.sha256`

Parser summary:

- RV targeted: 6 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA targeted: 6 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Four-way report: 3 candidates: `locktests`, `ltpServer`, `stress`; blocked/incomplete 0.

Non-LTP caveat: these rows are helper/harness-style LTP binaries and were banked only as named cases. They do not prove broad stress, lock, or server semantics. Full stable606 promotion and full all-minus-blacklist sweep were not run.

## Non-countable follow-up scout validation

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=sched_setaffinity01,signal01,nice04,clone04,gethostname02,gethostid01,getpgid01,kill05,kill10 LTP_CASE_TIMEOUT_SECS=90 timeout 80m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=readlinkat02,readlink03,statx01,statx04,fcntl30,pipe15,pipe2_03,writev03,pwritev03 LTP_CASE_TIMEOUT_SECS=90 timeout 80m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=readlinkat02 LTP_CASE_TIMEOUT_SECS=90 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-02-stable606/rv-light-process-scout-20260601T193756Z.log
python3 scripts/ltp_summary.py target/ltp-1000-milestone-02-stable606/rv-vfs-fd-remainder-scout-20260601T194216Z.log
python3 scripts/ltp_summary.py target/ltp-1000-milestone-02-stable606/la-readlinkat02-rescout-20260601T194310Z.log
```

Artifacts:

- Process/signal/scheduler RV scout: `target/ltp-1000-milestone-02-stable606/rv-light-process-scout-20260601T193756Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- VFS/FD RV scout: `target/ltp-1000-milestone-02-stable606/rv-vfs-fd-remainder-scout-20260601T194216Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`
- LA readlinkat follow-up: `target/ltp-1000-milestone-02-stable606/la-readlinkat02-rescout-20260601T194310Z.log`, `.summary.txt`, `.summary.json`, `.promotion-candidates.txt`, `.sha256`

Parser summary and promotion decision:

- RV process/signal/scheduler scout: 0 PASS / 8 FAIL, internal `TFAIL=5`, `TBROK=3`, `TCONF=1`, timeout matches 1, panic/trap matches 1; `kill10` is UNKNOWN with panic/trap. Counted candidates: 0.
- RV VFS/FD remainder scout: 2 PASS / 16 FAIL, internal `TFAIL=4`, `TBROK=10`, `TCONF=2`; only `readlinkat02` was RV clean. Counted candidates: 0 from this batch until four-way proof exists.
- LA readlinkat02 follow-up: 1 PASS / 1 FAIL; LA musl has one `TFAIL`, so `readlinkat02` is blocked and not banked.

These failures remain visible and non-countable; no blacklist/SKIP/status0 evidence was used.


<!-- stable606-final-closure:start -->
## Final stable606 validation closure on 2026-06-02

### Live stable count

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
# 606 606 0
```

### Team-state reconciliation for stale nudges

```bash
omx team status complete-dev-1000ltp-c632b4a0
# No team state found for complete-dev-1000ltp-c632b4a0
```

The requested mailbox file under `/root/.omx-runs/run-20260601071305-5c04/.omx/state/team/complete-dev-1000ltp-c632b4a0/mailbox/leader-fixed.json` was absent. The durable session state reported `active=False`, `current_phase=cancelled`, `lifecycle_outcome=finished`, `run_outcome=finish`, `completed_at=2026-06-01T17:22:55.786Z`, so later worker nudges were treated as stale and no worker was assigned.

### Final RV full stable gate

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=120 timeout 180m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-02-stable606/rv-stable606-final-gate-20260601T200557Z.log
python3 scripts/ltp_summary.py --json target/ltp-1000-milestone-02-stable606/rv-stable606-final-gate-20260601T200557Z.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv --promotion-libcs glibc,musl target/ltp-1000-milestone-02-stable606/rv-stable606-final-gate-20260601T200557Z.log
```

Artifacts: raw log, summary, JSON, and promotion report share prefix `target/ltp-1000-milestone-02-stable606/rv-stable606-final-gate-20260601T200557Z`.

Parser summary:

- PASS LTP CASE: 1212
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 (`TCONF=4`, all `read02`)
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- Suite summaries: `ltp-musl` 606 passed / 0 failed; `ltp-glibc` 606 passed / 0 failed.

### First LA full gate: preserved non-promotion evidence

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=150 timeout 210m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-20260601T203354Z.log
```

This run exited 143 and is not promotion evidence. It had 579 `PASS LTP CASE`, 2 wrapper FAIL, 6 internal caveats (`TCONF=2`, `TBROK=4`), one timeout match, and no ENOSYS or panic/trap. The visible blockers were `rename14`, `kill02`, and a later stalled `times03` segment; they were re-run below.

Checksums for this non-promotion evidence:

- `b5c6263c13b28b7c1c1b06d790f0356ed3922d5afd47b25901745cdc7b87ce4b  target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-20260601T203354Z.log`
- `c6290bd30f2083bae9a78c4712301a5e04a34ede21829d9ff8b097789983163c  target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-20260601T203354Z.summary.txt`
- `04507f03386c2e7dc7e01ca42c2f5fd5a90aa9cde6e5bdf847151fe4e9b33056  target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-20260601T203354Z.summary.json`
- `de38605ec712b43b45f9b7857015f95c9a0256b68706f1a8b06e0c6826ab4ce3  target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-20260601T203354Z.promotion-candidates.txt`

### LA recovery evidence after first full-gate stall

- `la-regression-rename14-kill02-times03-20260601T210237Z.log`: 6 `PASS LTP CASE`, 0 wrapper FAIL, 0 internal TFAIL/TBROK/TCONF, 0 timeout, 0 ENOSYS, 0 panic/trap.
- `la-local-order-shard-457-489-20260601T210637Z.log`: 66 `PASS LTP CASE`, 0 wrapper FAIL, 0 internal TFAIL/TBROK/TCONF, 0 timeout, 0 ENOSYS, 0 panic/trap.
- Cases file for the shard: `target/ltp-1000-milestone-02-stable606/la-local-order-shard-457-489.cases`.

### Final LA full stable retry

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=180 timeout 240m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z.log
python3 scripts/ltp_summary.py --json target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la --promotion-libcs glibc,musl target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z.log
```

Artifacts: raw log, summary, JSON, and promotion report share prefix `target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z`.

Parser summary:

- PASS LTP CASE: 1212
- FAIL LTP CASE: 0
- Internal TFAIL/TBROK/TCONF: 4 (`TCONF=4`, all `read02`)
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- Suite summaries: `ltp-musl` 606 passed / 0 failed; `ltp-glibc` 606 passed / 0 failed.

### Combined RV + LA promotion report

```bash
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la --promotion-libcs glibc,musl   target/ltp-1000-milestone-02-stable606/rv-stable606-final-gate-20260601T200557Z.log   target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z.log   > target/ltp-1000-milestone-02-stable606/stable606-final-rv-la.promotion-candidates.txt
```

- Combined report: 605 parser-clean candidates and one blocked/incomplete case, `read02`.
- Blocked reason: `read02` has `TCONF=2` in each of `la:glibc`, `la:musl`, `rv:glibc`, and `rv:musl`.
- This is an inherited stable506 caveat and is not a new stable606 regression; it remains disclosed rather than hidden.

Final checksums for the LA retry and combined report:

- `b02c163661e303034e8bf6ea81001ba901256e354f8a76a7ccfbd6b4babf31dd  target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z.log`
- `d0a87a10917776ba19d943bbc55f05803852096fc1ab65129c561f1c69b56941  target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z.summary.txt`
- `401d8031b42989eb4dae7b5e54df4db017f60edaf1f06c1a7bf856cac9261106  target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z.summary.json`
- `7ba78dc6fef66d679079adb79aa42173872fbf54528f93ebe11e2fa0c1a07dca  target/ltp-1000-milestone-02-stable606/la-stable606-final-gate-retry-20260601T211001Z.promotion-candidates.txt`
- `d6d8d8f0a7ada57b77f67375ea1dccbd956734af511d6b16837fefeceeabdbae  target/ltp-1000-milestone-02-stable606/stable606-final-rv-la.promotion-candidates.txt`
<!-- stable606-final-closure:end -->
