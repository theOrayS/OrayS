# milestone-02-stable606 promotion candidates preflight

## Status

This file records candidate evidence only. It is **not** a stable606 promotion list.

- Current stable: 556 total / 556 unique / 0 duplicate.
- Stable list changes: none.
- Required new unique cases for milestone-02: 50.
- Current candidate bank after this preflight: at most 26, so promotion is blocked.

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

## Promotion decision

Do not update `LTP_STABLE_CASES` yet. The candidate bank is short of 50 and some candidates need grouped regression confirmation.

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
