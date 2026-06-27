# milestone-01-stable556 ABI and behavior impact

## Code changes

This milestone changes the stable gate list only:

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` gains 50 unique case names.

No kernel syscall implementation, errno mapping, flag parsing, struct layout, file descriptor lifetime, signal delivery, futex behavior, mmap semantics, or user pointer copy-in/copy-out behavior was modified in this milestone patch.

## User-visible effect

- `LTP_CASES=stable` now enumerates 556 unique stable cases instead of 506.
- The evaluator will run these 50 additional cases for both `ltp-musl` and `ltp-glibc` wrappers.
- This is a promotion/list change, not a semantic kernel fix.

## POSIX/Linux surface covered by the promoted cases

| Surface | Cases | Behavior confidence boundary |
| --- | --- | --- |
| FD/fcntl record-lock and command variants | fcntl19_64, fcntl20, fcntl20_64, fcntl21, fcntl21_64, fcntl22_64 | Confirms current implementation satisfies these LTP lock/command scenarios; no new locking semantics were introduced. |
| VFS/path/metadata | fs_perms, ioctl_ns07, readdir01 | Confirms current permission helper, directory iteration, and ENOTTY ioctl behavior for these wrappers. |
| Time/select | clock_nanosleep02, poll02, pselect01, pselect01_64, settimeofday01, time-schedule | Confirms timing wrappers stay within LTP thresholds in targeted/final gates; timer slack/systemd-detect-virt TINFO lines are not failures. |
| Socket/net errno/readiness | accept01, listen01, socket02, socketpair02 | Confirms selected socket flag/error/readiness paths; does not claim broad TCP/UDP/UNIX socket completeness. |
| MM/resource/lifetime | data_space, dirty, mlockall01, mmap-corruption01, mmstress_dummy, page01, page02, sbrk02, stack_space, ulimit01 | Confirms current VM/resource behavior survives these stress/smoke tests; memory free-frame deltas remain documented in parser matrix. |
| System identity/process-visible | newuname01, utsname01, utsname04 | Confirms current uname/hostname/namespace-permission visible results for these tests. |
| User-libc/libm smoke | nextafter01, genacos, genasin, genatan, genceil, gencos, gencosh, genexp, genfabs, genfloor, genfmod, genj0, genj1, genldexp, genlgamma, genlog, genlog10, genpow | Counts as stable wrapper coverage only. It is not claimed as kernel semantic expansion beyond demonstrating the userspace/runtime path remains healthy. |

## Resource/lifetime risks

- `data_space`, `dirty`, `mmap-corruption01`, `page01`, `page02`, and `stack_space` have visible allocation/free-frame deltas in parser output; keep these in the stable regression set to detect lifetime regression.
- `poll02`, `pselect01`, `pselect01_64`, `clock_nanosleep02`, and `time-schedule` are timing-sensitive and may expose scheduler/timer jitter under overloaded hosts.
- `fs_perms` shows large free-frame delta in LA proof output, so future work should distinguish expected cached filesystem allocations from leaks before expanding adjacent metadata cases.

## Maintenance boundary

Future agents must not treat this milestone as proof that the whole lane is complete. It only promotes the exact case names listed in `targeted-cases.txt` after four-way parser-clean evidence. Adjacent cases with TCONF, TBROK, timeout, ENOSPC, fixture failures, or arch-empty behavior remain outside stable until separately fixed and revalidated.
