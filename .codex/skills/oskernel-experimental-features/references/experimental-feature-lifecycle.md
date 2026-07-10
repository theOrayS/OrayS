<a id="experimental-feature-lifecycle"></a>
# Experimental Feature Lifecycle

<a id="experimental-feature-lifecycle-mandatory-gate-order"></a>
## Mandatory Gate Order

`Proposal → Isolation/feature gate → Evidence → Validation/observability → performance/resource budget →`
`RV/LA regression → maintainability review → Promote/Continue/Retire`.

Each stage is mandatory. A failed exit condition follows the stated failure destination; it never silently advances.

| Stage | Minimum artifact | Exit condition | Owner | Failure destination |
| --- | --- | --- | --- | --- |
| Proposal | Written hypothesis, intended general semantics, affected subsystem, risk class, success/stop metrics, owner, deadline, and rollback boundary | Review confirms the problem is real, the metric can falsify the idea, and no testcase/environment specialization is proposed | Proposal owner plus kernel-domain reviewer | Reject or revise the proposal before code work |
| Isolation/feature gate | Dedicated `exp/` branch or disabled-by-default feature/config gate, baseline SHA/status, owned path list, activation and rollback instructions | Default behavior is unchanged, activation is explicit, cleanup is reproducible, and unrelated dirty state is protected | Implementation owner | Restore baseline and redesign isolation |
| Evidence | Reproducible workload, baseline capture time/configuration, raw results, correctness observations, and negative/adversarial cases | Evidence supports or falsifies the hypothesis without hiding failures, treating skips as passes, or relying on fixed evaluator details | Experiment owner with evidence reviewer | Continue evidence collection within deadline or Retire |
| Validation/observability | Targeted tests, relevant regression plan, visible error/timeout/panic/trap reporting, diagnostics, and rollback smoke proof | Correctness and failure visibility pass; claimed behavior is bounded by fresh evidence | Validation owner via `$oskernel-validation` | Fix in isolation, narrow the claim, or Retire |
| performance/resource budget | Before/after measurements for time, memory, storage, CPU, and relevant contention plus declared acceptable ceilings | Improvement is real under correct semantics, resource ceilings hold, and no unmeasured tradeoff invalidates the claim | Performance/resource owner | Optimize within deadline, Continue with reviewed bounds, or Retire |
| RV/LA regression | RV and LA build/runtime evidence at the appropriate local/remote boundary, config identity, and explicit unverified gaps | Both architectures meet the declared contract, or a reviewed architecture scope limit is documented without false parity claims | Cross-architecture owner via `$oskernel-cross-arch-delivery` | Fix architecture split, narrow scope, or Retire |
| maintainability review | Design/ADR or debt record, diff/scope review, unsafe/dependency/config audit, operational burden, rollback cost, and deletion plan | Independent reviewer finds the design understandable, bounded, supportable, and free of testcase-specific behavior | Kernel maintainer independent of primary implementer | Simplify/redesign, Continue with owner/deadline, or Retire |
| Promote/Continue/Retire | Signed decision record linking every prior artifact and listing owner, date, rationale, residual risk, and next action | Exactly one disposition is approved and its exit actions are complete | Maintainer/release authority | Remain isolated; no implicit promotion |

<a id="experimental-feature-lifecycle-dispositions"></a>
## Dispositions

<a id="experimental-feature-lifecycle-promote"></a>
### Promote

Hand the reviewed patch and evidence to `$oskernel-collaboration-delivery`. Promotion requires the normal integration,
review, freeze, staging, and Lore process; an experimental branch or high score is not itself promotion evidence.

<a id="experimental-feature-lifecycle-continue"></a>
### Continue

Keep isolation active and record the unanswered question, next falsifiable milestone, owner, deadline, resource ceiling,
rollback state, and automatic retirement trigger. Repeating “continue” without new evidence is not progress.

<a id="experimental-feature-lifecycle-retire"></a>
### Retire

Disable and remove activation paths and experimental artifacts that are safe to delete, restore the declared baseline,
preserve the decision/evidence record, and verify that default behavior and owned repository state are clean. A retired
experiment must not leave an undocumented feature gate, dependency, config change, or score claim behind.

<a id="experimental-feature-lifecycle-cross-domain-evidence"></a>
## Cross-Domain Evidence

Kernel semantics are owned by `$oskernel-kernel-engineering`; validation and bounded-regression records by
`$oskernel-validation`; architecture/platform proof by `$oskernel-cross-arch-delivery`; specialized LTP/evaluator
interpretation by `$oskernel-compatibility-evaluation`; repository cleanup by `$oskernel-repo-hygiene`; and promotion
delivery by `$oskernel-collaboration-delivery`. Link their artifacts rather than reproducing their runbooks.
