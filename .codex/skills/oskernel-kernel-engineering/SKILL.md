---
name: oskernel-kernel-engineering
description: "Use when designing or changing OSKernel architecture, kernel runtime, Rust/unsafe code, Linux/POSIX semantics, user-pointer boundaries, or technical debt; exclude evaluator campaign planning, repository cleanup, release delivery, and experiment lifecycle management."
---

# OSKernel Kernel Engineering

Use this skill for implementation or design work in kernel subsystems, Linux/POSIX-visible interfaces, runtime paths,
and architecture decisions. Preserve the root `AGENTS.md` invariants throughout; this skill adds the engineering
procedure but does not relax compliance or ownership boundaries.

## Workflow

1. State the intended semantic or architectural outcome and the smallest owned subsystem boundary.
2. Map the affected layer and invariants with [architecture and semantics](references/architecture-and-semantics.md).
3. Apply the relevant checks from [high-risk change boundaries](references/high-risk-change-boundaries.md) before editing.
4. Prefer a local, reversible patch that reuses current interfaces and preserves feature and architecture structure.
5. Record durable tradeoffs or deferred liabilities with [technical debt and decisions](references/technical-debt-and-decisions.md).
6. Hand verification design to `$oskernel-validation`; hand RV/LA or remote delivery work to
   `$oskernel-cross-arch-delivery` rather than duplicating those procedures.
7. Report changed files, semantic effect, validation evidence, residual risk, and every relevant ABI-visible field.

## Boundaries

- Treat raw user pointers, lengths, flags, and ABI structs as untrusted input; validate before conversion or access.
- Unsupported capability must fail honestly with the correct error shape. Never substitute success, hide a failure, or
  specialize behavior for a test name, path, binary, process, fixed input, or evaluator environment.
- Keep new `unsafe` blocks narrow and document non-obvious invariants with `// SAFETY:`.
- Do not perform unrelated cross-subsystem refactors, bulk formatting, dependency changes, evaluator changes, or
  generated-artifact edits.
- Compatibility campaigns, stable-case promotion, blacklist selection, and score interpretation belong to
  `$oskernel-compatibility-evaluation`.
- A proposed high-risk idea whose semantics are not yet integration-ready belongs to
  `$oskernel-experimental-features` until it passes that lifecycle.

## Stop Condition

Stop only when the requested semantics are implemented or the honest unsupported boundary is explicit, the smallest
relevant verification is fresh, and the report states syscall/errno/flag/struct-layout/FD/signal/futex/mmap/user-copy,
runtime, build, evaluator, and RV/LA effects (including explicit "no change" entries where applicable).
