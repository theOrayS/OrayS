<a id="regression-and-evidence"></a>
# Regression and evidence

<a id="regression-and-evidence-evidence-to-claim-contract"></a>
## Evidence-to-claim contract

For each claim, record:

- exact command and working tree/commit;
- start/end or freshness context when results can drift;
- exit status and a concise observed result;
- what the check proves and what it does not prove;
- unrun checks with a concrete reason;
- syscall, errno, flags, layouts, FD, signal, futex, mmap, user-copy, ABI, build, evaluator, and user-visible impact.

Local RV evidence does not prove LA or remote behavior. Build success does not prove runtime semantics. Wrapper completion does not replace inner test results. A skipped, blacklisted, timed-out, `TCONF`, `TBROK`, `TFAIL`, `ENOSYS`, panic, or trap result remains visible.

<a id="regression-and-evidence-bounded-regression-record"></a>
## Bounded-regression record

Use a bounded regression only when a broader check is currently impractical and the residual risk is explicit. Record:

1. baseline and observation time;
2. changed scope and measured delta;
3. correctness evidence and failure visibility;
4. repo-visible rationale;
5. owner, deadline, rollback or containment action;
6. release/promote closure condition.

Do not use a bounded regression to waive safety, compatibility, fake-pass, or data-integrity requirements. An expired bound becomes a blocker until renewed with new evidence or closed.

<a id="regression-and-evidence-completion-report"></a>
## Completion report

Report changed files and intent, PASS/FAIL commands, unverified items, visible semantics/ABI impact, risk, and rollback. State explicitly when workflow-only changes did not touch kernel/build/evaluator behavior.
