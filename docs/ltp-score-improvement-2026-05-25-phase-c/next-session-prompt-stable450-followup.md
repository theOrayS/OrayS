# Next-session prompt: continue from stable383 stop-state toward stable450

工作目录：`/root/oskernel2026-orays`

请继续使用 Ultragoal + Team 模式，中文汇报，遵守 AGENTS.md。

## Current handoff

上一轮在用户要求停止后提交的是 stable383 stop-state，而不是 stable450 final delivery。下一会话必须 live 复核，不要只依赖本提示词：

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 应为 383 total / 383 unique / 0 duplicates。
- stable375 之后已保留的新增 case：`clock_settime01`, `clock_settime02`, `clone03`, `confstr01`, `chmod05`, `fchmod05`, `lseek02`, `pipe08`。
- 最新新增 `pipe08` 是 pipe/SIGPIPE 行为覆盖。
- stable400 剩余 gap：17；stable450 剩余 gap：67。
- known transparent TCONF 仍只有 `read02`；不能把它说成 clean。

## Evidence to re-read

- `docs/ltp-score-improvement-2026-05-25-phase-c/stable383-promotion-gate-report.md`
- `docs/ltp-score-improvement-2026-05-25-phase-c/stable400-promotion-gate-report.md`
- `docs/ltp-score-improvement-2026-05-25-phase-c/candidate-matrix.md`
- `docs/ltp-score-improvement-2026-05-25-phase-c/remote-marker-and-log-noise-regression-check.md`
- `docs/ltp-score-improvement-2026-05-25-phase-c/log-noise-repair-report.md`

Key summaries:

- LA exact stable383: `raw/stable383-la-gate-001-summary.txt` => PASS LTP CASE 766 / FAIL 0; `ltp-musl` 383/0; `ltp-glibc` 383/0; internal TCONF 4 from `read02`; timeout/ENOSYS/panic 0.
- RV support: `raw/stable384-rv-gate-001-summary.txt` => prior stable384 superset PASS 768 / FAIL 0; `ltp-musl` 384/0; `ltp-glibc` 384/0; internal TCONF 4 from `read02`; timeout/ENOSYS/panic 0. Exact RV stable383 rerun was started but user-stopped; do not use its incomplete raw log as evidence.
- Targeted `pipe08`: RV evidence in `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt`; LA evidence in `raw/target-stable400-kill02-pipe08-la-001-summary.txt`; both musl+glibc clean.

## Explicit non-promotion evidence

- `kill02`: targeted clean but LA aggregate unstable. `raw/stable384-la-gate-001-summary.txt` showed LA musl aggregate `kill02` TBROK/setup timeout (`FAIL LTP CASE kill02 : 2`). Do not promote until aggregate-stable.
- `access04`, `chmod06`, `fchmod06`: RV TBROK due tmpfs mount setup EINVAL in `raw/target-stable400-access-chmod-rv-001-summary.txt`.
- `chmod07`, `fchmod02`: RV TBROK due `getgrnam(daemon)` setup in `raw/target-stable400-access-chmod-rv-001-summary.txt`.
- `waitid07/08/10`, `munmap01`, `mmap04/05`, `mprotect01/02`, `pipe07/15`: real RV failures in `raw/target-stable400-proc-vm-pipe-rv-001-summary.txt`.
- `readlinkat02`: RV clean and LA glibc clean, but LA musl TFAIL; do not promote without root-cause fix.
- `lseek03/04/05/06/08/09/10`: missing sdcard test binaries; `lseek11`: SEEK_DATA/SEEK_HOLE TCONF+ENOSYS.
- `statx04`-`statx12`: RV PASS 0 / FAIL 18, TBROK/TCONF blockers.
- Any invalid-concurrent/aborted QEMU logs are not evidence.

## Startup checklist

1. `df -h / /root` and `du -sh /root/.codex`.
2. `git status --short`; protect user `Riscv输出.txt` / `LoongArch输出.txt`, root kernels, sdcard/disk images, raw `.log`.
3. Recompute live stable count/duplicates from `examples/shell/src/cmd.rs`.
4. Re-read the reports listed above and current `.omx/ultragoal` status.
5. If exact proof is required before further promotion, rerun RV exact stable383 aggregate first:
   `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv` and parse with `python3 -B scripts/ltp_summary.py`.
6. Keep QEMU/evaluator gates serialized. Worker QEMU evidence is discovery-only unless isolated.

## Goal

Continue from stable383 toward stable400, then stable425/stable450. Stretch stable460/475 only if clean subset and resources are enough.

Promotion rule remains strict: every new case must have RV+LA x musl+glibc clean evidence: wrapper FAIL 0, internal TFAIL/TBROK 0, no新增 TCONF, parser timeout/ENOSYS/panic/trap 0. Do not fake PASS, hardcode case names, modify LTP source, or convert real failures into SKIP/TCONF/PASS.

## Suggested lanes

- VFS/metadata: fix tmpfs mount/getgrnam/setup blockers honestly, then retry small batches.
- FD/pipe: use `pipe08` as regression protection; do not retry `pipe07/15` until `/proc`/pipe capacity surfaces are real.
- Process/wait/signal: investigate `kill02` LA aggregate setup timeout before promotion; waitid blockers need real wait status support.
- VM: mmap/mprotect failures are real; target semantics before broad batches.
- Guardrail: keep marker prefix bad lines 0 and keep original `axfs::fops:297 [AxError::NotADirectory]` at 0 in completed logs; residual `axfs_ramfs::file:69` NotADirectory noise may remain but must be disclosed.
