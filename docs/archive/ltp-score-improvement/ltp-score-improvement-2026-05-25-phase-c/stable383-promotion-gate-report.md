# Stable383 stop-state promotion gate report

Date: 2026-05-26
Status: **partial honest stop-state committed; stable450 not achieved**.

## A. Current gap summary

Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` was recalculated after the user stop request:

- total: 383
- unique: 383
- duplicates: 0
- newly retained case over stable382: `pipe08`
- stable400 remaining gap: 17 cases
- stable450 remaining gap: 67 cases

`pipe08` covers pipe/SIGPIPE behavior. It was retained because targeted RV/LA x musl/glibc evidence was clean, and the latest completed aggregate evidence does not show a regression for this case. `kill02` was explicitly rejected even though targeted evidence was clean, because LA aggregate exposed TBROK/setup timeout risk.

## Accepted stop-state evidence

| Evidence | Result | Use |
| --- | --- | --- |
| `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt` | RV targeted `kill02,pipe08,...`: `pipe08` musl+glibc PASS clean; no timeout/ENOSYS/panic | Targeted RV proof for `pipe08` |
| `raw/target-stable400-kill02-pipe08-la-001-summary.txt` | LA targeted `kill02,pipe08`: PASS 4 / FAIL 0; `pipe08` musl+glibc clean | Targeted LA proof for `pipe08` |
| `raw/stable383-la-gate-001-summary.txt` | LA exact stable383 aggregate PASS LTP CASE 766 / FAIL 0; `ltp-musl` 383/0; `ltp-glibc` 383/0; internal TCONF 4 (`read02` only); timeout/ENOSYS/panic 0 | Exact LA aggregate proof for live stable383 |
| `raw/stable384-rv-gate-001-summary.txt` | RV earlier stable384 aggregate PASS LTP CASE 768 / FAIL 0; `ltp-musl` 384/0; `ltp-glibc` 384/0; internal TCONF 4 (`read02` only); timeout/ENOSYS/panic 0 | Completed RV superset evidence containing `pipe08`; exact stable383 RV rerun was stopped by user request |

The attempted exact RV stable383 aggregate run was started after LA exact passed, but the user requested that the task stop before it completed. The incomplete raw log is not used as promotion evidence and is not committed.

## Rejected evidence / blockers

| Case/family | Evidence | Reason | Decision |
| --- | --- | --- | --- |
| `kill02` | `raw/target-stable400-kill02-pipe08-la-001-summary.txt`; `raw/stable384-la-gate-001-summary.txt` | Targeted RV/LA was clean, but LA stable384 aggregate showed `kill02` TBROK setup timeout in musl aggregate (`FAIL LTP CASE kill02 : 2`) | Do not promote until aggregate-stable on LA |
| `access04`, `chmod06`, `fchmod06` | `raw/target-stable400-access-chmod-rv-001-summary.txt` | `mount(... tmpfs ...)` setup EINVAL/TBROK | Blocked |
| `chmod07`, `fchmod02` | `raw/target-stable400-access-chmod-rv-001-summary.txt` | `getgrnam(daemon)` setup breakage | Blocked |
| `waitid07`, `waitid08`, `waitid10` | `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt` | wait status / `/proc/sys/kernel/core_pattern` blockers | Blocked |
| `munmap01`, `mmap04`, `mmap05`, `mprotect01`, `mprotect02` | `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt` | VM permission / maps / signal behavior failures | Blocked |
| `pipe07`, `pipe15` | `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt` | missing `/proc` pipe capacity/fd setup | Blocked |

## Marker and log-noise guardrail

Completed aggregate logs checked:

- `raw/stable383-la-gate-001.log`: bad marker prefix 0; `axfs::fops:297 [AxError::NotADirectory]` count 0; residual `AxError::NotADirectory` count 22.
- `raw/stable384-rv-gate-001.log`: bad marker prefix 0; `axfs::fops:297 [AxError::NotADirectory]` count 0; residual `AxError::NotADirectory` count 22.

The original high-frequency `fops:297` warning did not regress in the completed logs. Residual `axfs_ramfs::file:69`-style `NotADirectory` warnings remain future cleanup work and were not hidden.

## B. Not-yet-run cases worth adding to self-test

- `kill02`: high-value process/signal case, but must be rerun in LA aggregate after fixing setup timeout; targeted-only clean is insufficient.
- `access04` / `chmod06` / `fchmod06`: VFS permission/path setup; requires real tmpfs mount/setup compatibility, not a fake skip.
- `waitid07/08/10`: process lifecycle and wait status; high hidden-test value but currently TFAIL/TBROK.
- `mmap` / `mprotect` small cases: useful VM boundary coverage, but current failures are real and need semantics work.
- `pipe07/15`: FD/procfs/pipe capacity behavior; useful after `/proc/self/fd` and pipe sysctl surfaces are real.

## C. Next minimal execution plan

1. Treat this commit as the latest stop-state: live stable383 with `pipe08`, stable450 still open.
2. On resume, first rerun exact RV stable383 aggregate if strict two-arch exact proof is required; the previous attempt was intentionally stopped by user request.
3. Do not promote `kill02` until LA aggregate no longer shows setup timeout/TBROK.
4. Continue targeted candidate batches, but only promote after RV+LA x musl+glibc clean targeted evidence plus aggregate gates.
5. Preserve marker prefix checks and disclose `read02` TCONF transparently.
