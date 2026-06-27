# milestone-02-stable606 promotion candidates and final decision

## Status

This file contains both historical candidate evidence and the final stable606 promotion decision.

- Current stable: 606 total / 606 unique / 0 duplicate.
- Stable list change: +50 unique cases from live baseline 556.
- Final RV gate: wrapper 606/606 on musl + glibc; parser 1212 PASS, 0 FAIL, 4 inherited `read02` TCONF, no timeout/ENOSYS/panic/trap.
- Final LA gate retry: wrapper 606/606 on musl + glibc; parser 1212 PASS, 0 FAIL, 4 inherited `read02` TCONF, no timeout/ENOSYS/panic/trap.
- Combined RV+LA report: 605 parser-clean rows plus the pre-existing `read02` TCONF caveat. No blacklist/SKIP/status0 evidence is counted.
- Earlier preflight "blocked" notes below are retained as traceability and are superseded by this final section.

## Final +50 promoted cases

- `modify_ldt01`
- `modify_ldt02`
- `modify_ldt03`
- `print_caps`
- `test_ioctl`
- `tst_kvcmp`
- `tst_ncpus`
- `tst_ncpus_conf`
- `tst_ncpus_max`
- `tst_supported_fs`
- `fanotify_child`
- `genload`
- `gensin`
- `gensinh`
- `gensqrt`
- `gentan`
- `gentanh`
- `geny0`
- `geny1`
- `tst_exit`
- `tst_hexdump`
- `socket01`
- `nanosleep01`
- `mmap04`
- `vma01`
- `times03`
- `mmap14`
- `mmap12`
- `open10`
- `creat08`
- `chmod07`
- `fchmod02`
- `access04`
- `chmod06`
- `chown04`
- `fchmod06`
- `fchown04`
- `pipe07`
- `mknod03`
- `mknod04`
- `mknod09`
- `fchownat02`
- `setrlimit04`
- `clock_gettime04`
- `locktests`
- `ltpServer`
- `stress`
- `fcntl30`
- `mknod01`
- `pipe15`

## Final non-counted rows

- `read02`: retained in stable as an inherited baseline caveat; not presented as newly parser-clean.
- `statx01`: still `TCONF` in the mixed mknod/statx/pipe run and not promoted.
- The first LA full gate with `rename14`/`kill02`/`times03` failures: preserved as non-promotion evidence and superseded by targeted LA recovery plus the fresh full LA retry.
- Any blacklist/SKIP/status0/full-sweep partial TPASS row: not counted.

## Deferred clean bank from milestone-01

These 21 cases were four-way clean in milestone-01 combined proof but intentionally deferred from stable556:

`modify_ldt01`, `modify_ldt02`, `modify_ldt03`, `print_caps`, `test_ioctl`, `tst_kvcmp`, `tst_ncpus`, `tst_ncpus_conf`, `tst_ncpus_max`, `tst_supported_fs`, `fanotify_child`, `genload`, `gensin`, `gensinh`, `gensqrt`, `gentan`, `gentanh`, `geny0`, `geny1`, `tst_exit`, `tst_hexdump`.

## New targeted candidate evidence

### socket01

- Pre-fix RV scout state: both musl and glibc failed with two internal TFAIL rows due generic socket errno mismatch.
- Fix: generic AF_INET socket errno handling in `sys_socket_bridge`; no case-specific special-casing.
- Current evidence:
  - `rv-socket01-postfix-20260601T160003Z.log`: RV musl + glibc PASS, parser-clean.
  - `la-socket01-postfix-20260601T160247Z.log`: LA musl + glibc PASS, parser-clean.
  - `socket01-rv-la-postfix.promotion-candidates.txt`: one four-way candidate, `socket01`.
- Adjacent regression evidence:
  - `socket-adjacent-rv-la-postfix.promotion-candidates.txt` shows `accept01`, `listen01`, `socket01`, `socket02`, `socketpair02` clean across RV + LA x musl + glibc.

### nanosleep01

- RV 80-case scout state: glibc PASS, musl failed one timing TFAIL at the 10ms interval.
- Isolated rescout evidence:
  - `rv-nanosleep01-rescout-20260601T160605Z.log`: RV musl + glibc PASS, parser-clean.
  - `la-nanosleep01-rescout-20260601T160721Z.log`: LA musl + glibc PASS, parser-clean.
  - `nanosleep01-rv-la-rescout.promotion-candidates.txt`: one four-way candidate, `nanosleep01`.
