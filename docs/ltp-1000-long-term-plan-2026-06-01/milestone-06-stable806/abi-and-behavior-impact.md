# milestone-06 ABI and behavior impact so far

This interim stable806 checkpoint made no source-code, syscall, errno, flag, struct-layout, FD, signal, futex, mmap, user-pointer, or blacklist changes.

Observed behavior from RV scouting is recorded as blocker evidence only:

- Missing timerslack support is externally visible through `prctl(PR_SET_TIMERSLACK/PR_GET_TIMERSLACK)` and `/proc/self/timerslack_ns` expectations in `prctl08`/`prctl09`.
- Missing POSIX timer syscalls are externally visible as `TCONF`/`TBROK`/`ENOSYS` in timer rows.
- Time/signal rows expose timeout/kill-cleanup behavior that needs isolated repair before promotion.
- Priority/nice rows show libc-specific errno and `TCONF` boundaries that must be resolved generically, not by case-name handling.

Because there was no code edit, no ABI/POSIX-visible behavior intentionally changed in this checkpoint.
