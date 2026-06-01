# milestone-02-stable606 regression matrix preflight

## Stable baseline

- Current stable count: 556 total / 556 unique / 0 duplicate.
- Stable list changed in this preflight: no.

## Socket errno fix regression set

Rationale: the code change affects `sys_socket_bridge` errno behavior. The protected adjacent subset combines the newly fixed `socket01` with existing stable socket-adjacent cases.

Cases:

- `socket01` (new candidate)
- `socket02` (existing stable)
- `socketpair02` (existing stable)
- `accept01` (existing stable)
- `listen01` (existing stable)

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-socket-adjacent-postfix-20260601T160853Z.summary.txt`
  - 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-socket-adjacent-postfix-20260601T160953Z.summary.txt`
  - 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/socket-adjacent-rv-la-postfix.promotion-candidates.txt`
  - All five cases clean across RV + LA x musl + glibc.

## Time lane rescout

Case: `nanosleep01`.

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-nanosleep01-rescout-20260601T160605Z.summary.txt`
  - 2 PASS / 0 FAIL, no parser caveats.
- LA: `target/ltp-1000-milestone-02-stable606/la-nanosleep01-rescout-20260601T160721Z.summary.txt`
  - 2 PASS / 0 FAIL, no parser caveats.
- Combined report: `target/ltp-1000-milestone-02-stable606/nanosleep01-rv-la-rescout.promotion-candidates.txt`
  - `nanosleep01` clean across RV + LA x musl + glibc.

Caveat: the earlier grouped RV scout had one musl timing TFAIL, so this row needs grouped revalidation before final promotion.

## Not run yet

- Full stable606 RV + LA x musl + glibc gate.
- Broad stable regression beyond the socket-adjacent subset.
- LA full-sweep shard from G010.

## Proc maps / mmap regression set

Rationale: the code change affects synthetic `/proc/self/maps` output and per-process mmap metadata. The protected subset combines new candidates with existing stable mmap/mincore/mprotect anchors.

Cases:

- New candidates: `mmap04`, `vma01`
- Existing stable anchors: `mmap01`, `mmap02`, `mmap03`, `mmap06`, `mmap09`, `mmap10`, `mmap11`, `mincore01`, `mprotect05`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-proc-maps-mmap-regression-20260601T162607Z.summary.txt`
  - 22 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-proc-maps-mmap-regression-20260601T162755Z.summary.txt`
  - 22 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/proc-maps-mmap-regression-rv-la.promotion-candidates.txt`
  - All eleven rows clean across RV + LA x musl + glibc; `mmap04` and `vma01` are the new candidate rows.


## times03 / time accounting regression set

Rationale: the code change affects `times()` return units and `struct tms` self/child counters. The protected subset combines the new candidate with existing stable time anchors that exercise adjacent time syscalls.

Cases:

- New candidate: `times03`
- Existing stable anchors: `times01`, `gettimeofday01`, `gettimeofday02`, `clock_gettime02`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-times03-regression-20260601T164708Z.summary.txt`
  - 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-times03-regression-20260601T164956Z.summary.txt`
  - 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/times03-regression-rv-la.promotion-candidates.txt`
  - All five rows clean across RV + LA x musl + glibc; `times03` is the new candidate row.


## mmap14 / MAP_LOCKED VmLck regression set

Rationale: the code change affects `MAP_LOCKED` mmap metadata, range splitting/removal, eager population, and synthetic `/proc/self/status` `VmLck` output. The protected subset extends the prior proc-maps regression with `mmap14`.

Cases:

- New candidate: `mmap14`
- Already banked candidates: `mmap04`, `vma01`
- Existing stable anchors: `mmap01`, `mmap02`, `mmap03`, `mmap06`, `mmap09`, `mmap10`, `mmap11`, `mincore01`, `mprotect05`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-mmap14-regression-20260601T170753Z.summary.txt`
  - 24 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-mmap14-regression-20260601T171057Z.summary.txt`
  - 24 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/mmap14-regression-rv-la.promotion-candidates.txt`
  - All twelve rows clean across RV + LA x musl + glibc; `mmap14` is the new candidate row.

## mmap12 /proc/self/pagemap regression set

Rationale: the code change affects synthetic `/proc/self/pagemap` path lookup plus fd `read/lseek/stat` behavior. The protected subset combines the new pagemap candidate with existing mmap/proc maps, mincore, mprotect, and locked-mmap anchors.

Cases:

- New candidate: `mmap12`
- Already banked candidates: `mmap04`, `vma01`, `mmap14`
- Existing stable anchors: `mmap01`, `mmap02`, `mmap03`, `mmap06`, `mmap09`, `mmap11`, `mincore01`, `mprotect05`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-mmap12-regression-20260601T174051Z.summary.txt`
  - 24 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-mmap12-regression-20260601T174435Z.summary.txt`
  - 24 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/mmap12-regression-rv-la.promotion-candidates.txt`
  - All twelve rows clean across RV + LA x musl + glibc; `mmap12` is the new candidate row.

## open10 / creat08 setgid create regression set

Rationale: the code change affects metadata recorded after successful file, FIFO, and directory creation under setgid parent directories. The protected subset combines the new candidates with stable open/creat/chmod/chown/mkdir/mknod anchors.

Cases:

- New candidates: `open10`, `creat08`
- Existing stable anchors: `open01`, `open03`, `open08`, `open09`, `creat01`, `creat03`, `creat04`, `creat05`, `chmod05`, `chown01`, `chown02`, `chown03`, `mkdir04`, `mknod02`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-open-creat-setgid-regression-20260601T180236Z.summary.txt`
  - 32 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-open-creat-setgid-regression-20260601T180348Z.summary.txt`
  - 32 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/open-creat-setgid-regression-rv-la.promotion-candidates.txt`
  - All sixteen rows clean across RV + LA x musl + glibc; `open10` and `creat08` are the new candidate rows.
