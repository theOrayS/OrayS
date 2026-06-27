# milestone-01-stable556 promotion candidates

## Promotion rule used

A case is promotable only if all four wrappers are clean:

- RV musl wrapper PASS
- RV glibc wrapper PASS
- LA musl wrapper PASS
- LA glibc wrapper PASS
- `scripts/ltp_summary.py` reports no TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap for that case.

Blacklist, SKIP, status0/full-sweep-only local TPASS, TCONF, wrapper FAIL, timeout, and parser-unclean output were not counted.

## Four-way clean candidate pool

The combined proof report found 71 four-way clean candidates and 26 blocked/incomplete cases:

- Report: `target/ltp-1000-milestone-01-stable556/m01-proof-001-003-rv-la-promotion-candidates.txt`
- Checksum: `target/ltp-1000-milestone-01-stable556/m01-proof-001-003-rv-la-promotion-candidates.sha256`

## Selected 50 for stable556

Selected to balance semantic coverage and keep the milestone at exactly +50 unique cases:

| Lane | Promoted cases |
| --- | --- |
| FD/fcntl/io | fcntl19_64, fcntl20, fcntl20_64, fcntl21, fcntl21_64, fcntl22_64 |
| VFS/path/metadata | fs_perms, ioctl_ns07, readdir01 |
| Time/select | clock_nanosleep02, poll02, pselect01, pselect01_64, settimeofday01, time-schedule |
| Socket/net errno/readiness | accept01, listen01, socket02, socketpair02 |
| MM/resource/lifetime | data_space, dirty, mlockall01, mmap-corruption01, mmstress_dummy, page01, page02, sbrk02, stack_space, ulimit01 |
| System identity/process-visible | newuname01, utsname01, utsname04 |
| User-libc/libm smoke | nextafter01, genacos, genasin, genatan, genceil, gencos, gencosh, genexp, genfabs, genfloor, genfmod, genj0, genj1, genldexp, genlgamma, genlog, genlog10, genpow |

## Deferred clean cases

The following clean rows remain available for later scouting but were not promoted in this milestone:

- `modify_ldt01`, `modify_ldt02`, `modify_ldt03`: arch-empty/non-RISC-V/LoongArch semantic value is low despite wrapper clean output.
- `print_caps`, `test_ioctl`, `tst_kvcmp`, `tst_ncpus`, `tst_ncpus_conf`, `tst_ncpus_max`, `tst_supported_fs`: mostly harness/system-query smoke rows; defer until a broader proc/sys/resource lane can document semantics.
- `fanotify_child`, `genload`, `gensin`, `gensinh`, `gensqrt`, `gentan`, `gentanh`, `geny0`, `geny1`, `tst_exit`, `tst_hexdump`: clean but mostly user-libc or harness-adjacent; enough libm smoke was already selected to reach exactly +50.

## Explicit non-promotion list

- `openat02`, `openat03`: RV musl+glibc wrapper FAIL with TBROK in proof001.
- `access04`, `chmod06`, `chmod07`, `chown04`, `fchmod02`, `fchmod06`, `fchown04`, `fchownat02`, `mknod01`, `mknod03`, `mknod04`, `mknod07`, `mknod09`, `mknodat02`, `nice04`, `rename03`, `rename04`, `rename05`, `sched_rr_get_interval03`, `sched_setaffinity01`, `setpriority01`, `setrlimit04`, `signal01`: parser-unclean RV evidence or timeout/TCONF; not counted.
