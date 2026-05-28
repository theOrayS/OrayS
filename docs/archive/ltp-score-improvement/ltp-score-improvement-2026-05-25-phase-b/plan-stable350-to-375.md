# Plan: LTP stable350 -> stable375 / stretch stable380

Date: 2026-05-25
Repo: `/root/oskernel2026-orays`
Leader-owned Ultragoal plan: `.omx/ultragoal/goals.json`
Source prompt: `next-session-prompt-stable350-to-375.md`

## A. Baseline refresh

- Disk preflight: `/` and `/root` are 56% used with 25G free; `/root/.codex` is 14G. No cleanup needed before work.
- Git baseline: HEAD `b897f7e7 Keep agent guidance focused on evaluator ROI`.
- Initial dirty state: only untracked phase-b docs directory; remote output files, if present, remain user evidence and are not staged by default.
- Live stable list: `examples/shell/src/cmd.rs::LTP_STABLE_CASES` = 350 total / 350 unique / 0 duplicates.
- Stable350 regression protection: phase-a final quality gate reports RV and LA each `PASS LTP CASE 700`, `FAIL 0`, `ltp-musl 350/0`, `ltp-glibc 350/0`, known `read02` TCONF only, timeout/ENOSYS/panic/trap 0, marker-prefix bad 0.

## B. Candidate policy

Promote only cases that are clean across RV+LA x musl+glibc using `python3 -B scripts/ltp_summary.py` or equivalent matrix parsing. Wrapper success alone is insufficient. Do not fake PASS, hardcode case names, edit LTP source, launder failures, count timeouts as pass, or hide TFAIL/TBROK/TCONF/ENOSYS/panic/trap.

Primary target set:
`access02 access04 chmod05 chmod06 chmod07 fchmod02 fchmod05 fchmod06 fchmodat02 statx01 readlinkat02 rename01 rename03 rename04 openat02 writev03 pipe2_02 waitid07 waitid08 waitid10 kill02 mmap04 mmap05 mmap06 munmap01`.

Stretch set:
`mprotect01 mprotect02 openat03 rename05 statx03`.

Fallback pool, ROI order:
`fs_perms01`-`fs_perms06`, `ftest01`-`ftest04`, `rwtest01`, `rwtest02`, `stream01`, `stream02`, `mmap10`, `mmap10_1`, `vma01`, `vma02`.

## C. Execution stages

1. Baseline and Team startup: create context, launch 5 executor workers if resources allow, otherwise 4.
2. Discovery matrix: worker outputs are discovery only unless leader serializes QEMU evidence. Candidate matrix must classify clean/TCONF/TFAIL/TBROK/timeout/ENOSYS/panic per arch/libc.
3. stable360 gate: select about 10 clean cases, update stable list in leader root, run serial RV+LA gates, parse summaries, marker-prefix check, write report.
4. stable368 gate: repeat for next tranche.
5. stable375 final gate: exactly 375 unique stable cases, final RV+LA stable gates, summary parsing, marker-prefix check, fmt/build, code review, ai-slop-cleaner, quality gate JSON.
6. optional stable380: proceed only if stretch cases are already clean and resources remain.

## D. Team lanes

- Worker 1: discovery and promotion matrix.
- Worker 2: VFS/permission/metadata lane.
- Worker 3: fd/pipe/iovec lane.
- Worker 4: process/wait/signal lane.
- Worker 5: mmap/munmap/mprotect and guardrail lane.

## E. Stop condition

Main success: 375 unique stable cases and RV+LA final stable gates each `PASS LTP CASE 750`, `FAIL 0`, musl/glibc `375/0`, no internal TFAIL/TBROK, no timeout/ENOSYS/panic/trap, no new undisclosed TCONF, marker-prefix bad 0, review/cleanup clean, agent-owned commit created.

If blocked before stable375, save highest trustworthy stableN evidence and a blocker report with exact case/arch/libc/internal signals/log paths and next steps.
