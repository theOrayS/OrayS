# Official evidence infrastructure stabilization

## Status

- State: `READY_FOR_SEMANTIC_FIX`
- Branch: `stabilize/post-integration-gates-20260716`
- Authoritative base: `origin/integration/four-prs-20260715`
- Base commit and initial HEAD: `09f4076ac151e0e7800103de724d9042230738b5`
- Durable contracts: root `AGENTS.md`, `.codex/tasks/SESSION_GUIDANCE.md`, and
  `.codex/tasks/GOAL_A_EVIDENCE_INFRA.md`

## Goal

Stabilize the official-evidence producer, parser, and runner contract so that
ordered cases have stable identities, execution accounting is exact, genuine
semantic failures remain `FAIL`, and malformed or incomplete evidence remains
fail-closed. Produce fresh, clean-tree RISC-V64 and LoongArch64 official
evidence without changing the authoritative external test plan or either
official image.

The only successful terminal state is `READY_FOR_SEMANTIC_FIX`. If an external
artifact or plan defect is proven to be the only correct resolution, terminate
as `BLOCKED_EXTERNAL`. An unacceptable regression terminates as `FAILED`.

## Non-goals

- Do not fix kernel or userspace semantic failures reported by official tests.
- Do not begin Goal B or optimize the remote score.
- Do not change `main`, rewrite ancestry, rebase, squash, or force-push.
- Do not weaken parsers, convert failures to passes, add skips, or special-case
  a command, testcase, path, architecture, libc, or evaluator environment.
- Do not modify official images or the authoritative external command source.
- Do not update dependencies, the Rust toolchain, or unrelated code.

## Baseline and initial evidence

- Local base, merge-base, and remote `origin/integration/four-prs-20260715`
  all resolve to `09f4076ac151e0e7800103de724d9042230738b5`.
- The stabilization branch did not exist remotely at task start.
- Initial worktree was clean.
- RISC-V64 image SHA-256:
  `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99`.
- LoongArch64 image SHA-256:
  `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50`.
- Focused parser baseline: 106 tests passed.
- Immutable prior full-run captures each contain 24 completed official groups
  and a zero evaluator exit code. Revalidation reports two infrastructure
  findings per architecture, all `busybox-duplicate-case`, while retaining
  119 RISC-V64 and 161 LoongArch64 semantic findings.
- The trusted BusyBox plan has 55 ordered rows and 54 distinct command strings.
  The repeated command occurs at two different ordered positions and mutates
  shared state, so textual uniqueness is not a valid case identity.

## Design direction

1. Give each executed BusyBox row a stable, one-based execution ordinal in the
   producer protocol. Preserve the command text as evidence, not identity.
2. Represent the trusted plan as structured ordered cases. Validate ordinals,
   command text, and any explicit IDs independently; duplicate explicit IDs are
   invalid even when ordinals differ.
3. Parse structured results with an anchored, unambiguous protocol. Require
   exact planned order, exact identity, and exact completion. Duplicate frame
   replay, missing/extra cases, malformed identity, unknown groups, or order
   drift remain infrastructure errors.
4. Preserve the existing result lattice: structural/integrity defects are
   `ERROR`; completed executions with test failures are `FAIL`; only a complete
   failure-free run is `PASS`.
5. Treat legacy text-only captures as immutable diagnostic input. They may
   demonstrate the ambiguity but must not silently satisfy the new canonical
   identity contract.

## Phases and checkpoints

### Phase 0 — provenance and read-only replay

- [x] Confirm clean branch, base, merge-base, remote freshness, and ancestry.
- [x] Hash both official images.
- [x] Read repository, task, runner, parser, manifest, CI, and historical
  integration contracts.
- [x] Run the focused parser baseline.
- [x] Replay both immutable official capture pairs and classify the defect.

### Phase 1 — protocol and plan contract

- [x] Add structured ordered BusyBox identities to the trusted plan.
- [x] Emit stable row identity from the shell producer without changing command
  execution semantics.
- [x] Update parser and failure reporting for the structured protocol.
- [x] Keep legacy ambiguous records fail-closed.

### Phase 2 — regression coverage

- [x] Accept repeated command text at distinct ordinals.
- [x] Reject duplicate explicit IDs.
- [x] Reject duplicate frame replay, missing/extra/order drift, malformed
  identities, unknown groups, and incomplete accounting.
- [x] Prove identity rules are architecture- and libc-independent.
- [x] Prove semantic `FAIL` is not reclassified as infrastructure `ERROR` once
  identity evidence is valid.
- [x] Update manifest counts and integrity inventories exactly.

### Phase 3 — focused and clean validation

- [x] Run focused parser/runner/reporter/static-guard tests.
- [x] Run mutation-style negative cases.
- [x] Run `git diff --check` and inspect the complete diff.
- [x] Commit reviewable protocol/test/documentation stages normally.
- [x] From a clean candidate commit, run canonical `quick` and `baseline`.

### Phase 4 — fresh two-architecture evidence

- [x] Recheck image hashes before execution.
- [x] Run fresh canonical RISC-V64 official evaluation.
- [x] Run fresh canonical LoongArch64 official evaluation.
- [x] Require `planned == executed == completed` and zero infrastructure errors
  for both architectures.
- [x] Preserve real semantic failures as `FAIL`; do not require semantic pass.
- [x] Recheck image hashes and prove overlays/temporary writable image state are
  absent after each run.

### Phase 5 — independent review and terminal state

- [x] Obtain an independent read-only review of the final diff and evidence.
- [x] Resolve all blocker/major findings and rerun affected validation.
- [x] Complete the development log, AI disclosure, risks, and rollback record.
- [x] Push only `stabilize/post-integration-gates-20260716` normally.
- [x] Declare exactly one Goal A terminal state and stop before Goal B.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| A delimiter in command text makes the record ambiguous | Use a numeric identity in a fixed anchored prefix and treat the remaining text as payload. Add malformed-record tests. |
| Compatibility parsing hides replay or omission | Legacy text-only results remain non-canonical and fail-closed; fresh producer output is required for terminal evidence. |
| A semantic failure is mislabeled as infrastructure | Keep structural findings and test findings in separate collections and assert runner mapping. |
| Plan drift is mistaken for a parser fix | Keep the external row order and text unchanged; migrate only the tracked representation and verify source metadata/hash. |
| Architecture/libc-specific logic enters generic code | Use one identity implementation and exercise all canonical labels in table-driven tests. |
| Official images are mutated | Verify fixed SHA-256 values before and after runs and require ephemeral overlay cleanup. |
| Final documentation commit differs from the evidence commit | Record both commits explicitly, keep the closure commit documentation-only, and rerun clean quick/baseline on the final branch head. |

## Verification contract

Required before `READY_FOR_SEMANTIC_FIX`:

- focused producer/parser/runner/reporter/static integrity tests pass;
- `git diff --check` and final scope review pass;
- canonical clean-tree `quick` and `baseline` pass;
- fresh RV and LA official runs complete every planned group/case with zero
  infrastructure errors and only `FAIL` or genuine `PASS` terminal semantics;
- official image hashes are unchanged and no writable overlays remain;
- independent review reports zero blocker/major findings;
- the stabilization branch is pushed normally with stable ancestry and complete
  provenance.

## Rollback

Revert the stabilization commits in reverse order. The authoritative base and
official images remain unchanged, and no history rewrite is needed.
