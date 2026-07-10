---
name: oskernel-compatibility-evaluation
description: Use for LTP case selection, stable promotion, blacklist sweeps, score interpretation, or evaluator result reporting in OSKernel. Exclude kernel implementation, generic validation, and RV/LA delivery configuration.
---

# OSKernel Compatibility Evaluation

Use this capability as a specialized compatibility lane. Quality and truthful Linux/POSIX semantics remain the governing priorities; score and case count are evidence, not the default task sorter.

## Workflow

1. Identify the requested decision: candidate selection, stable promotion, blacklist sweep, or evaluator interpretation.
2. Read live repository facts and the narrow reference below; never reuse a remembered stable count or stale score.
3. Separate wrapper status from inner `TPASS`/`TFAIL`/`TBROK`/`TCONF`, timeout, `ENOSYS`, panic, and trap evidence.
4. Recommend or promote only from real semantics and reproducible evidence. A skipped or blacklisted case is never a pass.
5. Report the observed gap, evidence paths, validation boundary, architecture/libc coverage, and unresolved failures.

## References

- For candidate evidence and quality-first ranking, read [LTP selection](references/ltp-selection.md).
- For stable truth, promotion gates, and blacklist accounting, read [LTP promotion and blacklist](references/ltp-promotion-and-blacklist.md).
- For scorer contracts and compatibility reports, read [Evaluator reporting](references/evaluator-reporting.md).

## Boundaries and handoffs

- Hand kernel or libc semantic repairs to `$oskernel-kernel-engineering` and request `$oskernel-validation` after the patch.
- Hand RV/LA boot, platform configuration, offline toolchain, or submission-artifact issues to `$oskernel-cross-arch-delivery`.
- Hand branch ownership, freeze, staging, commit, and review to `$oskernel-collaboration-delivery`.
- Do not modify testsuites or evaluator scripts to manufacture success, specialize behavior to a case/path/process, or hide any failure class.

## Stop condition

Stop when the requested compatibility decision is backed by current source/log/parser evidence, all skips and failures remain visible, and promotion or follow-up gates are explicit.
