# Baseline refresh: stable460 -> stable520

Date: 2026-05-28

## Live environment

- Working directory: `/root/oskernel2026-orays`
- Branch/HEAD: `dev/more-stable-on-ltp` / `037ed3ae Preserve the stable520 handoff before the next LTP push`
- Prompt baseline mismatch: prompt named `score/best` at `f40332a9`, but live checkout is already one commit later on a different branch. This report records the mismatch and does not revert it.
- Disk: `/` and `/root` 37% used, 36G available; `/root/.codex` 1.3G. No cleanup needed.
- Worktree before phase-b edits: pre-existing dirty/untracked docs/archive state and `.codegraph/`; this campaign will stage only new agent-owned phase-b/source changes.

## Live stable list

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` recalculated at bootstrap:

```text
total=460 unique=460 duplicates=0
```

Last 20 stable rows include the stable460 additions: `fchown05`, `fchownat01`, `fcntl18`, `fcntl18_64`, `syscall01`, `mknod06`, `mknod02`, `mknod05`.

## Stable460 evidence recheck

Source directory: `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-27-phase-a/`.

| Evidence | Rechecked result |
| --- | --- |
| `stable460-delivery-report.md` | stable460 reached; final additions and demotions documented |
| `final-gate-quality-gate.json` | status passed; stable count 460/460/0; review/validation fields present |
| `final-gate-code-review-report.md` | code-review APPROVE and architect CLEAR recorded |
| `final-gate-ai-slop-cleaner-report.md` | pass/no-op cleanup |
| `remote-marker-and-log-noise-regression-check.md` | marker bad-prefix lines 0, noise disclosed |
| `raw/stable460-rv-final-gate-002.log` reparsed | PASS LTP CASE 920; FAIL 0; internal TCONF 4; timeout/ENOSYS/panic/trap 0; musl/glibc 460/0 |
| `raw/stable460-la-final-gate-002.log` reparsed | PASS LTP CASE 920; FAIL 0; internal TCONF 4; timeout/ENOSYS/panic/trap 0; musl/glibc 460/0 |
| marker-prefix files | RV 0 bad lines; LA 0 bad lines |
| noise files | RV/LA `AxError::NotADirectory=12`, `axfs::fops:297=0`, `axfs_ramfs::file:69=12` |

The only accepted internal caveat remains the known `read02` TCONF (4 per arch aggregate). This campaign must not introduce new TCONF/TFAIL/TBROK/timeout/ENOSYS/panic/trap.

## Preserved blockers/reserves

- Fresh-gate reserves: `mknod08`, `mknodat01`, `rename14`.
- Do not promote from targeted-only evidence: `kill02`, because LA aggregate stable460 exposed child setup timeout/TBROK.
- Do not promote without repair: `readlinkat02`, because LA musl had TFAIL.

## Next action

Create/force the new Ultragoal plan from `ultragoal-brief-stable460-to-520.md`, start Team workers, and build `candidate-matrix-stable460-to-520.md` before any stable-list edit.
