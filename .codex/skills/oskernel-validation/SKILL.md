---
name: oskernel-validation
description: Plan and run layered build, lint, test, QEMU, and regression validation for OSKernel changes, with evidence-to-claim boundaries and bounded-regression records. Use for generic verification selection and completion evidence; do not use for kernel semantic design, LTP case selection/scorer interpretation, cross-architecture delivery diagnosis, or repository cleanup.
---

# OSKernel Validation

Validate the smallest claim that proves the requested behavior, then expand only when risk requires it.

## Workflow

1. Establish the changed surface, user-visible claim, risk, prerequisites, and available environment.
2. Select the narrowest applicable layer from [validation layers and commands](references/validation-layers-and-commands.md).
3. Run dependent checks sequentially: static checks, targeted build/test, adjacent regression, then runtime/evaluator evidence when the claim requires it.
4. Read every result. Fix failures and rerun the affected layer; never convert missing evidence into a pass.
5. Report exact commands, outcomes, gaps, behavior/ABI impact, and rollback boundary using [regression and evidence](references/regression-and-evidence.md).

## Boundaries and handoffs

- Hand kernel architecture, syscall/errno/ABI, unsafe, and user-copy design to `$oskernel-kernel-engineering` before validating it.
- Hand LTP selection, stable promotion, blacklist, and scorer interpretation to `$oskernel-compatibility-evaluation`.
- Hand RV/LA local-versus-remote configuration and offline submission delivery to `$oskernel-cross-arch-delivery`.
- Hand generated artifacts, dirty baselines, disk pressure, caches, and safe cleanup to `$oskernel-repo-hygiene`.
- Do not modify testsuites or evaluators to hide a failure, and do not claim unrun QEMU/remote checks.

## Stop condition

Stop when the requested claim has fresh supporting evidence, regressions are bounded, and every unrun or failed check is explicitly visible.
