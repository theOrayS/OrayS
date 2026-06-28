# Ultragoal brief: LTP stable250 -> stable300

Work directory: `/root/oskernel2026-orays`.

Goal: promote live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` from the refreshed stable250 baseline to exactly stable300.

Required story stages:

1. Baseline refresh and evidence intake: disk/worktree/live count/current stable250 smoke evidence.
2. Discovery + candidate matrix: classify 250->330 candidates by subsystem and RV/LA x musl/glibc evidence.
3. stable270 promotion: fresh targeted evidence for new cases plus stable aggregate gates.
4. stable285 promotion: fresh targeted evidence for new cases plus stable aggregate gates.
5. stable300 promotion and final delivery: final RV/LA stable gates, marker-prefix check, fmt/build, code review, ai-slop-cleaner audit, quality gate JSON, reports, and auto commit.

Hard constraints:

- No fake PASS, no case-name hardcoding, no hiding real TFAIL/TBROK/TCONF/timeout/panic/trap/ENOSYS as SKIP/TCONF/PASS.
- Timeout cannot count as PASS.
- Wrapper success is insufficient; use `python3 -B scripts/ltp_summary.py` or equivalent case matrix for every gate.
- `read02` known `pass_with_tconf` remains transparent; new cases must be clean.
- LTP marker lines must remain at column 0 with no ANSI/color/reset prefix.
- Leader owns `.omx/ultragoal`, final `LTP_STABLE_CASES` edits, promotion decisions, and final verification. Team workers only own assigned discovery/fix/verification/report slices.
- Do not submit root-level kernels/images/large raw logs/user remote output logs by default.

Completion criteria:

- live `LTP_STABLE_CASES` exactly 300 unique cases.
- RV final stable gate: PASS LTP CASE 600, FAIL 0; ltp-musl 300/0; ltp-glibc 300/0.
- LA final stable gate: PASS LTP CASE 600, FAIL 0; ltp-musl 300/0; ltp-glibc 300/0.
- Internal TFAIL=0, TBROK=0; no new TCONF beyond explicitly disclosed known acceptable TCONF; timeout/ENOSYS/panic/trap all 0.
- Marker-prefix check: 0 bad marker lines.
- Final code review and ai-slop-cleaner audit clean.
- Agent-owned tracked changes committed with Lore commit protocol.
