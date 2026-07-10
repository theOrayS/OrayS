<a id="technical-debt-and-decisions"></a>
# Technical Debt and Decisions

<a id="technical-debt-and-decisions-when-to-record-a-decision"></a>
## When to Record a Decision

Create or update a repo-visible ADR/design note when a change moves a subsystem boundary, introduces a durable ABI or
dependency constraint, selects among materially different architectures, or deliberately accepts a bounded
regression. Routine local fixes need only a clear commit rationale and report.

<a id="technical-debt-and-decisions-minimal-adr-record"></a>
## Minimal ADR Record

- **Context:** current behavior, invariant, and measured problem.
- **Decision:** chosen boundary and why it matches the canonical project identity.
- **Alternatives:** credible options rejected and their concrete costs.
- **Semantics:** user/kernel-visible behavior, failure shape, and compatibility impact.
- **Risk:** safety, resource, concurrency, multi-architecture, and rollback exposure.
- **Validation:** owner and evidence required before integration and release.
- **Reversal trigger:** observation that invalidates the decision.

Do not encode a testcase-specific workaround as an architectural decision.

<a id="technical-debt-and-decisions-technical-debt-record"></a>
## Technical-Debt Record

Every accepted debt item needs an owner, affected subsystem, observable consequence, severity, evidence link, removal
or review condition, deadline/release target, and rollback/containment. Avoid vague debt entries such as "improve later".

Debt may not authorize fake success, missing permission/resource checks, hidden failure, unsafe user-copy, corrupted
data, or a known crash. Those are correctness defects and remain blocking.

<a id="technical-debt-and-decisions-bounded-regression-handoff"></a>
## Bounded Regression Handoff

If a general semantic or architectural repair temporarily regresses a metric or compatibility surface, hand the
record to `$oskernel-validation`. The record must include baseline and capture time, quantified delta and scope,
correctness evidence, repo-visible rationale, owner, deadline, rollback, visible failure reporting, and release
closure. A score improvement alone does not close architectural debt.
