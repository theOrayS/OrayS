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


## chmod07 / fchmod02 group database regression set

Rationale: the code change affects synthetic `/etc/group` content consumed by libc group-name lookup. The protected subset combines the new candidates with stable chmod/chown/open/creat anchors that could observe group metadata or setup behavior.

Cases:

- New candidates: `chmod07`, `fchmod02`
- Existing stable anchors: `chmod05`, `chown01`, `chown02`, `chown03`, `open01`, `creat01`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-groupdb-chmod-regression-20260601T181338Z.summary.txt`
  - 16 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-groupdb-chmod-regression-20260601T181429Z.summary.txt`
  - 16 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/groupdb-chmod-regression-rv-la.promotion-candidates.txt`
  - All eight rows clean across RV + LA x musl + glibc; `chmod07` and `fchmod02` are the new candidate rows.

## tmpfs read-only mount metadata regression set

Rationale: the code change affects `mount` flag acceptance, per-process mount translation, read-only mount errno handling, and VFS metadata permission ordering. The protected subset combines the five new candidates with existing stable access/chmod/chown/open/creat anchors.

Cases:

- New candidates: `access04`, `chmod06`, `chown04`, `fchmod06`, `fchown04`
- Existing stable anchors: `access01`, `access02`, `chmod05`, `chmod07`, `fchmod02`, `chown01`, `chown02`, `chown03`, `open01`, `creat01`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-tmpfs-readonly-regression-20260601T183034Z.summary.txt`
  - 30 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-tmpfs-readonly-regression-20260601T183152Z.summary.txt`
  - 30 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/tmpfs-readonly-regression-rv-la.promotion-candidates.txt`
  - All fifteen rows clean across RV + LA x musl + glibc; `access04`, `chmod06`, `chown04`, `fchmod06`, and `fchown04` are the new candidate rows.


## /proc/self/fd / pipe07 regression set

Rationale: the code change affects synthetic procfs fd-directory open/stat/getdents behavior and could interact with pipe fd accounting, procfs path lookup, readlink expectations, and fcntl descriptor operations. The protected subset combines the new candidate with existing stable pipe, proc, readlink, and fcntl anchors.

Cases:

- New candidate: `pipe07`
- Existing stable anchors: `pipe01`, `pipe02`, `pipe03`, `pipe04`, `pipe05`, `pipe06`, `pipe08`, `pipe09`, `pipe10`, `pipe14`, `pipe2_01`, `pipe2_02`, `pipe2_04`, `proc01`, `readlink01`, `readlinkat01`, `fcntl01`, `fcntl02`, `fcntl03`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-proc-fd-regression-20260601T185013Z.summary.txt`
  - 40 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-proc-fd-regression-20260601T185013Z.summary.txt`
  - 40 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Combined report: `target/ltp-1000-milestone-02-stable606/proc-fd-regression-rv-la.promotion-candidates.txt`
  - All twenty rows clean across RV + LA x musl + glibc; `pipe07` is the new candidate row.

## mknod mode errno regression set

Rationale: the code change affects `mknodat()` file-type validation and errno ordering before node creation. The protected subset combines the new mknod candidates with existing mknod/open/creat/chmod/chown anchors and already-banked VFS metadata candidates.

Cases:

- New candidates: `mknod03`, `mknod04`, `mknod09`
- Existing stable or already-banked anchors: `mknod02`, `open10`, `creat08`, `chmod07`, `fchmod02`, `access04`, `chmod06`, `chown04`, `fchmod06`, `fchown04`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-mknod-vfs-regression-20260601T190520Z.summary.txt`
  - 26 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-mknod-vfs-regression-20260601T190623Z.summary.txt`
  - 26 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Targeted four-way report: `target/ltp-1000-milestone-02-stable606/mknod-mode-rv-la.promotion-candidates.txt`
  - `mknod03`, `mknod04`, and `mknod09` are clean across RV + LA x musl + glibc.

## fchownat symlink nofollow regression set

Rationale: the code change affects `fchownat(..., AT_SYMLINK_NOFOLLOW)` path selection and synthetic symlink `lstat` ownership reporting. The protected subset combines the new candidate with existing symlink/readlink/lstat/chown/fchmod anchors and already-banked metadata candidates.

Cases:

