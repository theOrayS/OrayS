# Hard-blocker report: RV CVE/OOM and LA crash/trap

Report lane: `docs/ltp-score-improvement-2026-05-22`
Generated: 2026-05-21
Owner: worker-5
Scope: non-blocking evidence report for hard blockers outside the current stable/core LTP promotion lane. This report does not mark any hard-blocker case as passed, does not hide failures, and does not recommend broad speculative fixes without fresh single-case validation.

## Executive summary

- Stable/core promotion should remain independent from these blockers: the current core evidence reports `PASS LTP CASE: 32`, `FAIL LTP CASE: 0`, internal `TCONF: 2`, and `ENOSYS: 0` for both `output_la.md` and `output_rv.md` in the prior triage snapshot (`docs/ltp-score-improvement-2026-05-21/syscall-hardblocker-triage.md:7-12`).
- RV full-LTP stopped in the CVE region, not in the stable/core list: the full report records `242` started cases, last case `cve-2017-17053`, no group end, and stop reason `guest memory exhaustion during cve-2017-17052; glibc LTP could not map ELF afterward` (`eval-reports/full-ltp-20260519-132237/full-ltp-report.md:13,20-23`).
- LA full-LTP stopped at `crash01`: the full report records `220` started cases, last case `crash01`, no group end, and a kernel panic/unhandled trap stop reason (`eval-reports/full-ltp-20260519-132237/full-ltp-report.md:14,30-33`).
- No repo files named `docs/ltp-score-improvement-2026-05-21/rv-cve*` or `docs/ltp-score-improvement-2026-05-21/la-crash01*` exist in this checkout; the hard-blocker source document found is `docs/ltp-score-improvement-2026-05-21/syscall-hardblocker-triage.md`.

## Current promotion impact

These hard blockers should not block stable/core promotion unless a proposed promoted batch actually includes one of the hard-blocker cases or depends on the same failure mode.

| Area | Promotion impact | Reason |
| --- | --- | --- |
| RV `cve-2017-17052` / `cve-2017-17053` | Non-blocking for current stable/core | The stable/core snapshot is already green except the known `chdir01` symlink-loop `TCONF`; the CVE failure appears only in the older broad/full-LTP run (`syscall-hardblocker-triage.md:7-12,118-131`). |
| RV non-x86 CVE `TCONF` handling | Non-blocking; classify separately from failure | The old full-LTP log has legitimate architecture gating such as `TCONF: not (i386 or x86_64)` before later CVE cases (`rv.full-ltp.output.md:4503-4509`). This should be reported as TCONF/configuration, not kernel failure. |
| LA `crash01` trap | Non-blocking for current stable/core | `crash01` is a separate robustness/fault-injection case and is not part of the stable/core list used for the current score lane. |

## RV CVE/OOM evidence

### What happened

The RV full run entered the CVE region and then exhausted guest frames during `cve-2017-17052`. Early in the case, free frames were still plentiful: immediately before the case body there were `123182` to `123934` free frames in process-teardown diagnostics (`rv.full-ltp.output.md:4578-4588`). Much later in the same case, the log repeatedly reached `free_frames=0` and emitted both COW and fresh frame allocation failures (`rv.full-ltp.output.md:22234-22266`).

The decisive stop pattern is:

- COW allocation failures in `axmm::backend::alloc` with `free_frames=0` (`rv.full-ltp.output.md:22236,22243,22248,22255`).
- Fresh page allocation failures in the same backend with `free_frames=0` (`rv.full-ltp.output.md:22245-22247,22257-22262`).
- Fork failures returning `EFAULT` after `clone_user_mappings_from` / `map_shared` errors (`rv.full-ltp.output.md:22238-22242,22250-22254`).
- LTP-visible memory failures: `mmap(... 16777216 ...) failed: ENOMEM` (`rv.full-ltp.output.md:22263,22266`).
- Case timeout and final failed status for `cve-2017-17052` (`rv.full-ltp.output.md:22308-22325`).
- `cve-2017-17053` begins after the system is already under severe memory pressure, then glibc loader startup fails because a populated executable mapping cannot allocate a frame (`rv.full-ltp.output.md:22326-22338`).

### Likely ownership boundaries

Low-level memory and fork ownership should be kept separate from LTP harness/status reporting:

- `kernel/memory/axmm/src/backend/alloc.rs`: COW and fresh-frame allocation failures map directly to the logged messages from the failure (`handle_page_fault_alloc` warnings around COW allocation and fresh allocation).
- `kernel/memory/axmm/src/aspace.rs`: `clone_user_mappings_from` is the path that logs cloned-area map failures and returns mapping errors during fork pressure.
- `examples/shell/src/uspace/process_lifecycle.rs`: `MIN_FORK_FREE_FRAMES`, `ensure_user_task_capacity`, fork setup, timeout handling, and process teardown define the user-process pressure envelope.
- `examples/shell/src/uspace/memory_map.rs`: `sys_mmap`, `sys_munmap`, and `forget_mmap_range` affect per-process mapping accounting.
- `examples/shell/src/uspace/sysv_shm.rs`: SysV SHM attachment/removal can become relevant if future CVE probes prove shared-memory retention is part of the pressure, but the current log alone does not prove that.

