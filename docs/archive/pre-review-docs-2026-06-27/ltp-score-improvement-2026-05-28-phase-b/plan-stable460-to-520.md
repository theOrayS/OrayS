# Plan: LTP stable460 -> stable520 (stretch stable530)

Date: 2026-05-28
Mode: Ultragoal + Team

## Live bootstrap facts

- Disk preflight: `/` and `/root` 37% used with 36G available; `/root/.codex` 1.3G. No cleanup needed.
- Git live state: current branch `dev/more-stable-on-ltp`, HEAD `037ed3ae Preserve the stable520 handoff before the next LTP push`; prompt baseline `score/best`/`f40332a9` is not the current checkout and is not forced.
- Dirty state exists before this campaign: deleted long-term/phase-a docs plus many untracked archived evidence files and `.codegraph/`. This plan treats them as pre-existing and will not revert or stage them.
- Live stable count: `460 total / 460 unique / 0 duplicates`.
- Trusted previous final gate: archived stable460 RV+LA final 002 summaries both `PASS LTP CASE 920`, `FAIL 0`, `ltp-musl 460/0`, `ltp-glibc 460/0`, timeout/ENOSYS/panic/trap 0, marker-prefix bad lines 0, with only known `read02` TCONF caveat.

## Execution model

Leader owns:

- `.omx/ultragoal` plan/ledger/checkpoints;
- Team launch/monitor/shutdown;
- final candidate promotion decisions;
- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` edits;
- serialized QEMU promotion/final gates when default images are shared;
- final quality gate, code review, ai-slop-cleaner, commit.

Workers own discovery/diagnosis/report slices only. No worker may claim promotion from targeted-only or wrapper-only evidence.

## Stage gates

| Stage | Target count | Required evidence | Stop/demotion rule |
| --- | ---: | --- | --- |
| baseline-refresh | 460 | live stable count, disk/git facts, stable460 reparse/marker/noise evidence | if baseline suspect, run small stable smoke before promotion |
| candidate-matrix | 460 | matrix of candidate status by arch/libc/internal result and source/runtest anchors | unknown rows stay scout-only |
| stable475 | 475 | ~15 fresh four-way-clean cases, RV+LA aggregate gate, report | demote any wrapper FAIL/internal failure/timeout/ENOSYS/panic/trap/marker issue |
| stable490 | 490 | same | same |
| stable505 | 505 | same | same |
| stable520 | 520 | final RV+LA stable aggregate, quality gate, review, cleaner, validation | if not clean, revert/demote bad candidates and preserve highest stableN |
| optional stable530 | 530 | only if enough clean subset and time/resources remain | otherwise write follow-up prompt |

## First-wave candidate intent

1. Re-gate clean reserves: `mknod08`, `mknodat01`, `rename14`.
2. Scout adjacent VFS/path/mknod/rename/openat rows.
3. Scout FD/fcntl/pipe/ownership rows: `pipe07`, `fcntl19-22` variants, `fchown04`, `fchownat02`, `chown04`.
4. Diagnose metadata/statfs/getdents rows only with source-level expectation and targeted parser proof.
5. Use process/light syscall and fs-suite substitutes as fill only when parser-clean.

## Required deliverables

- `candidate-matrix-stable460-to-520.md`
- `stable475-promotion-gate-report.md`
- `stable490-promotion-gate-report.md`
- `stable505-promotion-gate-report.md`
- `stable520-delivery-report.md`
- `final-gate-quality-gate.json`
- `final-gate-code-review-report.md`
- `final-gate-ai-slop-cleaner-report.md`
- `remote-marker-and-log-noise-regression-check.md`
- follow-up/blocker prompt if stable520/stretch remains incomplete
