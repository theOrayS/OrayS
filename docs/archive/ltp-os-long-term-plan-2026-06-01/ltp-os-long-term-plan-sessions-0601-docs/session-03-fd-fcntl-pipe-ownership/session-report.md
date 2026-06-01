# Session 3 report: FD/fcntl/pipe/ownership

Commit SHA: to be recorded after this session commit is created.
Previous session commit: `c1c5dcd5` (Session 2 time/select/signal).

## Goal

Expand the stable FD/fcntl/pipe surface with a real semantic fix, then promote only cases that pass RV/LA × musl/glibc parser-clean gates.

## Changes

- Added a generic POSIX advisory record-lock table for regular-file `fcntl` locks.
- Implemented conflict-aware `F_GETLK`, `F_SETLK`, and yielding `F_SETLKW` behavior.
- Added process-aware record-lock release on `close`, `dup3` replacement, and process teardown.
- Promoted `fcntl11`, `fcntl14`, `fcntl19`, and `fcntl22` into `LTP_STABLE_CASES`.
- Did not modify blacklist or evaluator/testsuite code.

## Evidence summary

- Live stable count after promotion: `466 total / 466 unique / 0 duplicate`.
- RV final combined promotion + adjacent regression gate: `PASS LTP CASE 26`, `FAIL 0`, internal `{}`, timeout `0`, ENOSYS `0`, panic/trap `0`.
- LA final combined promotion + adjacent regression gate: `PASS LTP CASE 26`, `FAIL 0`, internal `{}`, timeout `0`, ENOSYS `0`, panic/trap `0`.
- Combined gates cover the 4 promoted cases plus 9 adjacent stable FD/fcntl/pipe cases.
- Build after final record-lock and stable466 edits passed.
- Guardrail scan found no LTP case-name/output hardcoding in changed runtime files.

Detailed commands, raw-log paths, parser outputs, and checksums are in `validation.md`.

## Result

Session 3 is complete. Stable advanced from 462 to 466 with 4 four-way-clean fcntl record-lock cases.

## Risks / limitations

- `F_SETLKW` uses cooperative yielding rather than a proper wait queue and does not yet model `EINTR` on pending signals.
- OFD locks (`F_OFD_*`) were not implemented.
- `/proc/sys/fs/pipe-*` and `/proc/self/fd` blockers are documented for later synthetic/procfs work rather than hidden or blacklisted.
- `fcntl14` has noticeable memory delta in parser summaries; it is not a failure signal, but later resource sessions should watch allocator high-water behavior.

## Next session entry

Session 4 should start from VFS/metadata/path candidates in the Session 1 matrix, keeping the same promotion rule: no stable-list edit without RV/LA × musl/glibc parser-clean evidence.