### Safe vs unsafe implementation slices

Low-risk next steps:

1. Run only the relevant RV cases with memory counters enabled by the existing harness:
   - `LTP_CASES='cve-2017-17052,cve-2017-17053' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh`
   - `python3 scripts/ltp_summary.py output_rv.md`
2. Compare `LTP MEMORY <case> before`, `after_run`, and `after_cleanup` lines to decide whether the problem is leak/cleanup, intentional stress exhaustion, or a fork pressure threshold issue.
3. If the reproduction reaches `free_frames=0`, add temporary diagnostic-only probes in a follow-up implementation lane; remove them before merge unless they are useful durable telemetry.

High-risk changes to avoid without fresh reproduction:

- Rewriting COW behavior or broad `clone_user_mappings_from` semantics.
- Raising memory limits or lowering fork limits just to make the case pass.
- Treating `ENOMEM`/timeout as PASS or silently skipping these cases.
- Freeing SysV SHM removed segments without attachment refcount proof; the current code intentionally keeps removed backing objects alive to avoid freeing pages under live mappings.

Confidence: **medium-high** that the old RV stop was real memory pressure/OOM, not a parser or top-level wrapper artifact. Confidence is **medium** on the exact fix slice until a current-source single-case run distinguishes leak from intended stress.

## RV/LA `cve-2017-17053` and non-x86 `TCONF` handling

There are two separate CVE-related facts that must not be conflated:

1. `cve-2017-17053` exists in both current sdcard images for musl (`debugfs -R 'ls -p /musl/ltp/testcases/bin' /root/oskernel2026-orays/sdcard-{rv,la}.img` shows `cve-2017-17053`).
2. The captured full-LTP logs do **not** contain a clean standalone verdict for `cve-2017-17053`: RV starts it only after `cve-2017-17052` has already exhausted frames, and LA stops earlier at `crash01`.

Evidence from the extracted case binaries supports expected non-x86 configuration handling, but this is weaker than source-level or run-level evidence:

- `strings /tmp/worker5-cve17053/rv-cve-2017-17053` includes `no asm/ldt.h header (only for i386 or x86_64)`, `This arch '%s' is not supported for test!`, `TCONF`, `x86_64`, and `cve-2017-17053.c`.
- `strings /tmp/worker5-cve17053/la-cve-2017-17053` includes the same unsupported-arch / `TCONF` strings and source marker.
- `eval-reports/full-ltp-20260519-132237/rv.full-ltp.output.md:22325-22338` proves the observed RV failure was under `free_frames=0` memory pressure, so it cannot be used to classify standalone `cve-2017-17053` behavior.
- `eval-reports/full-ltp-20260519-132237/la.full-ltp.output.md:3722-3768` proves LA never reached the CVE region in the captured full run.

Recommendation: do not infer a kernel bug from architecture-gated `TCONF` strings, and do not infer a standalone `cve-2017-17053` failure from the old full run. Run it in isolation on RV and LA; classify clean unsupported-arch output as `TCONF`, not as a stable-promotion blocker.

Validation commands:

- `LTP_CASES='cve-2017-17053' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh`
- `LTP_CASES='cve-2017-17053' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh la`
- `python3 scripts/ltp_summary.py output_rv.md`
- `python3 scripts/ltp_summary.py output_la.md`

Confidence: **medium**. Binary strings strongly indicate intended non-x86 `TCONF` handling, but this task did not execute a current-source standalone run.

## LA `crash01` / trap evidence

### What happened

The LA full run reached `crash01`, emitted the `crashme` command line, and then panicked in the LoongArch trap path:

- `RUN LTP CASE crash01` (`la.full-ltp.output.md:3722`).
- `crashme +2000.80 721 100` (`la.full-ltp.output.md:3726`).
- Panic at `vendor/axcpu/src/loongarch64/trap.rs:65:13` with `Unhandled trap Exception(InstructionNotExist) @ 0x10000b3020` (`la.full-ltp.output.md:3727-3728`).

The prior triage notes that current source now has a user `InstructionNotExist` path that maps to `SIGILL`, so the old line number may be stale relative to current code (`docs/ltp-score-improvement-2026-05-21/syscall-hardblocker-triage.md:105-116`). Current-source validation should therefore precede edits.

### Likely ownership boundaries

