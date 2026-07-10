---
name: oskernel-experimental-features
description: "Use when planning, isolating, evaluating, promoting, continuing, or retiring an experimental OSKernel feature; exclude ordinary bug fixes, production-ready kernel implementation, compatibility score campaigns, and release-branch delivery."
---

# OSKernel Experimental Features

Use this skill when an idea is intentionally experimental and needs a measurable path to promotion, continuation, or
retirement. It defines lifecycle governance only; it does not implement an experimental kernel feature.

## Workflow

1. Open [the experimental feature lifecycle](references/experimental-feature-lifecycle.md) and name the proposal,
   hypothesis, owner, affected subsystem, and rollback boundary.
2. Follow every stage in the exact order shown there. Do not skip a gate because a benchmark or evaluator score rises.
3. Keep the experiment isolated and failures visible. Never count blacklist/skip behavior as a semantic pass.
4. Request domain evidence from `$oskernel-kernel-engineering`, `$oskernel-validation`,
   `$oskernel-cross-arch-delivery`, or `$oskernel-compatibility-evaluation` without copying their procedures.
5. At the final gate, record exactly one reviewed decision: Promote, Continue, or Retire.

## Boundaries

- Experiments may be developed on an isolated `exp/` branch or feature gate, but `exp/` is not permanently
  non-promotable: promotion is allowed only after all gates pass.
- `score/best` is historical/compatibility evidence, not the default architecture target or automatic merge source.
- Do not weaken correctness, permission/resource checks, safety boundaries, honest errno, failure visibility, or the
  root compliance contract to meet an experimental metric.
- Ordinary subsystem implementation belongs to `$oskernel-kernel-engineering`; branch integration, freeze, staging,
  review, and Lore delivery belong to `$oskernel-collaboration-delivery`.

## Stop Condition

Stop only when all completed stages have their required artifact and exit evidence, the current owner and next action
are explicit, and the final decision either supplies a promotion handoff, a bounded continuation deadline, or verified
retirement and cleanup. An experiment with no decision, owner, rollback, or failure destination remains incomplete.
