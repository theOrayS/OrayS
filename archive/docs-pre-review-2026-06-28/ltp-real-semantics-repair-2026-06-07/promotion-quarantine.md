# Promotion quarantine rules (G001 baseline)

Date: 2026-06-07
Scope: evidence policy for `G001-g001-phase-0-quarantine`; leader owns final promotion and Ultragoal checkpointing.

## Non-negotiable rule

Stable promotion requires real Linux/POSIX behavior evidence across the required matrix:

- RV + LA
- musl + glibc
- parser-clean `scripts/ltp_summary.py` summaries
- no hidden TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap caveats

Runner markers, wrapper status, smoke probes, blacklist modes, and synthetic feature probes are audit hints unless backed by parser-clean behavior evidence.

## Evidence that must not promote a case

Do **not** promote from any of the following:

1. `blacklist`, `score-blacklist`, `all-minus-blacklist`, `sweep:blacklist`, `stable-plus-blacklist`, or `stable-plus-all-minus-blacklist` runs.
2. Explore/scouting modes or full-sweep partial TPASS results.
3. `status0-only` results that lack parser-clean case evidence.
4. Logs where a nonzero numeric status is hidden by a `PASS` token.
5. Logs where timeout/panic/trap appears after or around a status0 marker.
6. Any relevant TCONF/TBROK/TFAIL/ENOSYS in the parser summary.
7. Synthetic probe-only success, including `/proc`, `/dev`, `/etc`, config, userdb, or metadata existence without behavior proof.
8. Case-name/path/process-name hardcoding, including runner branches that change a named LTP case environment to make it pass.
9. Runtime musl byte-patch evidence without a patch manifest and raw syscall + musl + glibc cross-checks.
10. Raw create/delete smoke when the API under test requires setter/getter or signal/side-effect behavior.

## Parser truth requirements

For any candidate promotion report:

- Use `scripts/ltp_summary.py` output as the result source of truth.
- Numeric case status decides wrapper PASS/FAIL classification.
- The report must list TFAIL/TBROK/TCONF, timeouts, ENOSYS, panic/trap, and prior failure events.
- `--promotion-candidates` must require all configured arch/libc combinations; missing RV/LA or musl/glibc entries block promotion.
- Marker text inside test output must not be treated as a clean result for the active case.

Minimum acceptable report fields:

- selection mode;
- stable list total/unique/duplicate baseline;
- blacklist source and blacklist count, if any;
- skipped count;
- guest override or manifest source (`/ltp_cases.txt`, `/tmp/ltp_cases.txt`, env, or stable default);
- architecture/libc matrix;
- complete caveat list: TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap;
- raw log paths and parser summary paths.

## Quarantine domains

A case must stay quarantined when it depends on any of these unresolved domains:

| Domain | Quarantine trigger | Release condition |
| --- | --- | --- |
| POSIX/libc fake success | TODO/unimplemented path returns `0` or `Ok(0)`. | Returns real errno or implements observable behavior; regression proves it. |
| stat/lstat/metadata | Metadata is default/hard-coded where Linux semantics matter. | Correct metadata fields with targeted LTP and smoke proof. |
| fd/fcntl/flags | Unsupported command succeeds, flags ignored, or CLOEXEC/nonblock behavior not observable. | Set/get/fork/exec or relevant behavior proof. |
| rlimit/sysconf | Resource/config API accepts but does not store/enforce/report real values. | get/set/enforcement or Linux-compatible unsupported errno proof. |
| signals/timers | Handler/mask/delivery is discarded or timer is non-delivering. | Signal state/delivery regression or honest unsupported behavior. |
| socket/time/mempolicy weak success | Return value succeeds without follow-up behavior. | Getter/peer-address/signal/policy behavior proof. |
| synthetic proc/dev/config | Synthetic feature exists without capability source. | Capability map plus tests for each advertised feature. |
| runner selection/specialization | Case source is blacklist/explore/stable-plus or a named-case branch changes semantics. | Stable-only selection and general mechanism, no named-case behavior hack. |
| runtime libc patch | Loader mutates libc bytes without manifest/cross-check. | Patch manifest plus raw syscall, musl, and glibc evidence, or patch removed. |

## Stable-count handling

The live G001 baseline is:

```text
examples/shell/src/cmd.rs::LTP_STABLE_CASES
const_line_range=50-619
total=1000
unique=1000
duplicate_extra_entries=0
duplicate_names_with_counts=<none>
```

This count is not a promotion claim.  It is a baseline for quarantine and later truthfulness repairs.

If removing fake success or synthetic-only evidence lowers the stable count, document the decrease as a truthfulness repair rather than a regression.  Do not re-promote those cases until the release condition for the affected domain is met.

## Allowed evidence for release from quarantine

A case can leave quarantine only when all applicable conditions hold:

1. The underlying implementation no longer relies on fake success or case-specific hacks.
2. A targeted test or LTP case proves the actual behavior, not only process exit status.
3. RV and LA both pass.
4. musl and glibc both pass, unless the report explicitly scopes a libc-specific non-promotion investigation.
5. `scripts/ltp_summary.py` is parser-clean for the case: no TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap, no missing matrix entry, no prior failure event.
6. The report includes raw logs, parser summary, selection mode, skipped/blacklist counts, and caveats.
7. The leader accepts the evidence and updates the stable/promotion artifact; workers do not mutate `.omx/ultragoal` or close aggregate goals.

## G001 worker boundary

This document is policy/baseline only.  It does not run the evaluator, does not edit `LTP_STABLE_CASES`, does not change source code, and does not checkpoint `.omx/ultragoal`.
