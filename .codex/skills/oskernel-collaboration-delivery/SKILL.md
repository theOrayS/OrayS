---
name: oskernel-collaboration-delivery
description: Use for branch strategy, multi-agent ownership, freeze and review gates, delivery reports, selective staging, or Lore commits in OSKernel. Exclude repository cleanup, kernel semantics, and test selection.
---

# OSKernel Collaboration and Delivery

Use this capability to coordinate bounded ownership and deliver verified repository changes without overwriting unrelated work.

## Workflow

1. Read the live branch, worktree, task ownership, and dirty-state facts.
2. Establish an explicit file allowlist and handoffs before edits; do not absorb another agent's scope.
3. Apply the appropriate branch, freeze, review, and integration gates.
4. Report actual behavior and verification boundaries, then stage only owned paths.
5. Inspect the cached diff and create the requested Lore commit after relevant checks pass.

## References

- For branch roles, ownership, freeze, and review, read [Branches, freeze, and review](references/branches-freeze-and-review.md).
- For reports, selective staging, and Lore commits, read [Delivery and Lore](references/delivery-and-lore.md).
- For historical source-to-destination audit metadata only, read [Legacy guidance migration](references/legacy-guidance-migration.md). It is not a runbook.

## Boundaries and handoffs

- Hand disk pressure, generated artifacts, caches, and cleanup to `$oskernel-repo-hygiene` before delivery.
- Hand generic test selection and bounded-regression design to `$oskernel-validation`.
- Hand LTP promotion, blacklist, score, or scorer interpretation to `$oskernel-compatibility-evaluation`.
- Never revert, overwrite, format, stage, or commit another owner's unrelated changes.

## Stop condition

Stop when ownership is closed, reviews and verification support the claim, only allowlisted changes are staged, the commit follows Lore, and remaining risks or untested items are explicit.
