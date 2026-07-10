<a id="stable-truth"></a>
# Stable promotion and blacklist truth

The live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` list is the only current stable-count source. Recompute it rather than quoting memory. Smoke logs, wrapper status, full-sweep discovery, and blacklist skips are not promotion proof.

<a id="promotion-gate"></a>
## Promotion gate

Promote a case only when all applicable evidence is present:

- targeted execution passes on the relevant libc and architecture combinations;
- adjacent high-value regressions show no material degradation;
- `scripts/ltp_summary.py` preserves timeout, `TCONF`, `ENOSYS`, panic, trap, and inner result counts;
- the report links raw logs or durable summaries and identifies untested combinations;
- no testcase-name/path/process specialization, fake output, testsuite modification, or evaluator bypass exists.

Local evidence may justify a candidate, but it cannot be described as remote parity without remote evidence. A case seen in a full sweep remains a candidate until the targeted promotion gate closes.

<a id="blacklist-boundary"></a>
## Blacklist boundary

A blacklist protects an experimental sweep from cases that hang, exhaust memory, fork uncontrollably, crash shared state, run destructive stress, or require a kernel facility outside the current model. It is not a pass mechanism or a way to improve presentation.

Do not blacklist an ordinary `TFAIL`, wrong errno, `ENOSYS`, or `TBROK` merely because it lowers a pass rate. Record every entry's source, reason category, first failure evidence, and removal condition. Remove it after a real fix and targeted validation.

<a id="sweep-selection-contract"></a>
## Sweep selection and timeout contract

Inspect the live shell runner and build wiring before every sweep; the retained baseline contract is:

- `LTP_CASES=blacklist`, `LTP_CASES=all-minus-blacklist`, and `LTP_CASES=sweep:blacklist` are equivalent entry
  points that enumerate the guest LTP executable directory and subtract the merged blacklist;
- `LTP_CASES=all` enumerates without that subtraction and is only for carefully bounded diagnosis because a case may
  hang, exhaust resources, or damage later evaluator state;
- the common blacklist inputs are source default `LTP_SWEEP_DEFAULT_BLACKLIST_CASES`, build-time `LTP_BLACKLIST`,
  guest `/ltp_blacklist.txt`, and guest `/tmp/ltp_blacklist.txt`;
- the per-case timeout is selected from guest `/ltp_case_timeout_secs` or build-time `LTP_CASE_TIMEOUT_SECS` before
  the compiled default.

Current code may add architecture-specific inputs or modes. Record every effective source, exact mode spelling,
effective timeout and origin from the live implementation; do not treat this retained list as permission to ignore
newer inputs or as promotion evidence.

<a id="sweep-closure"></a>
## Sweep closure

The report must distinguish RUN, PASS, FAIL, TIMEOUT, SKIP, and incomplete cases; list the selection mode and merged blacklist sources; identify the last closed and any unclosed case; and retain parser totals for `TPASS`, `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, panic, and trap. Never convert skipped or unclosed work into stable, promotion, or pass counts.

<a id="regression-families"></a>
## Regression families

When changing VFS, FD, process, signal, user-memory, mmap, or errno behavior, select adjacent regressions from already working access, stat/chmod, open, pipe, fork/wait, signal, read/write, and vectored-I/O families as applicable. The exact set follows the changed semantic boundary rather than a fixed score list.
