# Baseline refresh report: stable413 -> stable460

Date: 2026-05-27
Mode: Ultragoal + Team startup.

## Preflight

| Check | Evidence | Result |
| --- | --- | --- |
| Working directory | `/root/oskernel2026-orays` | pass |
| Disk | `df -h / /root`: `/dev/vda2` 59G size, 21G used, 37G available, 36% used | pass; no cleanup needed |
| Codex home size | `du -sh /root/.codex`: 1.2G | pass; no cleanup needed |
| Git state | `git status --short` at start: clean | pass |
| Team runtime | `tmux 3.4`, `$TMUX` set, `omx` found at `/root/.nvm/versions/node/v24.15.0/bin/omx`, one HUD watcher pane | pass |

## Live stable list

Parsed `examples/shell/src/cmd.rs::LTP_STABLE_CASES` live at startup:

- Total: 413
- Unique: 413
- Duplicates: 0
- Last 30 entries match the stable413 promoted FD/sendfile/preadv2/pwritev2 set.

## stable413 evidence reviewed

Evidence root: `docs/ltp-score-improvement-2026-05-26-phase-a/`.

Reviewed files:

- `stable413-delivery-report.md`
- `final-gate-quality-gate.json`
- `final-gate-code-review-report.md`
- `final-gate-ai-slop-cleaner-report.md`
- `remote-marker-and-log-noise-regression-check.md`
- `raw/stable413-rv-final-gate-002-summary.txt`
- `raw/stable413-la-final-gate-002-summary.txt`
- Worker/deferred reports listed in the user handoff prompt.

Final stable413 gate result:

| Arch | PASS LTP CASE | FAIL LTP CASE | ltp-musl | ltp-glibc | Internal TCONF | timeout | ENOSYS | panic/trap |
| --- | ---: | ---: | --- | --- | ---: | ---: | ---: | ---: |
| RV | 826 | 0 | 413/0 | 413/0 | 4 known `read02` only | 0 | 0 | 0 |
| LA | 826 | 0 | 413/0 | 413/0 | 4 known `read02` only | 0 | 0 | 0 |

Marker/noise guardrail from stable413:

- Bad marker-prefix lines: 0 on RV and LA.
- Remote-sensitive `axfs::fops:297 [AxError::NotADirectory]`: 0 on RV and LA.
- Residual `axfs_ramfs::file:69` NotADirectory: 22 on RV and 22 on LA; disclosed, not marker-affecting.

## Prior worker/deferred evidence folded into this round

- `candidate-matrix-easy30-40.md`: inventory-present candidates are scout inputs, not promotion evidence.
- `worker1-candidate-matrix-delta-after-reports.md`: exact delta rows must replace heuristics when available.
- `worker2-light-syscall-process-scout-report.md` and `worker2-light-syscall-rv001-diagnosis.md`: `poll02`, `gethostid01`, `getcpu01`, and `gethostname02` remain blocked until libc-specific failures are resolved.
- `worker3-metadata-statfs-getdents-report.md` and `worker3-metadata-narrow-repair-feasibility.md`: no blind legacy syscall aliases; getdents/statfs require narrow repair plus fresh RV/LA proof.
- `worker4-fd-io-vfs-guardrail-report.md`: FD/sendfile adjacent and VFS create/remove cases need parser-backed promotion, not source-shape assumptions.
- `stable423-stretch-report.md`: stretch was deferred honestly because no extra 10-case subset had completed RV+LA x musl/glibc proof.

## Baseline conclusion

stable413 is accepted as the regression baseline for this campaign. No cleanup was required, no user evidence files were modified, and the next safe step is Team-backed candidate discovery plus leader-serialized targeted runs.
