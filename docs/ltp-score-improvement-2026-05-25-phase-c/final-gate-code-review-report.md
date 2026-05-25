# Final gate code review report

Date: 2026-05-26
Scope: phase-c partial promotion to stable379 and earlier expected-errno log-noise repair.

## Verdict

- Stable450 final delivery: **not approved / not claimed**.
- Stable379 partial promotion: **approved with disclosed caveats**.

## Reviewed changes

- `examples/shell/src/cmd.rs`: adds exactly four accepted stable cases: `clock_settime01`, `clock_settime02`, `clone03`, `confstr01`.
- Earlier committed `kernel/fs/axfs/src/fops.rs` and `kernel/fs/axfs/src/root.rs` log-noise changes return the same `AxError` values directly instead of invoking noisy `ax_err!` on expected negative paths.

## Evidence reviewed

- RV stable379 aggregate: `raw/stable379-rv-gate-002-summary.txt` PASS 758 / FAIL 0.
- LA stable379 aggregate: `raw/stable379-la-gate-001-summary.txt` PASS 758 / FAIL 0.
- Marker prefix: 0 bad lines in both accepted aggregates.
- Original `axfs::fops` NotADirectory noise: 0 in both accepted aggregates.
- Known `read02` TCONF remains disclosed.

## Risks

- The campaign did not find enough clean cases for stable400/425/450.
- LA raw stable379 log contains two inherited internal `Test timeouted` notices in pre-existing cases; parser wrapper timeout count is 0 and no newly promoted case is affected, but future gates should continue tracking this separately.
- Residual `axfs_ramfs::file:69` NotADirectory noise remains at 22 per accepted aggregate.

## Fresh validation run by leader

- `cargo fmt --all -- --check`: passed.
- `git diff --check`: passed.
- `make A=examples/shell ARCH=riscv64`: passed; this Makefile path built the remote-submission RV wrapper and LA ELF outputs (`kernel-rv`/`kernel-la`) as generated artifacts, which are not committed.