- New candidate: `fchownat02`
- Existing stable or already-banked anchors: `symlink01`, `symlink02`, `readlink01`, `readlinkat01`, `lstat01`, `lstat01_64`, `chown01`, `chown02`, `chown03`, `fchownat01`, `fchmodat01`, `chown04`, `fchown04`, `chmod07`, `fchmod02`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-fchownat-symlink-regression-20260601T191310Z.summary.txt`
  - 32 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-fchownat-symlink-regression-20260601T191417Z.summary.txt`
  - 32 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Targeted four-way report: `target/ltp-1000-milestone-02-stable606/fchownat02-nofollow-rv-la.promotion-candidates.txt`
  - `fchownat02` is clean across RV + LA x musl + glibc.
- Regression four-way report: `target/ltp-1000-milestone-02-stable606/fchownat-symlink-regression-rv-la.promotion-candidates.txt`
  - All sixteen rows clean across RV + LA x musl + glibc; only `fchownat02` is new relative to current stable and previously banked rows.

## setrlimit04 busybox applet exec regression set

Rationale: the code change affects exec path compatibility for missing `/bin`/`/usr/bin` busybox applets. The protected subset combines the new candidate with stable rlimit, fork, wait, and waitid anchors that exercise process lifetime after exec/fork.

Cases:

- New candidate: `setrlimit04`
- Existing stable anchors: `setrlimit01`, `setrlimit02`, `setrlimit03`, `getrlimit01`, `getrlimit02`, `fork01`, `waitpid01`, `wait401`, `wait402`, `waitid01`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-setrlimit-exec-regression-20260601T192057Z.summary.txt`
  - 22 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-setrlimit-exec-regression-20260601T192159Z.summary.txt`
  - 22 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Targeted four-way report: `target/ltp-1000-milestone-02-stable606/setrlimit04-bin-true-rv-la.promotion-candidates.txt`
  - `setrlimit04` is clean across RV + LA x musl + glibc.
- Regression four-way report: `target/ltp-1000-milestone-02-stable606/setrlimit-exec-regression-rv-la.promotion-candidates.txt`
  - All eleven rows clean across RV + LA x musl + glibc; only `setrlimit04` is new relative to current stable and previously banked rows.


## clock_gettime04 / clock-time evidence-only regression set

Rationale: no code changed in this follow-up, but the case belongs to the time syscall lane and was protected with adjacent stable clock/gettimeofday/times anchors before banking.

Cases:

- New candidate: `clock_gettime04`
- Existing stable anchors: `clock_gettime02`, `gettimeofday01`, `gettimeofday02`, `times01`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-clock-time-regression-20260601T193006Z.summary.txt`
  - 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-clock-time-regression-20260601T193006Z.summary.txt`
  - 10 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Isolated targeted four-way report: `target/ltp-1000-milestone-02-stable606/clock-gettime04-isolated-rv-la.promotion-candidates.txt`
  - `clock_gettime04` is clean across RV + LA x musl + glibc.
- Regression four-way report: `target/ltp-1000-milestone-02-stable606/clock-time-regression-rv-la.promotion-candidates.txt`
  - All five rows clean across RV + LA x musl + glibc; only `clock_gettime04` is new relative to current stable and previously banked rows.


## legacy clean-tail evidence-only matrix

Rationale: no code changed, so there is no new syscall-lane regression set. The protected claim is limited to fresh four-way parser-clean proof for three named helper/harness-style LTP binaries.

Cases:

- New candidates: `locktests`, `ltpServer`, `stress`

Evidence:

- RV: `target/ltp-1000-milestone-02-stable606/rv-legacy-clean-tail-scout-20260601T194031Z.summary.txt`
  - 6 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- LA: `target/ltp-1000-milestone-02-stable606/la-legacy-clean-tail-scout-20260601T194116Z.summary.txt`
  - 6 PASS / 0 FAIL, no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap.
- Four-way report: `target/ltp-1000-milestone-02-stable606/legacy-clean-tail-rv-la.promotion-candidates.txt`
  - `locktests`, `ltpServer`, and `stress` are clean across RV + LA x musl + glibc.

## Non-countable scout blockers

- `rv-light-process-scout-20260601T193756Z.summary.txt`: 0 PASS / 8 FAIL with TFAIL/TBROK/TCONF, one timeout, and one panic/trap. No regression protection or candidate credit is claimed.
- `rv-vfs-fd-remainder-scout-20260601T194216Z.summary.txt`: 2 PASS / 16 FAIL with TFAIL/TBROK/TCONF; the only RV-clean row, `readlinkat02`, failed LA musl in `la-readlinkat02-rescout-20260601T194310Z.summary.txt` and remains blocked.
