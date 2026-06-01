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
