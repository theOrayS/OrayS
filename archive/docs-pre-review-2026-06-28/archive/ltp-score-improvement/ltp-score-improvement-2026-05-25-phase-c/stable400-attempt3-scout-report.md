# Stable400 attempt 3 scout report

Date: 2026-05-26
Status: **no promotion accepted**; G002 remains active/in-progress.

## Live baseline

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: 379 total / 379 unique / 0 duplicates.
- Highest trusted delivered gate remains stable379:
  - RV: `raw/stable379-rv-gate-002-summary.txt` — PASS LTP CASE 758 / FAIL 0; `ltp-musl` 379/0; `ltp-glibc` 379/0.
  - LA: `raw/stable379-la-gate-001-summary.txt` — PASS LTP CASE 758 / FAIL 0; `ltp-musl` 379/0; `ltp-glibc` 379/0.

## Scout evidence

| Evidence | Result | Decision |
| --- | --- | --- |
| `raw/target-stable400-readlinkat02-rv-serial-001-summary.txt` | RV `readlinkat02` musl+glibc PASS 2 / FAIL 0, internal 0 | Not enough; LA required |
| `raw/target-stable400-readlinkat02-la-serial-001-summary.txt` | LA glibc PASS, LA musl FAIL with internal TFAIL 1 | Blocked; do not promote |
| `raw/target-stable400-readlinkat02-serial-promotion-candidates.txt` | Promotion candidates: 0 | Confirms no four-way clean candidate |
| `raw/target-stable400-wave2-rv-001-summary.txt` | RV musl PASS 1 / FAIL 8; TBROK 8, ENOSYS 1, panic/trap 1 (`pipe02`) | Negative scout only; avoid broad batches containing `pipe02` |
| `raw/target-stable400-timesignal-rv-serial-001-summary.txt` | RV musl PASS 1 / FAIL 10; TFAIL 22, TCONF 2, TBROK 1, timeout 3; glibc did not complete because scout was stopped after blockers | Negative/aborted scout only; no promotion |
| `raw/target-stable400-fd-rv-serial-001-summary.txt` | RV FD/fcntl scout PASS 0 / FAIL 16; TBROK 10, TFAIL 900, ENOSYS 6 | Negative scout only; no promotion |
| `raw/target-stable400-fspath-rv-serial-001-summary.txt` | RV FS/path scout PASS 0 / FAIL 16; TFAIL 26, TBROK 3, ENOSYS 4 | Negative scout only; no promotion |

Invalidated logs from an accidental concurrent-QEMU attempt were deleted or kept out of commit. They are not evidence and must not be used for promotion.

## Blocker details

- `readlinkat02`: RV clean and LA glibc clean, but LA musl reports a real internal TFAIL on the `readlinkat(..., NULL, 0)` negative-path expectation. Fresh diagnostic `raw/readlinkat02-la-diagnostic-003-summary.txt` shows the LA-musl call reaches `sys_readlinkat` with `bufsiz=1`, while LA-glibc reaches it with `bufsiz=0`; the syscall-body guard for real zero size is already correct. This is not safe to fix by changing kernel `readlinkat` semantics.
- `pipe02`: RV scout hit a panic/trap; do not include it in broad batches until root-caused.
- Wave2 metadata/path cases (`access04`, `chmod06`, `chmod07`, `fchmod02`, `fchmod06`, `statx01`, `rename04`, `rename05`) remain TBROK/ENOSYS-blocked in RV musl.
- Time/signal/wait scout (`clock_gettime01`, `nanosleep01`, `nanosleep02`, `pause01`, `sigpending02`, `signal01`, `signal06`, `waitid07`, `waitid08`, `waitid10`) is not clean: it includes real TFAIL/TBROK/TCONF/timeout signals. `kill02` alone passed RV musl in this scout, but it lacks glibc and LA proof and was previously called out as aggregate-risk, so it remains unpromoted.
- FD/fcntl scout (`dup05`, `fcntl07`, `fcntl11`, `fcntl14`, `fcntl15`, and 64-bit variants) is not clean: it fails both RV musl and RV glibc, with `mkfifo` ENOSYS on `dup05`/`fcntl07*` and record-locking TFAILs on `fcntl11`/`fcntl14`/`fcntl15*`.
- FS/path scout (`link02`, `mkdir02`, `unlink05`, `readlink03`, `symlink03`, `lstat02`, `stat03`, `stat04`) is not clean: it fails both RV musl and RV glibc with TFAIL/TBROK and `link`/`unlink` ENOSYS signals.

## Next minimal plan

1. Keep stable379 as the accepted baseline; stable400 still needs 21 additional four-way-clean cases, stable450 needs 71.
2. Do not re-run broad batches with `pipe02` or known wave2 blockers until those blocker lanes are fixed.
3. Treat `readlinkat02` as blocked outside the syscall-body semantics unless the LA-musl call-boundary root cause is found; forcing `EINVAL` for `bufsiz=1` would be Linux-incompatible.
4. Prefer a fresh candidate lane outside the current blockers: small FD/fcntl, process, or FS metadata subsets with one serial RV batch at a time, then LA only for RV-clean candidates.
