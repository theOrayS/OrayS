# Final gate code review report

Date: 2026-05-27
Scope: stable460 final story after replacing `kill02` with `mknod06` and rerunning RV/LA final gates.

## Verdict

**Approved for stable460 delivery.**

## Reviewed changes

- `examples/shell/src/cmd.rs`
  - Adds exactly eight cases to `LTP_STABLE_CASES`: `fchown05`, `fchownat01`, `fcntl18`, `fcntl18_64`, `syscall01`, `mknod06`, `mknod02`, `mknod05`.
  - Live list check: `460 total / 460 unique / 0 duplicates`.
  - `kill02`, `readlinkat02`, `mknod08`, `mknodat01`, and `rename14` are not present in the final stable460 list.
- `docs/ltp-score-improvement-2026-05-27-phase-a/*`
  - Adds final delivery, quality gate, marker/noise, cleanup audit, and follow-up artifacts.
  - Raw `.log` files are kept local and are not included in the final commit; small summaries/status/marker/noise files are the durable evidence surface.

## Independent review evidence

### Code reviewer

Subagent: `code-reviewer` (`019e6b0a-d0ed-7f02-9c2e-1fff22db27b3`)

Verdict: **APPROVE**

Evidence reported by the reviewer:

- `examples/shell/src/cmd.rs:50`: `LTP_STABLE_CASES total=460 unique=460 duplicates=0`.
- `raw/stable460-rv-final-gate-002-summary.txt:3-4,11-12`: PASS 920, FAIL 0, ltp-musl 460/0, ltp-glibc 460/0.
- `raw/stable460-la-final-gate-002-summary.txt:3-4,11-12`: PASS 920, FAIL 0, ltp-musl 460/0, ltp-glibc 460/0.
- `git show --name-only HEAD`: no committed `*.log` files.
- `final-gate-quality-gate.json`: contains `aiSlopCleaner`, `codeReview`, and `verification` evidence.
- Must-fix items: none.

### Architect reviewer

Subagent: `architect` (`019e6af0-fbf6-7223-bcc3-ae51df901bcd`)

ArchitectStatus: **CLEAR**

Evidence reported by the architect:

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES`: `total=460 unique=460 duplicates=[]`; `kill02/readlinkat02` are absent from the final list.
- Reparsed final logs with `scripts/ltp_summary.py`: RV and LA final 002 both PASS 920 / FAIL 0 / musl 460/0 / glibc 460/0.
- `read02` TCONF remains transparent and limited to the known caveat.
- `kill02` and `readlinkat02` demotion is documented in the delivery report and quality gate.
- Marker/noise guardrail is parser-safe: both final marker-prefix scans are 0 bad lines.
- Final source change is stable-list/docs only, with no new ABI/POSIX runtime logic.
- Residual risks: do not call the gate internal-TCONF-clean; do not re-promote `kill02` without fixing LA aggregate TBROK; `make all` was not run because no remote helper/submission path changed.

## Safety checks

| Check | Result | Evidence |
| --- | --- | --- |
| No fake pass / case-name hardcoding | Pass | No runtime logic was changed in this final story; stable-list-only source edit. |
| No LTP test-source modification | Pass | No files under the testsuite tree were edited. |
| No timeout/TBROK/TFAIL/TCONF laundering | Pass | `kill02` was demoted after LA aggregate `TBROK=8`; `readlinkat02` was demoted after LA musl `TFAIL=1`. |
| Stable count invariant | Pass | `LTP_STABLE_CASES total=460 unique=460 duplicate_names=0`. |
| RV final gate | Pass | `raw/stable460-rv-final-gate-002-summary.txt`: PASS 920, FAIL 0, musl/glibc 460/0. |
| LA final gate | Pass | `raw/stable460-la-final-gate-002-summary.txt`: PASS 920, FAIL 0, musl/glibc 460/0. |
| Marker prefix | Pass | RV/LA final 002 both `bad_marker_prefix_lines=0`. |
| Known TCONF caveat | Pass | Only previously disclosed `read02` TCONF remains: 4 per arch aggregate. |
| Formatting/build | Pass | `cargo fmt --all -- --check`, `git diff --check`, `make A=examples/shell ARCH=riscv64`. |

## Notes for future reviewers

- Do not re-promote `kill02` from targeted-only evidence. It needs an aggregate LA setup-timeout/TBROK fix first.
- `mknod08`, `mknodat01`, and `rename14` are clean reserves but were left out to keep stable460 exactly 460 unique cases.
- This final story has no new syscall/errno/ABI-visible behavior change. Earlier campaign behavior changes are in prior commits and are protected by the final stable460 aggregate gates.
