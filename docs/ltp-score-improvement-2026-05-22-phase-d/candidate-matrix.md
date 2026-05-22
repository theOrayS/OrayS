# Phase-D Wave A candidate matrix

Date: 2026-05-22

## Scope and guardrails

- Worker lane: discovery/matrix only; no `.omx/ultragoal` checkpoint and no `LTP_STABLE_CASES` edit.
- Phase-C source of truth: `/root/oskernel2026-orays/docs/ltp-score-improvement-2026-05-22-phase-c/`.
- Promotion remains leader-owned; this file recommends targeted batches and preserves blockers as blockers.

## Evidence inputs

| Evidence | Result used for Phase-D |
| --- | --- |
| `final-gate-report.md` / `final-gate-quality-gate.json` | stable157 final full LA/RV gate had 314 PASS LTP CASE and 0 FAIL LTP CASE on each arch; known `read02` TCONF remained visible. |
| `stable157-promotion-gate-report.md` | final stable set is 157 cases per libc/arch; `sched_getscheduler02` rejected after LA stable regression. |
| `wave-d-exploratory.cases` + `wave-d-exploratory-rv-summary.json` | 129 exploratory cases; 43 were RV clean across musl/glibc. |
| `wave-d-la-confirmation-summary.json` | the same 43 were LA clean across musl/glibc with no internal fail/brok/conf, timeout, ENOSYS, panic/trap. |
| `stable183-targeted-la-summary.json` | cumulative stable183 attempt was rejected: 360 PASS / 6 FAIL wrappers, driven by timeout regressions in existing/cumulative cases; not a promotion gate. |

## Candidate inventory

| Bucket | Count | File / cases | Recommended use |
| --- | ---: | --- | --- |
| Stable baseline | 157 | `stable157.cases` | Always include for regression guard; do not edit stable list here. |
| Phase-C exploratory union | 271 | `wave-a-candidate-union-271.cases` | 180-300 enumeration pool: stable157 plus wave-D exploratory cases. |
| Cross-arch clean and not already stable | 28 | `fcntl29, fstatat01, personality01, personality02, pipe09, pipe10, pipe14, readv01, setegid01, setfsgid01, setfsuid01, setgid01, setgid03, setgroups01, setgroups02, setpgid01, setpgid02, setpgrp01, setpgrp02, setregid01, setresgid01, setresuid01, setreuid01, setuid01, statx02, write03, write06, writev02` | Best promotion candidates; fresh Phase-D targeted confirmation still required. |
| Selected Wave A run set | 180 | `wave-a-selected-180.cases` | Stable157 + 23 high-confidence new cases for stable180 objective. |
| RV not-clean wave-D cases | 86 | summarized below | Keep out of Wave A promotion batch until fixes land. |


## Subagent cross-check integration

- Runner conventions cross-check: current `examples/shell/src/cmd.rs` still exposes `LTP_STABLE_CASES` as 157 cases and supports file-driven targeted runs via `LTP_CASES=file:<path>` plus `LTP_CASE_TIMEOUT_SECS`; this matrix keeps promotion out of `LTP_STABLE_CASES` as required.
- Older Phase-C Wave-A shortlist files contain candidates that are now already part of final stable157 (for example `chmod03`, `faccessat02`, `fchmodat01`, `sched_get_priority_max02`, `sched_get_priority_min02`, `sysinfo02`, `unlinkat01`, `wait402`). Final Phase-C `stable157.cases` is therefore the baseline authority for this Phase-D matrix.
- Subagent risk review agreed that stale baseline assumptions and blocked `sched_getscheduler02` / wait / timeout / ENOSYS clusters must not be promoted without fresh evidence.

## Selected Wave A promotion candidates (+23)

These are selected from the 28 cross-arch clean not-yet-stable cases. Five clean alternates are intentionally held for a follow-up batch to keep Wave A at exactly 180 total cases.

| Priority | Cases | Rationale |
| --- | --- | --- |
| A1 fs/fd/io | `fcntl29`, `fstatat01`, `pipe09`, `pipe10`, `pipe14`, `readv01`, `write03`, `write06`, `writev02` | Clean in RV and LA confirmation; syscall families already partially represented in stable157. |
| A2 personality | `personality01`, `personality02` | Clean in both arches/libcs; small isolated surface. |
| A3 credential/process-id | `setegid01`, `setfsgid01`, `setfsuid01`, `setgid01`, `setgid03`, `setpgid01`, `setpgid02`, `setpgrp01`, `setpgrp02`, `setregid01`, `setresgid01`, `setresuid01` | Clean in both arches/libcs; excludes riskier alternates for the first Wave A pass. |

Held clean alternates for Wave B: `setgroups01`, `setgroups02`, `setreuid01`, `setuid01`, `statx02`.

## Targeted batch split

| Batch file | Count | Purpose |
| --- | ---: | --- |
| `wave-a-batch1-stable-core.cases` | 45 | early stable baseline/core syscalls. |
| `wave-a-batch2-stable-mid.cases` | 45 | stable process/time/resource middle section. |
| `wave-a-batch3-stable-recent.cases` | 45 | recently promoted stable and regression-heavy families. |
| `wave-a-batch4-stable180-newcases.cases` | 45 | remaining stable guard plus all +23 selected new candidates. |

