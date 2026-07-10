<a id="branch-purpose"></a>
# Branches, freeze, and review

Choose branches by delivery purpose, risk, and integration boundary, not by score alone. Inspect `git branch --show-current`, `git status --short`, relevant refs, and current evidence before creating, switching, renaming, or integrating a branch.

<a id="work-priority"></a>
## Work priority

Prioritize correctness, security, release blockers, and regressions in already supported Linux/POSIX behavior. Next prefer bounded compatibility or maintainability work with current evidence and a clear validation plan. Exploratory and cleanup work remains legitimate when isolated, owned, and justified; it does not become lower priority merely because it lacks immediate score impact. Freeze rules may narrow this order for a release window.

<a id="branch-roles"></a>
## Branch roles

- `main`: stable integration line with build, boot, and baseline-test evidence appropriate to the change.
- `dev`: optional shared integration buffer for coordinated parallel work.
- `feat/<subsystem>-...` and `fix/<subsystem>-...`: bounded feature and repair lanes.
- `exp/<name>-...`: isolated exploration. It may advance after explicit correctness, regression, maintainability, and owner gates; it is not permanently non-promotable.
- `release/<stage-or-date>`: frozen delivery line that accepts only authorized, bounded, reversible changes.

A score snapshot may be preserved as a tag or branch, but it does not outrank correctness or become the default integration target. Never delete or rewrite local/remote history without explicit authority.

<a id="ownership-contract"></a>
## Multi-agent ownership

Record the task, owner, allowed paths, dependencies, handoff artifact, and verification responsibility before concurrent edits. Workers stay inside their allowlists and escalate shared-file conflicts. The integrator checks every worker range for scope, merge commits, evidence, and commit policy before applying it. Do not stage or clean another worktree's changes.

<a id="freeze-gate"></a>
## Freeze gate

During a release freeze, accept only submission blockers and low-risk correctness fixes authorized for that release. Require a rollback path and prohibit broad refactors, bulk formatting, low-confidence promotion, unverified dependency/toolchain/platform changes, runner-output contract changes, and unrelated cross-subsystem edits.

<a id="review-separation"></a>
## Review separation

The implementer owns diagnosis, minimal edits, targeted evidence, risks, and handoff. A reviewer independently checks scope, fake-success risk, errno/ABI/user-pointer/lock/lifetime/cfg boundaries, regression adequacy, and claim-to-evidence fit. Use explicit `APPROVE`, `REQUEST_CHANGES`, or `COMMENT`; repair blocking findings and repeat affected checks before integration.

<a id="integration-gate"></a>
## Integration gate

Before integration, verify the target baseline, allowed range, clean ownership boundary, applicable build/runtime evidence, rollback plan, and unresolved risks. Experimental work advances only through its defined promotion gates. Handoff repository cleanup to `$oskernel-repo-hygiene` and validation selection to `$oskernel-validation` rather than duplicating those procedures here.