- Caveat: because the earlier grouped RV scout failed, keep this as tentative until a later grouped regression/promotion gate proves it is not timing-flaky.

## Not promoted / blocked examples from this preflight

- `nice04`: musl failed because `nice(-10)` surfaced `EACCES` while POSIX `nice()` expects `EPERM`; a kernel-only remap would also change direct Linux `setpriority()` semantics, so no shortcut was taken.
- `clone04`: musl TBROK/SIGSEGV in the 80-case scout; requires separate clone/thread diagnosis.
- `signal01`, `setitimer01`: timed out in both libcs in RV scout; not promotion candidates.
- `getdents02`, `sched_rr_get_interval03`, `setpriority01`: wrapper PASS but TCONF/ENOSYS caveats remain; not promotion candidates.
- VFS/device rows such as `openat02`, `mknodat02`, `rename03` remain blocked by real ENOSPC/device/mount/metadata behavior.

## Superseded pre-final promotion decision

This earlier preflight decision is superseded by the final stable606 closure above. `LTP_STABLE_CASES` is now updated to 606 after full RV + LA x musl + glibc gate evidence.

### mmap04 and vma01

- Pre-fix RV scout state:
  - `mmap04` failed because `/proc/self/maps` did not contain parseable dynamic mmap ranges.
  - `vma01` failed because adjacent mmap VMAs were not visible in `/proc/self/maps`.
- Fix: generic `UserProcess` mmap-region tracking and `/proc/self/maps` dynamic range emission; no case-specific special-casing.
- Current evidence:
  - `rv-proc-maps-mmap-vma-postfix2-20260601T162318Z.log`: RV musl + glibc PASS for `mmap04,vma01`, parser-clean.
  - `la-proc-maps-mmap-vma-postfix-20260601T162441Z.log`: LA musl + glibc PASS for `mmap04,vma01`, parser-clean.
  - `proc-maps-mmap-regression-rv-la.promotion-candidates.txt`: combined RV + LA x musl + glibc report; `mmap04` and `vma01` are four-way clean.
- Adjacent regression evidence:
  - `rv-proc-maps-mmap-regression-20260601T162607Z.log` and `la-proc-maps-mmap-regression-20260601T162755Z.log` both report 22 PASS / 0 FAIL for `mmap04,vma01` plus stable mmap/mincore/mprotect anchors.

Updated candidate-bank note before the time follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` = at most 25 plausible cases, still short of stable606.


### times03

- Pre-fix RV scout state: both musl and glibc failed six internal TFAIL rows because `times()` reported zero self and waited-child CPU counters.
- Fix: generic per-process `times()` accounting now records process start ticks, returns `USER_HZ` clock ticks, and accumulates waited-child user/system counters on `wait4`/`waitid`; no case-specific special-casing.
- Current evidence:
  - `rv-times03-postfix-20260601T164216Z.log`: RV musl + glibc PASS for `times03`, parser-clean.
  - `la-times03-postfix-20260601T164436Z.log`: LA musl + glibc PASS for `times03`, parser-clean.
  - `times03-rv-la-postfix.promotion-candidates.txt`: one four-way candidate, `times03`.
- Adjacent regression evidence:
  - `rv-times03-regression-20260601T164708Z.log` and `la-times03-regression-20260601T164956Z.log` both report 10 PASS / 0 FAIL for `times03` plus stable time anchors `times01`, `gettimeofday01`, `gettimeofday02`, and `clock_gettime02`.
  - `times03-regression-rv-la.promotion-candidates.txt` shows all five rows clean across RV + LA x musl + glibc; only `times03` is a new not-yet-stable candidate from this fix.

Updated candidate-bank note after the time follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` = at most 26 plausible cases, still short of stable606.


### mmap14

- Pre-fix RV scout state: both musl and glibc failed one internal TFAIL row because `/proc/self/status` reported `VmLck` as 0K after an anonymous `MAP_LOCKED` mmap.
- Fix: generic `UserMmapRegion` locked-mapping metadata plus `/proc/self/status` `VmLck` reporting; `MAP_LOCKED` mappings are eagerly populated and tracked until range removal/splitting. No case-specific special-casing.
- Current evidence:
  - `rv-mmap14-postfix-20260601T170355Z.log`: RV musl + glibc PASS for `mmap14`, parser-clean.
  - `la-mmap14-postfix-20260601T170553Z.log`: LA musl + glibc PASS for `mmap14`, parser-clean.
  - `mmap14-rv-la-postfix.promotion-candidates.txt`: one four-way candidate, `mmap14`.