- `vendor/axcpu/src/loongarch64/trap.rs`: classify user instruction faults and choose panic vs user signal path.
- `examples/shell/src/uspace/signal_abi.rs`: `USER_EXCEPTION` converts `SIGILL` / `SIGSEGV` into process termination.
- `examples/shell/src/uspace/process_lifecycle.rs`: signal-triggered exit and teardown must wake waiters and avoid a kernel panic path.

### Safe vs unsafe implementation slices

Low-risk next steps:

1. Run the single current-source LA case:
   - `LTP_CASES='crash01' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh la`
   - `python3 scripts/ltp_summary.py output_la.md`
2. If it still panics, capture whether `loongarch64_trap_handler` received `from_user=true` for `InstructionNotExist`.
3. If `from_user=true`, the safe implementation slice is user-signal routing only. If `from_user=false`, the problem is trap-origin/context preservation and is higher risk.

High-risk changes to avoid without fresh reproduction:

- Broadly downgrading kernel-mode `InstructionNotExist` to user SIGILL.
- Masking all unknown LoongArch traps as user signals.
- Reporting `crash01` as passed merely because the kernel remains alive; the LTP case must produce a real PASS/FAIL/TCONF marker.

Confidence: **medium**. The old log is direct evidence of the panic, but current code appears to have changed in the relevant trap path.

## Validation plan for future implementation lanes

Minimum validation after any hard-blocker change:

1. Targeted single-case run:
   - RV: `LTP_CASES='cve-2017-17052,cve-2017-17053' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh`
   - LA: `LTP_CASES='crash01' LTP_CASE_TIMEOUT_SECS=20 ./run-eval.sh la`
2. Summary classification:
   - `python3 scripts/ltp_summary.py output_rv.md`
   - `python3 scripts/ltp_summary.py output_la.md`
3. Regression guard for stable/core promotion:
   - `./run-eval.sh`
   - `./run-eval.sh la`
4. Log checks:
   - `rg --text -n "free_frames=0|failed to map ELF segment|InstructionNotExist|Unhandled trap|TIMEOUT LTP CASE|PASS LTP CASE|FAIL LTP CASE|TCONF" output_rv.md output_la.md`
5. Non-x86 `cve-2017-17053` classification check if image inspection is needed:
   - `debugfs -R 'dump /musl/ltp/testcases/bin/cve-2017-17053 /tmp/rv-cve-2017-17053' /root/oskernel2026-orays/sdcard-rv.img`
   - `debugfs -R 'dump /musl/ltp/testcases/bin/cve-2017-17053 /tmp/la-cve-2017-17053' /root/oskernel2026-orays/sdcard-la.img`
   - `strings /tmp/*-cve-2017-17053 | rg -n 'TCONF|x86|arch|not supported|cve-2017-17053|asm/ldt'`

## Stop condition

This report is complete when it is treated as a non-blocking hard-blocker map: it separates RV memory pressure from LA trap panic, preserves internal `TCONF` classification, identifies file ownership boundaries, and avoids speculative fixes that could destabilize stable/core promotion.


## Commands run while producing this report

```sh
cat /root/.omx-runs/run-20260521082245-b541/.omx/state/team/ltp-score-improvement-8231c807/tasks/task-10.json
find docs/ltp-score-improvement-2026-05-21 -maxdepth 1 -type f ...
nl -ba docs/ltp-score-improvement-2026-05-21/syscall-hardblocker-triage.md
nl -ba docs/ltp-score-improvement-2026-05-21/discovery-candidates.md
nl -ba eval-reports/full-ltp-20260519-132237/full-ltp-report.md
nl -ba eval-reports/full-ltp-20260519-132237/{rv,la}.full-ltp.output.md
rg -n -C ... 'cve-2017-17052|cve-2017-17053|free_frames=0|failed to map ELF segment|InstructionNotExist|crash01|TCONF|x86'
debugfs -R 'ls -p /musl/ltp/testcases/bin' /root/oskernel2026-orays/sdcard-{rv,la}.img
debugfs -R 'dump /musl/ltp/testcases/bin/cve-2017-17053 /tmp/worker5-cve17053/<arch>-cve-2017-17053' /root/oskernel2026-orays/sdcard-{rv,la}.img
strings /tmp/worker5-cve17053/{rv,la}-cve-2017-17053 | rg -n 'TCONF|x86|arch|not supported|cve-2017-17053'
python3 scripts/ltp_summary.py eval-reports/full-ltp-20260519-132237/la.full-ltp.output.md | sed -n '1,8p'
python3 scripts/ltp_summary.py eval-reports/full-ltp-20260519-132237/rv.full-ltp.output.md | sed -n '1,8p'
git diff --check
```

## Files changed

- `docs/ltp-score-improvement-2026-05-22/hard-blocker-report.md` — updated non-blocking hard-blocker report only.
