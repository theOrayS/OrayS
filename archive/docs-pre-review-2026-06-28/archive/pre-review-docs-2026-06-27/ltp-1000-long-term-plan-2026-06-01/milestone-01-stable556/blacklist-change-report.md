# milestone-01-stable556 blacklist change report

## Summary

No blacklist file or blacklist selection rule was changed in this milestone.

## Severe-blocker policy

- No ordinary failing case was hidden behind blacklist for promotion.
- `signal01` timeout, `sched_rr_get_interval03`/`setpriority01` TCONF, `openat02`/`openat03` TBROK, and VFS metadata fixture failures remain visible in proof evidence and are not counted.
- Existing blacklist/all-minus-blacklist scouting rules remain unchanged and are not part of this stable556 promotion proof.

## Unlock conditions for blocked cases

Blocked cases may be reconsidered only after a focused fix plus fresh RV + LA × musl + glibc parser-clean evidence. A local status0, SKIP, TCONF, blacklist exclusion, or partial wrapper pass is not sufficient.