- Adjacent regression evidence:
  - `rv-mmap14-regression-20260601T170753Z.log` and `la-mmap14-regression-20260601T171057Z.log` both report 24 PASS / 0 FAIL for `mmap14` plus mmap/proc stable anchors.
  - `mmap14-regression-rv-la.promotion-candidates.txt` shows all twelve rows clean across RV + LA x musl + glibc; only `mmap14` is a new not-yet-stable candidate from this follow-up.

Updated candidate-bank note after the mmap14 follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` = at most 27 plausible cases, still short of stable606.

### mmap12

- Pre-fix RV scout state: both musl and glibc failed one internal TFAIL row because `/proc/self/pagemap` was absent (`ENOENT`).
- Fix: generic synthetic `/proc/self/pagemap` and `/proc/<pid>/pagemap` support with sparse read/lseek and present-bit snapshots from the current process mappings. No LTP case/path/process/output is hardcoded.
- Current evidence:
  - `rv-mmap12-postfix-20260601T173127Z.log`: RV musl + glibc PASS for `mmap12`, parser-clean.
  - `la-mmap12-postfix-20260601T173441Z.log`: LA musl + glibc PASS for `mmap12`, parser-clean.
  - `mmap12-rv-la-postfix.promotion-candidates.txt`: one four-way candidate, `mmap12`.
- Adjacent regression evidence:
  - `rv-mmap12-regression-20260601T174051Z.log` and `la-mmap12-regression-20260601T174435Z.log` both report 24 PASS / 0 FAIL for `mmap12` plus mmap/proc stable anchors.
  - `mmap12-regression-rv-la.promotion-candidates.txt` shows all twelve rows clean across RV + LA x musl + glibc; only `mmap12` is the new not-yet-stable candidate from this follow-up.

Updated candidate-bank note after the mmap12 follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` = at most 28 plausible cases, still short of stable606.

### open10 and creat08

- Pre-fix RV scout state: both musl and glibc failed in `dir_b/*` rows because files created under a setgid parent directory kept the process gid (`65534`) instead of inheriting the parent directory gid (`1`).
- Fix: generic create-path metadata recording now inherits gid from setgid parent directories for `open(O_CREAT)`, `creat()`, `mkdirat()`, and `mknodat()` paths; new subdirectories also inherit the setgid bit. No case-specific special-casing.
- Current evidence:
  - `rv-open-creat-setgid-postfix-20260601T180048Z.log`: RV musl + glibc PASS for `open10,creat08`, parser-clean.
  - `la-open-creat-setgid-postfix-20260601T180132Z.log`: LA musl + glibc PASS for `open10,creat08`, parser-clean.
  - `open-creat-setgid-rv-la-postfix.promotion-candidates.txt`: two four-way candidates, `open10` and `creat08`.
- Adjacent regression evidence:
  - `rv-open-creat-setgid-regression-20260601T180236Z.log` and `la-open-creat-setgid-regression-20260601T180348Z.log` both report 32 PASS / 0 FAIL for `open10,creat08` plus stable VFS metadata anchors.
  - `open-creat-setgid-regression-rv-la.promotion-candidates.txt` shows all sixteen rows clean across RV + LA x musl + glibc; only `open10` and `creat08` are new relative to the current stable list.

Updated candidate-bank note after the open/creat setgid follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` = at most 30 plausible cases, still short of stable606.


### chmod07 and fchmod02

- Pre-fix RV scout state: both cases failed in both musl and glibc during setup because `/etc/group` did not contain `users`, and the fallback `daemon` group was also absent (`getgrnam(...)` TBROK).
- Fix: generic synthetic `/etc/group` content now includes conventional `daemon` and `users` groups alongside `root` and `nogroup`. No case-specific special-casing.
- Current evidence:
  - `rv-groupdb-chmod-fchmod-20260601T181203Z.log`: RV musl + glibc PASS for `chmod07,fchmod02`, parser-clean.
  - `la-groupdb-chmod-fchmod-20260601T181243Z.log`: LA musl + glibc PASS for `chmod07,fchmod02`, parser-clean.
  - `groupdb-chmod-fchmod-rv-la.promotion-candidates.txt`: two four-way candidates, `chmod07` and `fchmod02`.
- Adjacent regression evidence:
  - `rv-groupdb-chmod-regression-20260601T181338Z.log` and `la-groupdb-chmod-regression-20260601T181429Z.log` both report 16 PASS / 0 FAIL for `chmod05,chmod07,fchmod02,chown01,chown02,chown03,open01,creat01`.
  - `groupdb-chmod-regression-rv-la.promotion-candidates.txt` shows all eight rows clean across RV + LA x musl + glibc; only `chmod07` and `fchmod02` are new relative to the current stable list.

Updated candidate-bank note after the group database follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` + `chmod07` + `fchmod02` = at most 32 plausible cases, still short of stable606.

