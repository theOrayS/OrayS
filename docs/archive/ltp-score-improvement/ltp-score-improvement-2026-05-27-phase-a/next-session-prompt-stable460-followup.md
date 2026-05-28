# Next-session prompt: stable460 follow-up toward stable470+

工作目录：`/root/oskernel2026-orays`

请用中文汇报，继续按 `AGENTS.md`、Team + Ultragoal 模式推进。当前已交付 baseline 是 **stable460**，请先 live 复核，不要只依赖本提示词。

## 已知交付点

- 当前 stable460 final commit 会在本轮最终提交中记录。
- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` 应为 `460 total / 460 unique / 0 duplicates`。
- 证据目录：`docs/ltp-score-improvement-2026-05-27-phase-a/`。
- Final gates:
  - RV: `raw/stable460-rv-final-gate-002-summary.txt` -> PASS LTP CASE 920, FAIL 0; ltp-musl 460/0; ltp-glibc 460/0。
  - LA: `raw/stable460-la-final-gate-002-summary.txt` -> PASS LTP CASE 920, FAIL 0; ltp-musl 460/0; ltp-glibc 460/0。
- Known caveat: only the previously disclosed `read02` O_DIRECT/tmpfs TCONF remains (4 internal TCONF per arch aggregate, 2 per libc group). Do not describe the gate as internal-TCONF-clean.
- Marker prefix: final RV/LA 002 both `bad_marker_prefix_lines=0`。
- Noise: final RV/LA 002 both `AxError::NotADirectory=12`, `axfs::fops:297=0`, `axfs_ramfs::file:69=12`。

## Follow-up objective

Primary: stable470 (+10 real clean cases) if clean subset and final-gate budget permit.
Stretch: stable480 only if discovery produces enough low-risk RV+LA x musl+glibc clean cases.

## Mandatory startup

1. Read `AGENTS.md` and this prompt.
2. Disk preflight: `df -h / /root`; `du -sh /root/.codex`.
3. `git status --short`; do not revert user files or root remote-output logs.
4. Recompute live `LTP_STABLE_CASES` count/duplicates from `examples/shell/src/cmd.rs`.
5. Reparse stable460 final summaries with `python3 -B scripts/ltp_summary.py` and confirm final gates are still the latest trusted evidence.
6. Create a new dated docs directory if needed: `docs/ltp-score-improvement-2026-05-27-phase-b/` or the current local date/phase.
7. Create a new Ultragoal brief and plan; Team workers may do discovery/repair/verification reports, but leader owns `.omx/ultragoal`, stable-list edits, and aggregate gates.

## Clean reserves from stable460 discovery

These are not automatically promotable; rerun fresh RV+LA x musl+glibc targeted gates before use:

- `mknod08`
- `mknodat01`
- `rename14`

Blocked/demoted rows to handle carefully:

- `kill02`: targeted scout was clean, but LA aggregate stable460 first attempt failed both musl/glibc with `TBROK=4` per libc due child setup timeout (`kill02.c:289` and `kill02.c:496`). Do not promote from targeted-only evidence.
- `readlinkat02`: LA musl failed with `TFAIL=1` in `raw/stable460-clean13-la-confirm-001-summary.txt`.

## Candidate direction

Prefer low-risk adjacent cases first:

- mknod/mknodat/rename/path rows near the clean reserves.
- FD/fcntl/pipe rows adjacent to stable440/stable452 behavior.
- Credential/chown/fchown rows if source-level expectations are narrow.
- Avoid high-risk `kill02` until the LA aggregate setup timeout is understood.
- Avoid mmap/fs-suite broad stress unless low-risk pools under-yield.

## Promotion gate rules

Only add cases after targeted matrix proves RV+LA x musl+glibc clean. After any stable-list edit:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py <logs>
```

Stop/demote on wrapper FAIL, internal TFAIL/TBROK/TCONF beyond known `read02`, timeout, ENOSYS/not implemented, panic/trap, or marker-prefix pollution. Never hardcode case names, modify LTP tests, or convert real failures into SKIP/TCONF/PASS.