Run recommendation: execute batch files RV-first, parse every output with `scripts/ltp_summary.py`, then run the exact same batch on LA only if RV has 0 wrapper FAIL, 0 TFAIL/TBROK, 0 timeout, 0 ENOSYS, and no panic/trap. Known `read02` TCONF is expected only when included in stable guard; do not hide it.

## Cumulative stable183 rejection details

| Case | Bad evidence in `stable183-targeted-la-summary.json` |
| --- | --- |
| `access01` | glibc(status=FAIL/code=137; timeout) |
| `getpid01` | glibc(status=FAIL/code=137; timeout) |
| `read02` | glibc(internal=TCONF=2); musl(internal=TCONF=2) |
| `waitpid07` | glibc(status=FAIL/code=137; timeout); musl(status=FAIL/code=137; timeout) |
| `waitpid09` | glibc(status=FAIL/code=137; timeout); musl(status=FAIL/code=137; timeout) |

## Phase-C RV blocked candidates snapshot

The following wave-D candidates were not RV-clean and should not be promoted without fixes and fresh LA/RV confirmation:

| Case | RV failure signal |
| --- | --- |
| `alarm05` | musl:TFAIL=1, musl:code=1 |
| `alarm07` | musl:TFAIL=1, musl:code=1 |
| `chmod05` | glibc:TFAIL=1, glibc:code=1, musl:TBROK=1, musl:code=2 |
| `chmod06` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `chmod07` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `close_range01` | glibc:TBROK=1, glibc:code=6, musl:TBROK=1, musl:code=6 |
| `close_range02` | glibc:ENOSYS, glibc:TBROK=1, glibc:TFAIL=9, glibc:code=3, musl:TCONF=1, musl:code=32 |
| `dup207` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `fchmod05` | glibc:TFAIL=1, glibc:code=1, musl:TBROK=1, musl:code=2 |
| `fchmod06` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `fchmodat02` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `fcntl05` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `fcntl07` | glibc:ENOSYS, glibc:TBROK=2, glibc:code=2, musl:ENOSYS, musl:TBROK=2, musl:code=2 |
| `fcntl11` | glibc:TFAIL=75, glibc:code=1, musl:TFAIL=75, musl:code=1 |
| `fcntl12` | glibc:TFAIL=1, glibc:code=5, musl:TFAIL=1, musl:code=5 |
| `fcntl13` | glibc:TFAIL=3, glibc:code=1, musl:TFAIL=3, musl:code=1 |
| `fcntl14` | glibc:TFAIL=361, glibc:code=1, musl:TFAIL=361, musl:code=1 |
| `fcntl15` | glibc:TFAIL=7, glibc:code=1, musl:TFAIL=7, musl:code=1 |
| `fcntl17` | glibc:TFAIL=6, glibc:code=1, musl:TFAIL=7, musl:code=1 |
| `fcntl18` | glibc:TFAIL=4, glibc:code=1, musl:TFAIL=4, musl:code=1 |
| `fcntl19` | glibc:TFAIL=37, glibc:code=1, musl:TFAIL=37, musl:code=1 |
| `fcntl20` | glibc:TFAIL=45, glibc:code=1, musl:TFAIL=45, musl:code=1 |
| `fcntl21` | glibc:TFAIL=81, glibc:code=1, musl:TFAIL=81, musl:code=1 |
| `fcntl22` | glibc:TFAIL=2, glibc:code=1, musl:TFAIL=2, musl:code=1 |
| `fcntl23` | glibc:TBROK=2, glibc:code=6, musl:TBROK=2, musl:code=6 |
| `fcntl24` | glibc:TCONF=2, glibc:code=32, musl:TCONF=2, musl:code=32 |
| `fcntl25` | glibc:TBROK=2, glibc:code=6, musl:TBROK=2, musl:code=6 |
| `fcntl26` | glibc:TCONF=2, glibc:code=32, musl:TCONF=2, musl:code=32 |
| `fcntl27` | glibc:TFAIL=2, glibc:code=1, musl:TFAIL=2, musl:code=1 |
| `fcntl30` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `fcntl31` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `fcntl32` | glibc:TCONF=2, glibc:code=32, musl:TCONF=2, musl:code=32 |
| `fcntl33` | glibc:TCONF=1, glibc:code=36, musl:TCONF=1, musl:code=36 |
| `fcntl34` | glibc:TBROK=1, glibc:code=2 |
| `fcntl35` | glibc:TCONF=1, glibc:code=36, musl:TCONF=1, musl:code=36 |
| `fcntl36` | glibc:TBROK=1, glibc:code=2 |
| `fcntl37` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `fcntl38` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `fcntl39` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `fstat03` | musl:TFAIL=1, musl:code=1 |
| `getcwd02` | musl:TBROK=1, musl:code=2 |
| `getcwd03` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `getcwd04` | glibc:TCONF=1, glibc:code=32, musl:TCONF=1, musl:code=32 |
| `gethostid01` | glibc:TFAIL=1, glibc:code=137, glibc:timeout, musl:TCONF=1, musl:code=32 |
| `gethostname02` | musl:TFAIL=1, musl:code=1 |
| `getpgid01` | glibc:TBROK=1, glibc:TFAIL=1, glibc:code=3, musl:TBROK=1, musl:TFAIL=1, musl:code=3 |
| `getrusage04` | glibc:code=137, glibc:timeout, musl:code=137, musl:timeout |
| `open07` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `open08` | glibc:TFAIL=4, glibc:code=1, musl:TFAIL=4, musl:code=1 |
| `open09` | glibc:TFAIL=2, glibc:code=1, musl:TFAIL=2, musl:code=1 |
| `open10` | glibc:TFAIL=3, glibc:code=1, musl:TBROK=1, musl:code=2 |
| `open11` | glibc:ENOSYS, glibc:TBROK=1, glibc:code=2, musl:ENOSYS, musl:TBROK=1, musl:code=2 |
| `open12` | glibc:TBROK=2, glibc:code=2, musl:TBROK=2, musl:code=2 |
| `open13` | glibc:ENOSYS, glibc:TFAIL=3, glibc:code=1, musl:ENOSYS, musl:TFAIL=3, musl:code=1 |
| `open14` | glibc:TBROK=2, glibc:code=2, musl:TBROK=2, musl:code=2 |
| `openat03` | glibc:TBROK=2, glibc:code=2, musl:TBROK=2, musl:code=2 |
| `openat04` | glibc:TBROK=1, glibc:code=6, musl:TBROK=1, musl:code=6 |
| `pipe08` | glibc:TFAIL=2, glibc:code=1, musl:TFAIL=2, musl:code=1 |
| `pipe11` | glibc:code=137, glibc:timeout, musl:code=137, musl:timeout |
| `pipe12` | glibc:TFAIL=1, glibc:code=137, glibc:timeout, musl:TFAIL=1, musl:code=137, musl:timeout |
| `pipe13` | glibc:code=137, glibc:timeout, musl:code=137, musl:timeout |
| `pipe15` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `pipe2_01` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `pipe2_02` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `pipe2_04` | glibc:TBROK=1, glibc:code=2, musl:TBROK=1, musl:code=2 |
| `read03` | glibc:ENOSYS, glibc:TBROK=1, glibc:code=6, musl:ENOSYS, musl:TBROK=1, musl:code=6 |
| `readv02` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `seteuid01` | glibc:code=-1, musl:code=-1 |
| `setgroups03` | musl:TFAIL=1, musl:code=1 |
| `setgroups04` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `setsid01` | glibc:TFAIL=3, musl:TFAIL=2 |
| `setuid03` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `statx01` | glibc:ENOSYS, glibc:TBROK=1, glibc:code=2, musl:ENOSYS, musl:TBROK=1, musl:code=2 |
| `statx03` | glibc:TFAIL=3, glibc:code=1, musl:TFAIL=3, musl:code=1 |
| `statx04` | glibc:TBROK=1, glibc:code=6, musl:TBROK=1, musl:code=6 |
| `statx05` | glibc:TCONF=1, glibc:code=32, musl:TCONF=1, musl:code=32 |
| `truncate03` | glibc:TFAIL=2, glibc:code=1, musl:TFAIL=2, musl:code=1 |
| `wait403` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `waitid01` | glibc:ENOSYS, glibc:TBROK=1, glibc:TFAIL=5, glibc:code=3, musl:ENOSYS, musl:TBROK=1, musl:TFAIL=5, musl:code=3 |
| `waitid02` | glibc:ENOSYS, glibc:TFAIL=1, glibc:code=1, musl:ENOSYS, musl:TFAIL=1, musl:code=1 |
| `waitid03` | glibc:ENOSYS, glibc:TFAIL=1, glibc:code=1, musl:ENOSYS, musl:TFAIL=1, musl:code=1 |
| `waitid04` | glibc:ENOSYS, glibc:TFAIL=1, glibc:code=1, musl:ENOSYS, musl:TFAIL=1, musl:code=1 |
| `write04` | glibc:ENOSYS, glibc:TBROK=1, glibc:code=2, musl:ENOSYS, musl:TBROK=1, musl:code=2 |
| `write05` | glibc:TFAIL=1, glibc:code=1, musl:TFAIL=1, musl:code=1 |
| `writev01` | glibc:TFAIL=2, glibc:code=1, musl:TFAIL=2, musl:code=1 |
| `writev03` | glibc:TCONF=1, glibc:code=32, musl:TCONF=1, musl:code=32 |

## Stop condition for leader promotion

- Candidate/matrix artifacts only establish the Wave A run plan.
- Promotion to stable180 requires fresh targeted evidence for `wave-a-selected-180.cases` or an equivalent leader-approved cumulative set on RV and LA, with wrapper FAIL=0, internal TFAIL/TBROK=0, timeout=0, ENOSYS=0, panic/trap=0, and visible TCONF accounting.
