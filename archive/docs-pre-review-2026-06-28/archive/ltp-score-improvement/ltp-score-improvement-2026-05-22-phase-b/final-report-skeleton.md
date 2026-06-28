# Phase B final report skeleton: stable101 -> stable120/125

> Fill this report only with verified evidence. Keep timeout, `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, panic/trap, and non-LTP markers separate from PASS counts.

## Executive summary

- Starting stable count: `101 cases / libc / arch`.
- Final stable count: `TODO`.
- Newly promoted cases: `TODO`.
- Target outcome:
  - [ ] Main target stable120 reached.
  - [ ] Minimum stable115 reached.
  - [ ] Stretch stable125+ reached.
  - [ ] Target missed; blockers documented below.
- Final decision: `TODO: PASS / FAIL / BLOCKED`.

## Changed files and ABI/runtime changes

| Area | Files changed | Reason | Risk notes |
| --- | --- | --- | --- |
| Runner/stable list | `TODO` | `TODO` | `TODO` |
| Proc/sched/wait ABI | `TODO` | `TODO` | `TODO` |
| Time/signal ABI | `TODO` | `TODO` | `TODO` |
| FD/pipe/open/lseek/access | `TODO` | `TODO` | `TODO` |
| FS/metadata/statfs/sysinfo | `TODO` | `TODO` | `TODO` |
| Docs/artifacts only | `TODO` | `TODO` | `TODO` |

## Command and exit-code evidence

Record every command with its exact exit code and artifact path.

| Stage | Command | Exit | Artifact(s) | Notes |
| --- | --- | ---: | --- | --- |
| Format | `cargo fmt --all -- --check` | `TODO` | `TODO` | `TODO` |
| Build | `make A=examples/shell ARCH=riscv64` | `TODO` | `TODO` | Required if shell/uspace changed. |
| RV targeted batch | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:<casefile> LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh` | `TODO` | `TODO` | Run before LA unless unsafe. |
| LA targeted batch | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=file:<casefile> LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh la` | `TODO` | `TODO` | Use same case file as RV where applicable. |
| Stable targeted gate | `TODO` | `TODO` | `TODO` | Required after each promotion. |
| Final LA full gate | `./run-eval.sh la 2>&1 | tee output_la.md` | `TODO` | `TODO` | Final only after targeted gates are clean. |
| Final RV full gate | `./run-eval.sh 2>&1 | tee output_rv.md` | `TODO` | `TODO` | Final only after targeted gates are clean. |
| LA summary parse | `python3 -B scripts/ltp_summary.py output_la.md` | `TODO` | `TODO` | Do not rely on wrapper exit alone. |
| RV summary parse | `python3 -B scripts/ltp_summary.py output_rv.md` | `TODO` | `TODO` | Do not rely on wrapper exit alone. |

## LA/RV final summaries

| Arch | PASS LTP CASE | FAIL LTP CASE | musl pass/fail | glibc pass/fail | Notes |
| --- | ---: | ---: | --- | --- | --- |
| LA | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` |
| RV | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` |

## Internal LTP signal accounting

| Arch | TFAIL | TBROK | TCONF | timeout | ENOSYS | panic/trap | Artifact |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| LA | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` |
| RV | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` |

Known transparent caveat:

- `read02` may remain `pass_with_tconf`; if present, record it here and do not call it a clean pass.

## Stable additions

For each promoted case, include LA/RV x musl/glibc evidence and the gate that justified promotion.

| Case | Group | LA musl | LA glibc | RV musl | RV glibc | Internal signals | Promotion artifact |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `TODO` | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` | `TODO` |

## Blocked / rejected cases

| Case | Category | Evidence | Required real fix / next step |
| --- | --- | --- | --- |
| `TODO` | `TFAIL/TBROK/TCONF/timeout/ENOSYS/panic-trap/errno/ABI/other` | `TODO` | `TODO` |

Do not reclassify blocked cases as SKIP or PASS to improve score.

## Non-LTP evaluator markers

Record non-LTP caveats separately from LTP stable success.

| Marker | Seen on | Impact on LTP stable gate | Follow-up |
| --- | --- | --- | --- |
| `TODO` | `TODO` | `TODO` | `TODO` |

## Remote sync

| Step | Command / action | Result | Notes |
| --- | --- | --- | --- |
| Local validation complete | `TODO` | `TODO` | `TODO` |
| Sync to `/root/oskernel2026-orays-remote` | `TODO` | `TODO` | Preserve remote-only address-mapping differences. |
| Remote verification / artifact copy | `TODO` | `TODO` | Do not force local-only gates as remote proof. |

## Quality gate

| Check | Result | Artifact | Notes |
| --- | --- | --- | --- |
| Fake PASS / case-name hardcode audit | `TODO` | `TODO` | Search changed files and runner paths. |
| Timeout-as-PASS audit | `TODO` | `TODO` | Timeout must remain separate. |
| Silent SKIP audit | `TODO` | `TODO` | Real failures must stay visible. |
| Code review | `TODO` | `TODO` | Include reviewer findings or no-findings evidence. |
| AI-slop cleanup / simplification review | `TODO` | `TODO` | Prefer deletion/reuse over new layers. |

## Final quality-gate JSON fields

The final `final-gate-quality-gate.json` should include at least:

```json
{
  "phase": "2026-05-22-phase-b",
  "start_stable_cases_per_libc_arch": 101,
  "final_stable_cases_per_libc_arch": "TODO",
  "newly_promoted_cases": [],
  "la": {
    "pass_ltp_case": "TODO",
    "fail_ltp_case": "TODO",
    "internal_tfail": "TODO",
    "internal_tbrok": "TODO",
    "internal_tconf": "TODO",
    "timeout": "TODO",
    "enosys": "TODO",
    "panic_trap": "TODO"
  },
  "rv": {
    "pass_ltp_case": "TODO",
    "fail_ltp_case": "TODO",
    "internal_tfail": "TODO",
    "internal_tbrok": "TODO",
    "internal_tconf": "TODO",
    "timeout": "TODO",
    "enosys": "TODO",
    "panic_trap": "TODO"
  },
  "read02_pass_with_tconf_documented": "TODO",
  "non_ltp_markers_documented": "TODO",
  "no_fake_pass": "TODO",
  "no_case_name_hardcode": "TODO",
  "no_silent_skip": "TODO",
  "timeout_not_counted_as_pass": "TODO",
  "remote_sync": "TODO",
  "decision": "TODO"
}
```

## Final stop condition

Declare Phase B complete only when all required targeted, promotion, review, final full-gate, parser-summary, and remote-sync evidence is present. If any required gate is unavailable or failed, mark the phase `BLOCKED` or `FAIL` and preserve the blocker evidence.
