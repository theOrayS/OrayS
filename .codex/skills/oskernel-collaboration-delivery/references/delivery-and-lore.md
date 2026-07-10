<a id="delivery-loop"></a>
# Delivery and Lore

Use the minimal loop: establish facts, choose a bounded target, edit, run targeted verification, run adjacent regressions when semantics changed, summarize evidence, obtain review, stage owned paths, inspect the cached diff, and commit.

<a id="delivery-report"></a>
## Delivery report

Report:

- changed files and intent;
- user-visible behavior and kernel-runtime/build/evaluator impact;
- syscall, errno, flag, struct layout, FD, signal, futex, mmap, userspace-copy, and ABI impact;
- each verification command with PASS/FAIL and a concise output reference;
- unrun checks with reasons;
- risk, rollback, ownership handoffs, and residual uncertainty.

Use neutral intent/subsystem titles. Do not optimize PR or commit wording for scoring, reviewers, agents, or evaluator presentation.

<a id="selective-staging"></a>
## Selective staging

Start from the saved baseline and current porcelain status. Stage only allowlisted owned paths, then inspect `git diff --cached --check`, `git diff --cached --name-status --no-renames`, the cached patch, and remaining status. If unrelated changes cannot be separated safely, report a blocker instead of committing them.

<a id="lore-commit"></a>
## Lore commit

The first line records why the decision was made, not a file inventory. Add only trailers that preserve useful decision context:

```text
<intent line>

Constraint: <external constraint>
Rejected: <alternative> | <reason>
Confidence: <low|medium|high>
Scope-risk: <narrow|moderate|broad>
Directive: <warning for future modifiers>
Tested: <fresh verification>
Not-tested: <known gap>
```

`Rejected:` prevents repeated dead-end exploration; `Directive:` records a forward safety warning. Never claim a check in `Tested:` unless its fresh output was read. Use `Not-tested:` for relevant gaps rather than hiding them.

<a id="commit-stop"></a>
## Commit stop gate

Commit durable source, documentation, or project-state changes after relevant verification when they are safely separable and the task expects delivery. Stop without committing when required checks still fail, ownership is ambiguous, or the requested action would include unrelated work.
