# Worker 3 FD / pipe / iovec lane report

Date: 2026-05-25
Team: `ltp-stable375-to-stab-eae749f6`
Task: `task-3`
Mode: leader fallback report after the worker-3 pane stayed at the startup screen and did not execute the mailbox prompt. No QEMU/run-eval was started for this lane.

## Guardrails followed

- Claimed task 3 through `omx team api claim-task` before completing the fallback report.
- Did **not** run QEMU or `run-eval.sh` for promotion evidence.
- Did **not** edit `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Did **not** edit `.omx/ultragoal`.
- Used only live stable-list parsing plus existing phase-b/phase-c reports and parsed summaries.
- Subagent skip reason: worker-3 pane was nonresponsive, and this fallback was a bounded report-only artifact scan; no independent code-changing subtask was safe or useful while leader-owned serial QEMU was already running.

## Live stable membership refresh

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` remains `375 total / 375 unique / 0 duplicates`.

Already stable regression sentinels in this lane:

- Pipes: `pipe01`, `pipe03`, `pipe04`, `pipe05`, `pipe06`, `pipe09`, `pipe10`, `pipe11`, `pipe12`, `pipe13`, `pipe14`, `pipe2_01`, `pipe2_02`, `pipe2_04`.
- I/O vectors: `readv01`, `readv02`, `writev01`, `writev02`, `writev05`, `writev06`, `writev07`, `preadv01_64`, `preadv02_64`, `pwritev01_64`, `pwritev02_64`.
- Fcntl: `fcntl01`, `fcntl01_64`, `fcntl02`, `fcntl02_64`, `fcntl03`, `fcntl03_64`, `fcntl04`, `fcntl04_64`, `fcntl05`, `fcntl05_64`, `fcntl08`, `fcntl08_64`, `fcntl09`, `fcntl09_64`, `fcntl10`, `fcntl10_64`, `fcntl12`, `fcntl12_64`, `fcntl13`, `fcntl13_64`, `fcntl16`, `fcntl16_64`, `fcntl23`, `fcntl23_64`, `fcntl29`, `fcntl29_64`.

Not stable and requiring fresh four-way proof before promotion:

`writev03`, `pipe02`, `pipe07`, `pipe08`, `pipe15`, `pipe2_03`, `readv03`, non-`_64` `preadv01`/`preadv02`, non-`_64` `pwritev01`/`pwritev02`, `sendfile02`, `sendfile03`, `fcntl06`, `fcntl07`.

## Candidate decisions

| Candidate | Existing evidence | Decision |
| --- | --- | --- |
| `writev03` | `docs/ltp-score-improvement-2026-05-25-phase-b/raw/target-primary30-rv-002-summary.txt` reports RV glibc and RV musl `FAIL`, code `32`, internal `TCONF=1`; phase-c worker1 also classifies it as TCONF/fail-wrapper. | Do not promote. Keep the TCONF transparent; future work should inspect mmap-backed `MAP_SHARED`/iovec fault behavior before any stable-list edit. |
| `pipe02` | `docs/ltp-score-improvement-2026-05-25-phase-b/raw/target-scout26-rv-001-summary.txt` reports `panic_trap=1` and `UNKNOWN` for RV musl; phase-b `candidate-matrix.md` calls this a real discovery panic. | Do not promote. Root-cause panic first; no wrapper-only promotion evidence is acceptable. |
| `pipe07`, `pipe08`, `pipe15`, `pipe2_03` | No four-way clean evidence found in phase-b/phase-c summaries. The already stable pipe family provides regression coverage, but `pipe02` panic means adjacent pipe expansion is not free. | Unknown; scout serially only after `pipe02` panic is understood or with isolated raw capture. |
| `fcntl07` | `target-scout26-rv-001-summary.txt` reports RV musl `FAIL`, code `2`, `TBROK=2`, `ENOSYS=1`. | Do not promote; syscall/command support or setup path is still incomplete. |
| `fcntl06` | Absent from stable list and no clean four-way proof located in inspected artifacts. | Unknown; lower than metadata/openat/chmod candidates unless a fresh target matrix shows clean. |
| `readv03`, `preadv01`, `preadv02`, `pwritev01`, `pwritev02`, `sendfile02`, `sendfile03` | No clean phase-c proof found. `_64` preadv/pwritev variants are already stable and should be regression sentinels, not duplicate promotions. | Unknown; require targeted matrix with both libc/arch before any promotion. |

## Recommended next queue for this lane

For stable400, this lane currently contributes **blocker awareness**, not clean cases. If leader needs more FD/pipe scouts after Batch A metadata candidates, use this order:

1. `pipe07,pipe08,pipe15,pipe2_03` as a small isolated pipe scout, but stop immediately on panic/timeout.
2. `readv03,sendfile02,sendfile03` as FD/data-path scouts.
3. `fcntl06,fcntl07` only after checking which fcntl operations are missing and why `fcntl07` hit ENOSYS/TBROK.
4. Keep `writev03` out until its internal TCONF/fail-wrapper behavior is understood.

## Verification

- Parsed live `LTP_STABLE_CASES`: `375 total / 375 unique / 0 duplicates`.
- Confirmed task-specific target cases are either absent or already stable as listed above.
- Searched phase-b/phase-c parsed summaries and reports for `writev03`, `pipe02`, `fcntl07`, pipe/iovec/fcntl candidates.
- Ran `git diff --check` after writing this report: pass.
- No QEMU/run-eval and no stable-list or Ultragoal mutation.
