# Candidate matrix — stable375 toward stable450

Date: 2026-05-26
Team: `ltp-stable375-to-stab-eae749f6`
Status: stable381 partial promotion accepted; stable400/stable425/stable450 remain undelivered because only six new cases have fresh targeted plus RV/LA aggregate-clean evidence.

## A. Current gap summary

| Stage | Stable total | Unique | Duplicate | Gap to stable450 | Evidence |
| --- | ---: | ---: | ---: | ---: | --- |
| Live baseline before phase-c edits | 375 | 375 | 0 | 75 | `stable375-live-baseline.txt`; phase-b final gate summaries |
| Live accepted baseline after phase-c retry | 379 | 379 | 0 | 71 | `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; `raw/stable379-rv-gate-002-summary.txt`; `raw/stable379-la-gate-001-summary.txt` |
| Live accepted baseline after AF_UNIX group-lookup repair | 381 | 381 | 0 | 69 | `examples/shell/src/cmd.rs::LTP_STABLE_CASES`; `raw/stable381-rv-gate-001-summary.txt`; `raw/stable381-la-gate-001-summary.txt` |

Subsystem focus remains contest ROI: VFS/path/permissions, FD/pipe/iovec, process/wait/signal, mmap/VM, and light fs-suite substitutes. No low-ROI fake or broad subsystem promotion was accepted.

## Accepted promotion candidates with targeted and aggregate evidence

These cases passed targeted RV+LA x musl+glibc cleanly with no internal TFAIL/TBROK/TCONF, timeout, ENOSYS, panic, or trap. After retrying the aggregate gate, the initial four were accepted into stable379. A later AF_UNIX local socket connect repair unlocked `chmod05` and `fchmod05`; RV aggregate `stable381-rv-gate-001` and LA aggregate `stable381-la-gate-001` both report 762 wrapper PASS / 0 wrapper FAIL, ltp-musl 381/0, ltp-glibc 381/0, no ENOSYS/panic/trap, and only the pre-existing transparent `read02` TCONF pair. The earlier `ftest03` RV aggregate timeout was treated as a blocker until single-case retries at both 60s and 90s passed cleanly; it is not counted as a promoted new case.

| Case | Subsystem | RV evidence | LA evidence | Decision |
| --- | --- | --- | --- | --- |
| `clock_settime01` | time syscall errno/permission boundary | `raw/target-stable400-clocksettime2-rv-001-summary.txt`: PASS 4/FAIL 0 for pair; aggregate RV stable379 accepted | `raw/target-stable400-clocksettime2-la-001-summary.txt`: PASS 4/FAIL 0 for pair; aggregate LA stable379 accepted | Promoted in stable379 |
| `clock_settime02` | time syscall invalid pointer/clock boundary | same as above | same as above | Promoted in stable379 |
| `clone03` | process lifecycle / clone child exit | `raw/target-stable400-cloneconf2-rv-001-summary.txt`: PASS 4/FAIL 0 for pair; aggregate RV stable379 accepted | `raw/target-stable400-cloneconf2-la-001-summary.txt`: PASS 4/FAIL 0 for pair; aggregate LA stable379 accepted | Promoted in stable379 |
| `confstr01` | libc/sysconf compatibility surface | same as above | same as above | Promoted in stable379 |
| `chmod05` | permissions / musl group lookup via AF_UNIX nscd probe | `raw/target-stable400-chmod-fchmod-rv-001-summary.txt`: PASS 4/FAIL 0 for pair; aggregate RV stable381 accepted | `raw/target-stable400-chmod-fchmod-la-001-summary.txt`: PASS 4/FAIL 0 for pair; aggregate LA stable381 accepted | Promoted in stable381 |
| `fchmod05` | FD permissions / musl group lookup via AF_UNIX nscd probe | same as above | same as above | Promoted in stable381 |

## Rejected or blocked candidates from fresh phase-c scouts

| Case/family | Subsystem | Fresh evidence | Blocker | Next minimal action |
| --- | --- | --- | --- | --- |
| `readlinkat02` | VFS/path/user buffer | RV clean in `target-stable400-scout-rv-001`; LA diagnostic `raw/readlinkat02-la-diagnostic-003-summary.txt` | LA musl internal TFAIL: intended `bufsiz=0` reaches kernel as `bufsiz=1`; LA glibc reaches `0` and passes | Blocked outside syscall-body semantics; do not promote without LA-musl call-boundary/root-cause fix |
| `inode02` | fs metadata/stress | RV clean in scout; LA summary `target-stable400-rvclean2-la-001` | LA glibc timeout | Investigate LA runtime/memory growth before any promotion |
| `chmod05`, `fchmod05` | permission/group lookup | `target-stable400-chmod-fchmod-{rv,la}-001` plus stable381 aggregates | Fixed by AF_UNIX local socket connect errno repair; no longer blocked | Promoted in stable381; keep as regression target |
| `openat02`, `openat03` | openat flags/path semantics | `target-stable400-scout-rv-001` | RV both libc TBROK; ENOSPC/O_TMPFILE style setup failures | Check tmpfs capacity/O_TMPFILE errno path; not promotable |
| `rename01`, `rename03` | VFS rename/device setup | `target-stable400-scout-rv-001` | RV both libc TBROK, likely device/ENOSPC setup | Repair device/tmpfs setup before retest |
| `statx03` | statx mask/attribute semantics | `target-stable400-scout-rv-001` | RV both libc TFAIL | Compare expected statx fields/masks; no promotion |
| `ftest06`, `ftest09`, `openfile01` | fs-suite IO | `target-stable400-scout-rv-001` | wrapper failure or missing executable/path; not clean | Treat as fs-suite blocker lane, not quick promotion |
| `clock_getres01` | time | `target-stable400-light25-rv-001-partial-summary.txt` | pass_with_tconf, not clean | Do not promote unless TCONF is understood and accepted explicitly |
| `clock_gettime*`, `clock_nanosleep*`, `setitimer01`, `getitimer01`, `nanosleep01` | time/scheduler | `target-stable400-light25-rv-001-partial-summary.txt`, `target-stable400-light15-rv-001-summary.txt` | TFAIL/TBROK/timeout/UNKNOWN/ENOSYS | Needs syscall/timer semantics work, not promotion |
| `clone02`, `clone04`, `clone05` | clone flags/process | `target-stable400-light15-rv-001-summary.txt` | TFAIL/TBROK/ENOSYS | Requires clone flag semantics review |
| `creat04`, `creat06`, `creat07` | VFS/create permissions | `target-stable400-light15-rv-001-summary.txt` | TFAIL/TBROK or unknown | Check permission/ownership semantics before retest |

## Attempt 3 scout update (2026-05-26)

Attempt 3 initially accepted no new cases; after a follow-up AF_UNIX group-lookup repair, `chmod05` and `fchmod05` were accepted. Live stable is now 381 total / 381 unique / 0 duplicates.

| Case/family | Fresh evidence | Result | Decision |
| --- | --- | --- | --- |
| `readlinkat02` | `raw/target-stable400-readlinkat02-rv-serial-001-summary.txt`; `raw/target-stable400-readlinkat02-la-serial-001-summary.txt`; `raw/readlinkat02-la-diagnostic-003-summary.txt`; `raw/target-stable400-readlinkat02-serial-promotion-candidates.txt` | RV musl+glibc clean; LA glibc clean; LA musl TFAIL 1; diagnostic shows LA-musl passed `bufsiz=1` into syscall for the zero-size testcase | Blocked; kernel syscall already returns EINVAL for real `bufsiz=0`; do not add non-Linux special-case |
| Wave2 metadata/path batch | `raw/target-stable400-wave2-rv-001-summary.txt` | RV musl PASS 1 / FAIL 8; TBROK 8; ENOSYS 1; panic/trap 1 on `pipe02` | Negative scout only; do not use as promotion evidence |
| Time/signal/wait batch | `raw/target-stable400-timesignal-rv-serial-001-summary.txt` | RV musl PASS 1 / FAIL 10; TFAIL 22; TCONF 2; TBROK 1; timeout 3; scout stopped after blockers, so glibc incomplete | Negative/aborted scout only; no promotion |
| FD/fcntl batch | `raw/target-stable400-fd-rv-serial-001-summary.txt` | RV musl+glibc PASS 0 / FAIL 16; TBROK 10; TFAIL 900; ENOSYS 6 | Negative scout only; needs FIFO/syscall and fcntl record-lock semantics work |
| FS/path batch | `raw/target-stable400-fspath-rv-serial-001-summary.txt` | RV musl+glibc PASS 0 / FAIL 16; TFAIL 26; TBROK 3; ENOSYS 4 | Negative scout only; needs link/unlink syscall and metadata/path semantics work |

Guardrail: accidental concurrent-QEMU logs from an earlier wave3 attempt were explicitly invalidated and are not promotion evidence.

## Team lane conclusions

- Worker 1 built the initial discovery matrix and found no pre-existing artifact that could honestly promote non-stable cases without fresh four-way proof; the leader later accepted only four cases with fresh targeted plus aggregate proof.
- Worker 2 confirmed the VFS log-noise change preserves errno and found Batch A candidates mostly blocked by setup/metadata semantics.
- Worker 3 found FD/pipe/iovec candidates high-value but still blocker/unknown; no case was promoted from that lane.
- Worker 4 kept `kill02`/`waitid07`/`waitid08`/`waitid10` out of promotion because aggregate/four-way risk remains.
- Worker 5 found mmap/fs-suite substitutes useful for future work but not clean enough for immediate stable450 promotion.

## B. Not-yet-run cases worth adding to self-test

| Priority | Family/cases | Source/yield signal | Related subsystem | Rationale | Cost/risk |
| --- | --- | --- | --- | --- | --- |
| High | group/permission follow-ups around `chmod05`, `fchmod05` | already RV glibc clean, musl setup TBROK | VFS permissions, user/group lookup | Small compatibility repair may unlock multiple chmod/fchmod tests | Medium: setup/user database behavior can affect many tests |
| Blocked | `readlinkat02` LA-musl call-boundary split | RV clean, LA glibc clean, one LA musl TFAIL; diagnostic shows `bufsiz=1` reaches syscall | VFS path, user-memory copy, libc/ABI boundary | Not a safe syscall-body fix; only revisit with LA-musl call-boundary root cause | High: forcing EINVAL for `bufsiz=1` would break Linux-compatible readlinkat semantics |
| High | lightweight time/process neighbors after stable379 | `clock_settime01/02`, `clone03`, `confstr01` now promoted; nearby scouts still mixed | time syscall, process lifecycle | Nearby cases could produce more clean promotions after targeted fixes | Medium/high: timers caused timeouts/TFAIL in scout |
| Medium | openat/rename setup blockers | multiple TBROK in scout | VFS/tmpfs/device setup | Could unlock path semantics cases but setup failures must be real, not hidden | Medium/high: ENOSPC/device setup can mask real regressions |
| Medium | fs-suite missing executable/path issues | `ftest09`, `openfile01` wrapper `-1` | shell runner/test staging | May be staging rather than kernel semantics | Medium: must not modify LTP to fake PASS |

## C. Next minimal execution plan

1. Treat stable381 as the current live accepted baseline for the next campaign; stable450 remains 69 cases away.
2. Do not count the aborted `stable379-rv-gate-001` as accepted evidence. Keep its `ftest03` timeout documented, but prefer the accepted `stable379-rv-gate-002` / `stable379-la-gate-001` summaries plus the single-case `ftest03` retry summaries for current state.
3. Do not spend more promotion time on `readlinkat02` syscall-body changes: fresh diagnostic shows LA-musl passes `bufsiz=1` into the syscall for the zero-size testcase while LA-glibc passes `0`. Avoid `pipe02`, wave2 metadata/path blockers, time/signal/wait blockers, FD/fcntl record-lock/FIFO blockers, and FS/path link/unlink/stat blockers in broad batches until fixed.
4. After every blocker fix: run targeted RV+LA x musl+glibc matrix, then promote in small clean batches, then aggregate stable gate.
5. Preserve marker-prefix and remote log-size guardrails after every logging or runner change, and continue disclosing the known `read02` TCONF pair plus any inherited raw timeout notices separately from promoted-case cleanliness.
