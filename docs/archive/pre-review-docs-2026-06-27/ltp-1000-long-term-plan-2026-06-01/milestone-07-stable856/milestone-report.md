# milestone-07 stable856 promotion report

Date: 2026-06-04.

## Target

Move the live baseline from stable806 to stable856 with 50 additional trusted unique LTP stable cases. This milestone is valid only if the new current pool and the full stable856 list pass RV + LA × musl + glibc gates with parser-visible evidence, without blacklist/SKIP/status0/full-sweep local TPASS credit.

## Result

Promoted **50** new unique cases. `examples/shell/src/cmd.rs::LTP_STABLE_CASES` is now:

```text
856 total / 856 unique / 0 duplicate
```

Current pool audit:

```text
pool_count 50
pool_unique 50
pool_duplicates 0
pool_sha256 ae97ecb3975f7fc79fbb29b2532828b48f39011fbca30bc5f628aea634bfcd42
```

Milestone-07 is promoted to stable856. The only parser-visible full-stable internal marker is the inherited `read02` `TCONF` caveat (`O_DIRECT not supported on tmpfs`) on both libcs/arches; the new 50-case pool is parser-clean with no `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`.

## Promoted cases

1. `getpgid01`
2. `tgkill03`
3. `setsid01`
4. `fsync03`
5. `read03`
6. `write04`
7. `kill05`
8. `mmap05`
9. `mmap08`
10. `mprotect01`
11. `msync03`
12. `fcntl31`
13. `fcntl31_64`
14. `mprotect03`
15. `utimes01`
16. `shmt05`
17. `shmctl08`
18. `shmctl07`
19. `fallocate01`
20. `capget01`
21. `capget02`
22. `capset01`
23. `capset02`
24. `capset03`
25. `capset04`
26. `sched_setscheduler04`
27. `setdomainname01`
28. `setdomainname02`
29. `setdomainname03`
30. `sched_getattr01`
31. `sched_setattr01`
32. `ioprio_get01`
33. `ioprio_set01`
34. `ioprio_set02`
35. `ioprio_set03`
36. `timer_delete01`
37. `timer_delete02`
38. `timer_getoverrun01`
39. `timer_gettime01`
40. `timer_settime02`
41. `msync01`
42. `msync02`
43. `statx04`
44. `statx12`
45. `setfsgid03`
46. `inode02`
47. `crash01`
48. `cve-2017-17052`
49. `nptl01`
50. `pth_str02`

## Code changes in this checkpoint

- Added the stable856 cases to `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Repaired/extended generic POSIX-visible areas needed by the pool:
  - MAP_SHARED file-backed mmap/msync writeback and LA `PROT_NONE` trapping semantics.
  - Process group/session, signal permission/validation, realtime signal queue `EAGAIN`, and `tgkill` validation.
  - Linux capabilities (`capget/capset`, bounding-set prctl), domainname, scheduling attributes, `ioprio_get/set`, and POSIX timers.
  - VFS/statx attributes, `utimensat` null-path `EFAULT`, fsync/fdatasync special-fd errno, `close_range`, `/dev/zero` flag handling, named FIFO/O_ASYNC/fcntl owner state.
  - SysV shm IPC_SET/LOCK/UNLOCK and SHM limit plumbing.
  - Lazy global table initialization race hardening in socket/futex/shm/task-registry paths.

No testsuite/evaluator bypass was made, and no LTP case/path/process/output hardcoding was introduced.

## Evidence summary

| Gate | Artifact | Parser result |
| --- | --- | --- |
| RV regression probe after msync EOF-tail fix | `target/ltp-1000-milestone-07-stable856/rv-regression-probe-mmap01-setpriority02-after-msync-eof-20260604T204927+0800/rv-summary.txt` | `4 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap` |
| LA regression probe after msync EOF-tail fix | `target/ltp-1000-milestone-07-stable856/la-regression-probe-mmap01-setpriority02-after-msync-eof-20260604T205035+0800/la-summary.txt` | `4 PASS / 0 FAIL / 0 internal markers` |
| RV new50 current-pool gate | `target/ltp-1000-milestone-07-stable856/rv-stable856-new50-currentpool-gate-20260604T205151+0800/rv-summary.txt` | `100 PASS / 0 FAIL / 0 internal markers` |
| LA new50 current-pool gate | `target/ltp-1000-milestone-07-stable856/la-stable856-new50-currentpool-gate-20260604T205650+0800/la-summary.txt` | `100 PASS / 0 FAIL / 0 internal markers` |
| Four-way promotion report | `target/ltp-1000-milestone-07-stable856/la-stable856-new50-currentpool-gate-20260604T205650+0800/fourway-promotion-candidates.txt` | `Promotion candidates: 50`, `Blocked/incomplete cases: 0` |
| Full RV stable856 final gate | `target/ltp-1000-milestone-07-stable856/rv-stable856-final-gate-20260604T210444+0800/rv-summary.txt` | `1712 PASS / 0 FAIL`; inherited `read02` `TCONF` only |
| Full LA stable856 final gate | `target/ltp-1000-milestone-07-stable856/la-stable856-final-gate-20260604T215235+0800/la-summary.txt` | `1712 PASS / 0 FAIL`; inherited `read02` `TCONF` only |

See `validation.md` and `validation-checksums.sha256` for commands and checksums.

## Rejected evidence / caveats

- `nice04` was **not** promoted. An intermediate EPERM-only `setpriority` experiment made `nice04` cleaner but regressed existing stable `setpriority02`; that change was reverted, and `inode02` replaced `nice04` in the final current pool.
- The earlier partial/aborted full RV gate under `rv-stable856-final-gate-20260604T203229+08:00` is blocker history only; it is not promotion evidence.
- `read02` remains an inherited stable caveat with `O_DIRECT not supported on tmpfs` `TCONF` inside wrapper-PASS rows. No new pool case relies on this caveat.

## Resource and lifetime observations

- `inode02` is the heaviest current-pool case: four-way report max runtime `44680 ms`, min free-frame delta `-16646`. It is clean but should remain a watchpoint for later stable-wide gates.
- `nptl01` max runtime reached `21951 ms` in the current-pool four-way report.
- Capability rows showed transient free-frame deltas (`capget02` min `-4112`, `capset02` min `-4110`) while still parser-clean.
- `cve-2017-17052` remained clean with max runtime `8701 ms` and min delta `-661` in the current-pool four-way report.

## Conclusion

Milestone-07 reaches stable856 with 50 additional trusted unique stable cases. Promotion evidence is RV + LA × musl + glibc wrapper PASS for the new pool and full stable856 final gates, with no new `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`. Continue toward stable906 with `inode02`, thread/runtime, and capability resource deltas monitored as regression watchpoints.
