<a id="quality-first-ltp-selection"></a>
# Quality-first LTP selection

LTP is a specialized Linux/POSIX compatibility signal. Select work that improves general semantics and regression confidence; do not let contest score or raw case count override correctness, security, maintainability, or current release risk.

<a id="selection-facts"></a>
## Establish current facts

Before ranking candidates, inspect only current artifacts:

- the latest evaluator group table, raw log, and `scripts/ltp_summary.py` result;
- the live `examples/shell/src/cmd.rs::LTP_STABLE_CASES` contents;
- the applicable LTP runtest entry and testcase source from the contest baseline;
- internal fan-out such as `tcases[]`, variants, loops, forks, and expected pass/fail assertions;
- the active `LTP_CASES` mode, timeout, and every default, environment, or file blacklist source.

Do not infer current coverage from memory, old reports, wrapper-only output, or a local smoke log.

<a id="contest-ltp-source-baseline"></a>
## Contest LTP source baseline

For contest testcase and runtest evidence, retain `oscomp/testsuits-for-oskernel@pre-2025` and its
`ltp-full-20240524` tree as the comparison baseline; upstream LTP master is auxiliary evidence only. Before citing
that baseline as applicable to a run, inspect the available testsuite checkout, ref, source tree, executable set, and
runner wiring live. The retained identifiers preserve evaluator provenance, not an assertion that a checkout or ref
has remained unchanged.

<a id="candidate-ranking"></a>
## Rank candidates

Prefer a small target with reusable Linux/POSIX semantics, current failure evidence, adjacent regression value, and bounded implementation risk. Typical high-value surfaces include foundational VFS/FD/process/signal/user-memory behavior already exercised by nearby tests. Defer broad facilities whose prerequisite model is absent unless the task explicitly funds that architecture work.

For an experimental full sweep, treat high internal assertion density as a scouting signal only. Partial `TPASS` output does not close a case that also reports `TFAIL`, `TBROK`, timeout, or an unclosed run.

<a id="selection-execution"></a>
## Execute the selection lane

1. Record the current gap and evidence paths.
2. Run the smallest targeted case or batch first.
3. Diagnose syscall, errno, flag, ABI, lifetime, or resource boundaries from source and logs.
4. After a real fix, run adjacent regressions before considering stable promotion.
5. State RV/LA and glibc/musl coverage honestly; hand architecture-delivery gaps to `$oskernel-cross-arch-delivery`.

<a id="selection-report"></a>
## Selection report

Include the case and pass/all gap, subsystem, source/runtest/log paths, internal fan-out, likely semantic boundary, estimated cost and regression risk, targeted commands, adjacent regressions, and the final architecture/libc gate. For sweeps, also include mode, blacklist sources, started/closed/incomplete counts, last closed case, failure-class totals, and removal candidates for any temporary blacklist entry.