### access04, chmod06, chown04, fchmod06, and fchown04

- Pre-fix RV scout state: all five cases TBROK or TFAIL around `mount(..., tmpfs, MS_REMOUNT|MS_RDONLY, ...)` and missing read-only filesystem errno semantics. `chown04` also exposed an inaccessible-prefix errno ordering issue.
- Fix: generic per-process mount metadata now records `MS_RDONLY` state on valid remounts, and VFS metadata/write-permission paths return `EROFS` for write-like operations under read-only mounts. Chown path handling checks parent search permission before ownership checks. No case-specific special-casing.
- Current evidence:
  - `rv-tmpfs-readonly-metadata-20260601T182849Z.log`: RV musl + glibc PASS for all five cases, parser-clean.
  - `la-tmpfs-readonly-metadata-20260601T182942Z.log`: LA musl + glibc PASS for all five cases, parser-clean.
  - `tmpfs-readonly-rv-la.promotion-candidates.txt`: five four-way candidates: `access04`, `chmod06`, `chown04`, `fchmod06`, `fchown04`.
- Adjacent regression evidence:
  - `rv-tmpfs-readonly-regression-20260601T183034Z.log` and `la-tmpfs-readonly-regression-20260601T183152Z.log` both report 30 PASS / 0 FAIL for the five new rows plus stable VFS/permission anchors.
  - `tmpfs-readonly-regression-rv-la.promotion-candidates.txt` shows all fifteen rows clean across RV + LA x musl + glibc; only `access04`, `chmod06`, `chown04`, `fchmod06`, and `fchown04` are new relative to the current stable list.

Updated candidate-bank note after the tmpfs read-only mount follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` + `chmod07` + `fchmod02` + `access04` + `chmod06` + `chown04` + `fchmod06` + `fchown04` = at most 37 plausible cases before the proc-fd follow-up, still short of stable606.


### pipe07

- Pre-fix RV scout state: both musl and glibc TBROK because `opendir(/proc/self/fd)` returned `ENOENT`; the test fills pipe fds until `EMFILE` and needs to count existing open fds via the procfs fd directory.
- Fix: generic dynamic synthetic directories for `/proc/self/fd`, `/proc/<current-pid>/fd`, and `/dev/fd`; `getdents64` enumerates current open fd numbers without special-casing LTP.
- Current evidence:
  - `rv-proc-fd-pipe07-20260601T184539Z.log`: RV musl + glibc PASS for `pipe07`, parser-clean.
  - `la-proc-fd-pipe07-20260601T184915Z.log`: LA musl + glibc PASS for `pipe07`, parser-clean.
  - `proc-fd-pipe07-rv-la.promotion-candidates.txt`: one four-way candidate, `pipe07`.
- Adjacent regression evidence:
  - `rv-proc-fd-regression-20260601T185013Z.log` and `la-proc-fd-regression-20260601T185013Z.log` both report 40 PASS / 0 FAIL for `pipe07` plus pipe/proc/readlink/fcntl anchors.
  - `proc-fd-regression-rv-la.promotion-candidates.txt` shows all twenty rows clean across RV + LA x musl + glibc; only `pipe07` is new relative to the current stable list.

Updated candidate-bank note after the proc-fd follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` + `chmod07` + `fchmod02` + `access04` + `chmod06` + `chown04` + `fchmod06` + `fchown04` + `pipe07` = at most 38 plausible cases, still short of stable606.

### mknod03, mknod04, and mknod09

- Pre-fix RV scout state:
  - `mknod03` and `mknod04` were already clean in the post-fix RV rescout, but still needed current LA confirmation before banking.
  - `mknod09` failed because invalid `mknod()` file-type bits returned `EPERM` where Linux reports `EINVAL`.
