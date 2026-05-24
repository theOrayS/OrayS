# stable350 delivery report

Date: 2026-05-25
Requested target: stable350
Delivered state: **stable300 retained**

## Final status

- Live `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: **300 total / 300 unique / 0 duplicates**.
- No duplicate stable entries.
- No new stable cases were added.
- Stable350 final gate was **not run**, because stable315 promotion did not pass.
- Codex goal remains active; final `update_goal({status: "complete"})` was not called.

## What was completed

- Team state prompt was rechecked during this handoff; `omx team status ltp-stable300-to-stab-7c9de325` now reports no team state and the referenced leader mailbox is missing, so the worker prompts are stale.
- Discovery/report lanes produced candidate matrices for permissions/VFS, fd/pipe/iovec, process/wait/signal/rlimit, and mmap/mprotect/munmap.
- Earlier Team-integrated implementation changes remained in place:
  - LTP runner environment/cwd handling for resource-helper cases (`LTPROOT`, `PATH`, per-case `/tmp/ltp-work/<case>-run`).
  - Linux-visible `prlimit64(current pid)` acceptance plus default-fatal signal delivery/exit handling.
  - LoongArch musl scheduler wrapper patch preserving libc errno semantics for `sched_getscheduler02`.
- This follow-up added a real wait/signal fix: fork-like process children restore the pre-libc-fork all-application signal mask when libc temporarily blocks every maskable signal around fork.
- Validation completed before or during follow-up gates:
  - `cargo fmt --all -- --check` passed.
  - `python3 -B scripts/test_ltp_summary.py` passed earlier in this phase.
  - `git diff --check` passed.
  - `make A=examples/shell ARCH=riscv64` passed.
  - RV targeted `followup-rv-waitpid01-maskrestore-001`: PASS 2 / FAIL 0; `ltp-musl 1/0`, `ltp-glibc 1/0`; internal TFAIL/TBROK/TCONF=0.
  - LA targeted `followup-la-waitpid01-maskrestore-001`: PASS 2 / FAIL 0; `ltp-musl 1/0`, `ltp-glibc 1/0`; internal TFAIL/TBROK/TCONF=0.
  - RV guard `followup-rv-waitpid-signal-guard-001`: PASS 16 / FAIL 0, both libc 8/0, internal TFAIL/TBROK/TCONF=0.
  - LA guard `followup-la-waitpid-signal-guard-001`: PASS 16 / FAIL 0, both libc 8/0, internal TFAIL/TBROK/TCONF=0.
  - Follow-up marker-prefix scan over new waitpid/pipe logs: `TOTAL markers=42 bad=0`.
  - RV `pipe2_02` after `/bin/sh` compatibility fix: PASS 2 / FAIL 0; internal TFAIL/TBROK/TCONF=0.
  - LA `pipe2_02` after `/bin/sh` compatibility fix: PASS 2 / FAIL 0; internal TFAIL/TBROK/TCONF=0.
  - Pipe2 marker-prefix scan: `TOTAL markers=4 bad=0`.

## Why stable350 was not delivered

Fresh candidate evidence now contains eight four-way clean cases: `prctl05,sched_getscheduler02,sethostname01,setrlimit01,signal03,signal04,waitpid01,pipe2_02`. This is useful but still below stable315's +15 gate, so no stable315/stable330/stable350 aggregate gate was justified.

The former high-value blocker `pipe2_02` is now a clean seed after the `/bin/sh` exec fallback fix. Remaining work is to find at least 7 more four-way clean cases for stable315.

Post-Team LA attempts `followup-la-targeted-001/002/003` were aborted/untrusted due duplicated starts and are not used as evidence.

## Promotion policy preserved

No timeout, ENOSYS, panic/trap, TFAIL, TBROK, or TCONF was converted to PASS. No case-name hardcoding was introduced. `read02` remains transparently documented as `pass_with_tconf` in the stable300 baseline.

## User-visible / ABI-visible behavior changes integrated this round

- LTP runner behavior: cases with `<case>_*` resource helpers may run from a per-case work directory with `LTPROOT` and adjusted `PATH`; this is harness/environment behavior for executing real LTP binaries, not a PASS shim.
- POSIX/Linux-visible behavior: `prlimit64` now accepts the current process pid as a valid current target; default-fatal self/pending signals are handled more synchronously when unblocked or delivered.
- LoongArch musl loader behavior: ENOSYS-only exported musl scheduler wrappers are patched to issue the real syscall and tail-call musl `__syscall_ret`, preserving libc errno semantics while raw syscall paths still return raw `-errno`.
- POSIX/Linux-visible behavior added by this follow-up: fork-like process creation now avoids leaking libc's transient all-application-signal mask into the child, so default-fatal child signals are reported through wait status instead of being spuriously blocked until normal exit.
- POSIX/Linux-visible behavior added by this follow-up: `execve("/bin/sh", ...)` and related busybox shell aliases fall back to the current suite busybox when no root-level `/bin/sh` exists, allowing real libc `system()` calls to execute shell commands instead of failing before the command starts.
- `LTP_STABLE_CASES` and visible stable score did not change.
