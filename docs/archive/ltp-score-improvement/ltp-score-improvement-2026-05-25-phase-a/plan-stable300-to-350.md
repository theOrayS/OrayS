# LTP stable300 -> stable350 Ultragoal Plan

Date: 2026-05-25
Status: **delivered stable350**

## Goal and non-negotiable gates

Raise live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` from stable300 to stable350 without fake pass semantics.

Completion gates:

- live stable list is exactly 350 total / 350 unique / 0 duplicates;
- RV final stable aggregate: `PASS LTP CASE: 700`, `FAIL LTP CASE: 0`, `ltp-musl 350/0`, `ltp-glibc 350/0`;
- LA final stable aggregate: `PASS LTP CASE: 700`, `FAIL LTP CASE: 0`, `ltp-musl 350/0`, `ltp-glibc 350/0`;
- no internal TFAIL/TBROK, no timeout, no ENOSYS/not-implemented, no panic/trap;
- `read02` pass-with-TCONF remains explicitly disclosed and is the only known TCONF source;
- wrapper markers remain line-prefix clean: `PASS LTP CASE` / `FAIL LTP CASE` at column 0, 0 bad lines;
- final reports, ai-slop-cleaner audit, and code-review report are written before final Ultragoal completion.

## Phased promotion strategy

1. **stable315**: promote 15 cases only after RV+LA aggregate gates stayed clean.
2. **stable330**: promote the next 15 cases using fchdir/fcntl/readlinkat/sched/symlink evidence.
3. **stable350**: promote the final 20 cases, demote near-misses, then run full final RV+LA stable gates.

## Promotion batches

### stable315 additions
- `alarm05`
- `alarm07`
- `write05`
- `gettimeofday02`
- `waitpid01`
- `pipe2_02`
- `sched_getscheduler02`
- `fstat03`
- `fstat03_64`
- `statfs02`
- `fstatfs02`
- `fstatfs02_64`
- `sched_getparam03`
- `sched_setparam04`
- `sched_setparam05`

Evidence:

- `raw/stable315-rv-aggregate-002-summary.txt`: PASS 630 / FAIL 0; musl 315/0; glibc 315/0; read02 TCONF only; timeout/ENOSYS/panic 0.
- `raw/stable315-la-aggregate-001-summary.txt`: PASS 630 / FAIL 0; musl 315/0; glibc 315/0; read02 TCONF only; timeout/ENOSYS/panic 0.

### stable330 additions
- `fchdir01`
- `fchdir03`
- `fcntl05`
- `fcntl05_64`
- `fcntl12`
- `fcntl12_64`
- `fcntl13`
- `fcntl13_64`
- `fdatasync01`
- `fdatasync02`
- `readlinkat01`
- `sched_setscheduler01`
- `sched_setscheduler02`
- `symlinkat01`
- `ftruncate03_64`

Evidence:

- `raw/stable330-rv-aggregate-002-summary.txt`: PASS 660 / FAIL 0; musl 330/0; glibc 330/0; read02 TCONF only; timeout/ENOSYS/panic 0.
- `raw/stable330-la-aggregate-001-summary.txt`: PASS 660 / FAIL 0; musl 330/0; glibc 330/0; read02 TCONF only; timeout/ENOSYS/panic 0.

### stable350 additions
- `chdir04`
- `chown01`
- `chown02`
- `chown03`
- `chown05`
- `creat05`
- `abs01`
- `mkdir05`
- `statfs02_64`
- `truncate03_64`
- `fork03`
- `fork04`
- `fork07`
- `fork08`
- `fork09`
- `signal05`
- `string01`
- `memcmp01`
- `memcpy01`
- `memset01`

Final evidence:

- `raw/stable350-rv-final-002-summary.txt`: PASS 700 / FAIL 0; musl 350/0; glibc 350/0.
- `raw/stable350-la-final-002-summary.txt`: PASS 700 / FAIL 0; musl 350/0; glibc 350/0.
- `raw/stable350-rv-final-002-marker-prefix.txt`: `TOTAL markers=700 bad=0`.
- `raw/stable350-la-final-002-marker-prefix.txt`: `TOTAL markers=700 bad=0`.

## Key steering decision

`kill02` was initially attractive from targeted evidence but failed the LA final aggregate (`raw/stable350-la-final-summary.txt`) in glibc with TBROK setup failures. It was removed from stable and replaced with `abs01`, which has fresh RV+LA targeted replacement evidence and final aggregate evidence. This preserves the no-fake-pass/no-timeout-as-pass policy.
