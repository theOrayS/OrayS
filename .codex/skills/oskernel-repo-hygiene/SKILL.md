---
name: oskernel-repo-hygiene
description: Inspect and safely maintain OSKernel repository layout, generated/local evidence boundaries, dirty worktree baselines, disk pressure, Codex/OMX caches, and cleanup safety. Use before destructive cleanup or long resource-heavy work; do not use for branch strategy, staging/commits, kernel semantics, generic test selection, or evaluator score interpretation.
---

# OSKernel Repository Hygiene

Protect source, evidence, active state, and other owners' changes while restoring a safe workspace.

## Workflow

1. Inspect the repository/worktree identity, `git status --short`, owned paths, and generated/local artifacts using [layout and generated artifacts](references/layout-and-generated-artifacts.md).
2. Before long or storage-heavy work, run the disk gate in [worktree, disk, and cleanup](references/worktree-disk-and-cleanup.md).
3. Classify candidates as task-owned, reproducible transient, required evidence, active runtime state, credentials/memory, or another owner's data.
4. Prefer the smallest reversible cleanup; never delete from category guesses alone.
5. Recheck Git status, disk usage, active OMX state, and required evidence after cleanup.

## Boundaries and handoffs

- Hand branches, freezes, staging, commits, review, and Lore delivery to `$oskernel-collaboration-delivery`.
- Hand build/test selection to `$oskernel-validation` and architecture delivery to `$oskernel-cross-arch-delivery`.
- Do not edit generated artifacts, local evidence, vendor/tool closure, source, or another worktree merely to make status or disk output look clean.
- Preserve skills, prompts, agents, memories, authentication, active sessions, active `.omx` state, and evidence needed for reproduction unless explicit authority says otherwise.

## Stop condition

Stop when the requested workspace/resource condition is met, protected data is intact, and status/disk/runtime evidence records what changed and what remains.