- Fix: generic `mknodat()` file-type validation now distinguishes unsupported privileged device nodes (`S_IFCHR`/`S_IFBLK` -> `EPERM`) from invalid mode encodings (`S_IFMT`, directory, symlink, socket, unknown type bits -> `EINVAL`). Regular files and FIFOs keep the previous behavior. No case-specific special-casing.
- Current evidence:
  - `rv-mknod-mode-rescout-20260601T190332Z.log`: RV musl + glibc PASS for `mknod03,mknod04,mknod09`, parser-clean.
  - `la-mknod-mode-rescout-20260601T190415Z.log`: LA musl + glibc PASS for `mknod03,mknod04,mknod09`, parser-clean.
  - `mknod-mode-rv-la.promotion-candidates.txt`: three four-way candidates: `mknod03`, `mknod04`, and `mknod09`.
- Adjacent regression evidence:
  - `rv-mknod-vfs-regression-20260601T190520Z.log` and `la-mknod-vfs-regression-20260601T190623Z.log` both report 26 PASS / 0 FAIL for `mknod03,mknod04,mknod09` plus stable/setgid/permission anchors.

Updated candidate-bank note after the mknod mode-errno follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` + `chmod07` + `fchmod02` + `access04` + `chmod06` + `chown04` + `fchmod06` + `fchown04` + `pipe07` + `mknod03` + `mknod04` + `mknod09` = at most 41 plausible cases, still short of stable606.

### fchownat02

- Pre-fix RV scout state: `fchownat02` failed nofollow symlink rows because `fchownat(..., AT_SYMLINK_NOFOLLOW)` updated the target metadata while subsequent `lstat` still showed the symlink owner/group as 0.
- Fix: generic `fchownat` nofollow handling now selects the final synthetic symlink path as the metadata record, and synthetic symlink `lstat` applies recorded owner/group metadata. Non-symlink and fd/empty-path behavior is unchanged.
- Current evidence:
  - `rv-fchownat02-nofollow-20260601T191133Z.log`: RV musl + glibc PASS for `fchownat02`, parser-clean.
  - `la-fchownat02-nofollow-20260601T191212Z.log`: LA musl + glibc PASS for `fchownat02`, parser-clean.
  - `fchownat02-nofollow-rv-la.promotion-candidates.txt`: one four-way candidate, `fchownat02`.
- Adjacent regression evidence:
  - `rv-fchownat-symlink-regression-20260601T191310Z.log` and `la-fchownat-symlink-regression-20260601T191417Z.log` both report 32 PASS / 0 FAIL for `fchownat02` plus symlink/readlink/lstat/chown/fchmod anchors.
  - `fchownat-symlink-regression-rv-la.promotion-candidates.txt` shows all sixteen rows clean across RV + LA x musl + glibc; only `fchownat02` is new relative to current stable and previously banked rows.

Updated candidate-bank note after the fchownat nofollow follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` + `chmod07` + `fchmod02` + `access04` + `chmod06` + `chown04` + `fchmod06` + `fchown04` + `pipe07` + `mknod03` + `mknod04` + `mknod09` + `fchownat02` = at most 42 plausible cases, still short of stable606.

### setrlimit04

- Pre-fix RV scout state: `setrlimit04` failed because `execlp(/bin/true, /bin/true, ...)` returned `ENOENT`.
- Fix: generic exec fallback for missing `/bin/<busybox-applet>` and `/usr/bin/<busybox-applet>` paths. The applet name must be in the existing busybox allowlist, real filesystem files still take precedence, and `argv[0]` is normalized to the applet basename.
- Current evidence:
  - `rv-setrlimit04-bin-true-20260601T191920Z.log`: RV musl + glibc PASS for `setrlimit04`, parser-clean.
  - `la-setrlimit04-bin-true-20260601T191959Z.log`: LA musl + glibc PASS for `setrlimit04`, parser-clean.
  - `setrlimit04-bin-true-rv-la.promotion-candidates.txt`: one four-way candidate, `setrlimit04`.
- Adjacent regression evidence:
  - `rv-setrlimit-exec-regression-20260601T192057Z.log` and `la-setrlimit-exec-regression-20260601T192159Z.log` both report 22 PASS / 0 FAIL for `setrlimit04` plus stable rlimit/fork/wait anchors.
  - `setrlimit-exec-regression-rv-la.promotion-candidates.txt` shows all eleven rows clean across RV + LA x musl + glibc; only `setrlimit04` is new relative to current stable and previously banked rows.

