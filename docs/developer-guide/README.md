# Developer Guide for OSKernel 2026 ArceOS Branch

This guide is for developers maintaining the OSKernel 2026 evaluation branch of
ArceOS.  It explains where to make changes, which build/evaluation path proves a
change, and which shortcuts are not allowed in this contest-oriented tree.

Use this guide together with the repository root `README.md` and `AGENTS.md`:

- `README.md` is the quick entry point for building and running the repository.
- `AGENTS.md` is the operational contract for agents and maintainers.
- `docs/developer-guide/` is the developer-facing map for day-to-day code work.

## Recommended reading order

1. [`repository-map.md`](repository-map.md) — understand the major directories
   and the evaluator-specific ownership boundaries.
2. [`build-and-eval.md`](build-and-eval.md) — learn the local QEMU and remote
   submission build paths before touching evaluator code.
3. [`ltp-workflow.md`](ltp-workflow.md) — follow the honest LTP selection,
   targeted-run, parser, and promotion flow.
4. [`validation-matrix.md`](validation-matrix.md) — choose the smallest useful
   validation command for each type of change.

## Core development rules

- Keep local validation and remote submission behavior explicitly separated.
  Local RV/LA runs use `./run-eval.sh`; remote submission artifacts come from
  `make all`.
- Treat `examples/shell` as an evaluator integration surface, not just a demo.
  Changes there can affect official groups, LTP execution, wrapper markers, and
  remote scoring.
- Preserve honest Linux/POSIX semantics.  Do not hardcode testcase names, fake
  successful output, hide `TCONF`/timeouts, or modify testsuite sources to pass.
- Prefer targeted LTP batches and parser-backed summaries before full gates.
  A clean outer QEMU exit code is not enough evidence.
- Keep patches narrow.  Avoid broad refactors, generated-artifact edits, or
  unrelated formatting churn while fixing one subsystem.

## Current live evaluator facts

As of this document update, `examples/shell/src/cmd.rs::LTP_STABLE_CASES` has
383 unique cases.  The runner applies the selected list to both `/musl` and
`/glibc`, so the default stable list produces 766 LTP case executions per
architecture.

Re-check this from source when the number matters; do not rely on historical
campaign notes for current counts.
