# milestone-01-stable556 report

## Goal

Promote the live stable baseline from 506 to 556 trusted unique LTP stable cases on `dev/1000ltp-plan`, preserving honest evidence rules and leaving blocked/TCONF/timeout rows visible.

## Result

- Stable list target: 556 total / 556 unique / 0 duplicate.
- Source update: `examples/shell/src/cmd.rs::LTP_STABLE_CASES` appends exactly 50 cases.
- Milestone evidence directory: `target/ltp-1000-milestone-01-stable556/`.
- Milestone docs directory: `docs/ltp-1000-long-term-plan-2026-06-01/milestone-01-stable556/`.
- Final RV/LA gate result: RV + LA final stable556 gates completed with wrapper PASS 556/0 for both musl and glibc on both arches. `scripts/ltp_summary.py` reports 1112 PASS per arch, 0 FAIL, 0 TFAIL/TBROK/timeout/ENOSYS/panic/trap; only inherited `read02` TCONF remains (2 per libc) and no newly promoted case introduces TCONF.

## Promoted cases

See `targeted-cases.txt` for the full candidate/proof matrix. The 50 promoted cases are:

`accept01`, `clock_nanosleep02`, `data_space`, `dirty`, `fcntl19_64`, `fcntl20`, `fcntl20_64`, `fcntl21`, `fcntl21_64`, `fcntl22_64`, `fs_perms`, `ioctl_ns07`, `listen01`, `mlockall01`, `mmap-corruption01`, `mmstress_dummy`, `newuname01`, `page01`, `page02`, `poll02`, `pselect01`, `pselect01_64`, `readdir01`, `sbrk02`, `settimeofday01`, `socket02`, `socketpair02`, `stack_space`, `time-schedule`, `ulimit01`, `utsname01`, `utsname04`, `nextafter01`, `genacos`, `genasin`, `genatan`, `genceil`, `gencos`, `gencosh`, `genexp`, `genfabs`, `genfloor`, `genfmod`, `genj0`, `genj1`, `genldexp`, `genlgamma`, `genlog`, `genlog10`, `genpow`.

## Evidence summary

- Sanity non-TTY RV access01 gate: parser clean.
- Proof001: 20 four-way clean semantic candidates promoted; `openat02`/`openat03` blocked by RV TBROK and excluded.
- Proof002: 12 four-way clean candidates promoted; TCONF/TBROK/TFAIL/timeout rows excluded.
- Proof003: 18 low-risk libm/user-libc smoke cases promoted; extra harness-like rows deferred.
- Combined proof report: 71 four-way clean candidates / 26 blocked-incomplete cases.
- Final `LTP_CASES=stable` RV+LA gates: RV + LA final stable556 gates completed with wrapper PASS 556/0 for both musl and glibc on both arches. `scripts/ltp_summary.py` reports 1112 PASS per arch, 0 FAIL, 0 TFAIL/TBROK/timeout/ENOSYS/panic/trap; only inherited `read02` TCONF remains (2 per libc) and no newly promoted case introduces TCONF.

## ABI/POSIX/user-visible impact

This milestone changes the stable-list surface only. It does not change kernel syscall implementations, errno mappings, flags, FD lifetime, signal/futex/mmap behavior, or user pointer copy-in/copy-out. See `abi-and-behavior-impact.md` for lane-specific boundaries.

## Risk and caveats

- Existing stable506 evidence caveat around `read02` TCONF remains visible and is not hidden.
- The added cases themselves must be parser-clean in both final stable gates; any final failure blocks the commit.
- Libm generator rows improve wrapper/userspace smoke coverage but are not claimed as kernel semantic expansion.
- No blacklist change is made; see `blacklist-change-report.md`.

## Next step

After this milestone commit, continue to stable606 with emphasis on VFS metadata/path and FD/pipe/io cases that require real semantic fixes rather than harness-only promotions.