Updated candidate-bank note after the busybox applet exec follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` + `chmod07` + `fchmod02` + `access04` + `chmod06` + `chown04` + `fchmod06` + `fchown04` + `pipe07` + `mknod03` + `mknod04` + `mknod09` + `fchownat02` + `setrlimit04` = at most 43 plausible cases, still short of stable606.


### clock_gettime04

- Pre-follow-up state: the mixed RV mm/time scout showed `clock_gettime04` as a clean RV musl+glibc row, but many neighboring mm/wait/getcwd rows in the same scout had real TFAIL/TBROK/TCONF/failures and remain non-countable.
- Fix: none. This is evidence-only banking of an already-correct generic clock syscall path.
- Current evidence:
  - `rv-clock-gettime04-rescout-20260601T193254Z.log`: RV musl + glibc PASS for `clock_gettime04`, parser-clean.
  - `la-clock-gettime04-rescout-20260601T192915Z.log`: LA musl + glibc PASS for `clock_gettime04`, parser-clean.
  - `clock-gettime04-isolated-rv-la.promotion-candidates.txt`: one isolated four-way candidate, `clock_gettime04`.
- Adjacent regression evidence:
  - `rv-clock-time-regression-20260601T193006Z.log` and `la-clock-time-regression-20260601T193006Z.log` both report 10 PASS / 0 FAIL for `clock_gettime04` plus stable clock/gettimeofday/times anchors.
  - `clock-time-regression-rv-la.promotion-candidates.txt` shows all five rows clean across RV + LA x musl + glibc; only `clock_gettime04` is new relative to current stable and previously banked rows.

Updated candidate-bank note after the clock_gettime04 evidence-only follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` + `chmod07` + `fchmod02` + `access04` + `chmod06` + `chown04` + `fchmod06` + `fchown04` + `pipe07` + `mknod03` + `mknod04` + `mknod09` + `fchownat02` + `setrlimit04` + `clock_gettime04` = at most 44 plausible cases, still short of stable606.


### locktests, ltpServer, and stress

- Pre-follow-up state: these were the remaining clean-tail rows from archived scouting that were not present in the live stable556 list and not already banked for milestone-02.
- Fix: none. This is evidence-only banking of three named LTP helper/harness-style binaries after fresh four-way parser-clean validation.
- Current evidence:
  - `rv-legacy-clean-tail-scout-20260601T194031Z.log`: RV musl + glibc PASS for `locktests`, `ltpServer`, and `stress`, parser-clean.
  - `la-legacy-clean-tail-scout-20260601T194116Z.log`: LA musl + glibc PASS for all three cases, parser-clean.
  - `legacy-clean-tail-rv-la.promotion-candidates.txt`: three four-way candidates: `locktests`, `ltpServer`, and `stress`.
- Boundary: these rows are countable only as the named LTP cases. They are not a broad claim that all stress, locking, or server behavior is complete.

Updated candidate-bank note after the legacy clean-tail follow-up: 21 deferred rows + `socket01` + tentative `nanosleep01` + `mmap04` + `vma01` + `times03` + `mmap14` + `mmap12` + `open10` + `creat08` + `chmod07` + `fchmod02` + `access04` + `chmod06` + `chown04` + `fchmod06` + `fchown04` + `pipe07` + `mknod03` + `mknod04` + `mknod09` + `fchownat02` + `setrlimit04` + `clock_gettime04` + `locktests` + `ltpServer` + `stress` = at most 47 plausible cases, still short of stable606.

### Non-countable post-clock scouts

- `rv-light-process-scout-20260601T193756Z.log`: no clean rows; 0 PASS / 8 FAIL, `TFAIL=5`, `TBROK=3`, `TCONF=1`, timeout matches 1, panic/trap matches 1. `kill10` stayed UNKNOWN with panic/trap and is a blocker, not evidence.
- `rv-vfs-fd-remainder-scout-20260601T194216Z.log`: only `readlinkat02` was RV clean; the batch still had 16 wrapper FAIL rows and internal `TFAIL/TBROK/TCONF` caveats.
- `la-readlinkat02-rescout-20260601T194310Z.log`: LA glibc passed, but LA musl failed with one `TFAIL`; `readlinkat02` is not banked.
