# LTP stable score improvement next context

## Task statement
Continue improving `/root/oskernel2026-orays` LTP stable score with Ultragoal + Team. Start from the completed stable-63 baseline and promote only freshly validated cases toward 80-100 cases per libc/arch, using targeted validation before final full evaluator gates.

## Desired outcome
- Prefer a low-risk promotion from 63 to roughly 75-85 stable cases first; expand further only when evidence stays clean.
- Preserve real semantics: no fake PASS, no case-name hardcode, no silent SKIP, no timeout counted as PASS.
- Every promoted case must have LA/RV × musl/glibc evidence, internal TFAIL/TBROK/TCONF accounting, timeout, ENOSYS, and panic/trap classification.

## Baseline evidence already read
- Previous Ultragoal: `.omx/ultragoal/goals.json` reports 10/10 complete; current Codex goal is clear (`get_goal` returned null).
- Stable batch: 63 cases per libc/arch in `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Final gate: `docs/ltp-score-improvement-2026-05-22/final-gate-report.md` reports LA/RV `run-eval` exit 0, stable LTP 126 PASS / 0 FAIL per arch, ltp-musl 63/0, ltp-glibc 63/0.
- Internal signal: `read02` remains transparent pass_with_tconf (TCONF=2 per libc, 4 per arch total). Do not hide it.
- Final summaries show LTP group timeout 0; total `timeout matches: 10` are from non-LTP benchmark groups.
- Candidate report from 2026-05-22 had 62 clean candidates and 26 blocked/incomplete; clean candidates were effectively the current stable set except `read02` is categorized pass_with_tconf, so this run must discover or repair additional cases with fresh targeted evidence.

## Constraints
- Do not edit generated kernels/images/logs except intentional evaluator outputs (`output_la.md`, `output_rv.md`) and new docs under `docs/ltp-score-improvement-2026-05-23/`.
- Do not modify `.omx/ultragoal` from workers; leader owns Ultragoal checkpoints.
- Targeted batches before final full gates; final full gates only after promotion is justified.
- Use `scripts/ltp_summary.py`; do not rely on run-eval exit code only.
- Timeout is a failure signal and must be counted separately.

## Candidate/risk map
Priority candidates for targeted validation/repair:
- Low-risk getter/wait/exit/proc/read-only metadata neighbors not already in stable.
- Time/signal basics with clear semantics (`nanosleep*`, `clock_gettime01`, `sigprocmask*`, `sigsuspend01`, `pause01`) only after fresh evidence; known blocked cases must not be promoted on stale or partial evidence.
- FS metadata/open/link/rename/statfs/access variants only after real ABI/errno fixes; do not fake fs/statfs/sysinfo data.

Explicit blocked/not-promoted from prior evidence:
`access02 access04 clock_getres01 clock_gettime01 dup03 fstatfs01 getpgid01 getsid01 kill02 link02 lseek02 mkdir02 nanosleep01 nanosleep02 pause01 pipe02 read02 rename01 rt_sigprocmask01 sigpending02 sigprocmask01 sigsuspend01 statfs01 statvfs01 sysinfo01 unlink05`.
`read02` is already stable but has TCONF and must stay transparent.

## Likely touchpoints
- Runner/stable list: `examples/shell/src/cmd.rs`
- Summary/reporting: `scripts/ltp_summary.py`
- ABI fixes only from fresh targeted logs: `examples/shell/src/uspace/{syscall_dispatch.rs,fd_table.rs,metadata.rs,synthetic_fs.rs,memory_map.rs,process_lifecycle.rs,signal_abi.rs,time_abi.rs}`

## Initial execution plan
1. Create fresh Ultragoal plan and aggregate Codex goal.
2. Launch Team with lanes: discovery, stats/report, runner/harness, syscall/ABI, verification/review.
3. Discovery enumerates image testcases and selects 20-40 candidate cases not in current stable; leader selects 8-20 for first targeted batch.
4. Run targeted batches for RV and LA with `OSCOMP_TEST_GROUPS=ltp LTP_CASES=<comma-list> LTP_CASE_TIMEOUT_SECS=8 make run-* ...`, save raw logs/status/summary JSON/TXT.
5. Only promote cases that are clean on LA/RV × musl/glibc with no TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap, except explicitly transparent existing `read02`.
6. Run promoted stable targeted gate on LA/RV.
7. Final gate: `cargo fmt --all -- --check`, `./run-eval.sh la`, `./run-eval.sh`, then `scripts/ltp_summary.py` on both logs.
8. Final docs + quality-gate JSON; checkpoint Ultragoal story completion.
