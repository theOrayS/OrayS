<a id="evaluator-evidence-contract"></a>
# Evaluator evidence and reporting

Treat the evaluator's current parser and group output as the contract. Inspect the relevant runner/scorer implementation when interpretation depends on formatting or aggregation; do not repair a reporting mismatch by fabricating inner results or suppressing genuine failures.

<a id="result-layers"></a>
## Separate result layers

Report at least these layers independently:

1. invocation and environment: ref, arch, libc/runtime, case-selection mode, timeout, and raw-log path;
2. wrapper closure: started, pass, fail, timeout, skip, incomplete, and last closed case;
3. inner testcase results: `TPASS`, `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, panic, and trap totals;
4. evaluator aggregation: per-group scores before the total, with parser or artifact assumptions stated;
5. evidence boundary: local-only, remote-submission build, or official remote execution.

Do not explain a low total from a healthy core group until comparing the per-group rows. Do not infer remote behavior from local RV/LA alone.

<a id="sweep-invocation-provenance"></a>
## Sweep invocation provenance

For a full-sweep result, preserve the exact `LTP_CASES` spelling and distinguish blacklist aliases
(`blacklist`, `all-minus-blacklist`, `sweep:blacklist`) from unfiltered `all`. Report whether
`LTP_SWEEP_DEFAULT_BLACKLIST_CASES`, build-time `LTP_BLACKLIST`, guest `/ltp_blacklist.txt`, guest
`/tmp/ltp_blacklist.txt`, or any live architecture-specific source contributed. Record the effective per-case timeout
and whether it came from `/ltp_case_timeout_secs`, build-time `LTP_CASE_TIMEOUT_SECS`, or the compiled default.
Never collapse these inputs into a generic “sweep” label because they change what ran and what a score can mean.

<a id="compatibility-report-fields"></a>
## Compatibility report fields

Include current gap, exact sources and commands, changed or proposed semantic boundary, targeted and adjacent regression results, architecture/libc matrix, raw and parsed evidence paths, unresolved failure classes, and promotion/blacklist status. Explicitly state any syscall, errno, flag, struct layout, FD, signal, futex, mmap, userspace-copy, ABI, kernel-runtime, build, or evaluator behavior impact.

<a id="compatibility-campaign-artifacts"></a>
## Campaign artifacts

Store a new compatibility campaign under `docs/ltp-compatibility-YYYY-MM-DD-phase-x/` using the local creation date and the next phase letter for that date. Keep historical artifact paths unchanged and link them from the new report. When moving a prompt or plan, update its self-reference, OMX brief path, and follow-up text together. The neutral compatibility name replaces score-improvement naming for new work without rewriting history.

<a id="reporting-handoffs"></a>
## Handoffs

- Use `$oskernel-cross-arch-delivery` for local/remote address maps, platform configs, offline dependencies, or submission artifacts.
- Use `$oskernel-validation` to choose generic build, lint, QEMU, or bounded-regression evidence.
- Use `$oskernel-collaboration-delivery` for durable reports, review gates, staging, and Lore commits.
