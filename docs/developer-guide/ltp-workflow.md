# LTP Development Workflow

LTP work in this repository is contest-score oriented, but it must still preserve
honest Linux/POSIX semantics.  The goal is to promote real, regression-safe
coverage rather than to make individual test names appear green.

## Source of truth

LTP runner behavior lives mainly in `examples/shell/src/cmd.rs`:

- `LTP_CORE_CASES` — small smoke set.
- `LTP_STABLE_CASES` — current high-confidence stable set.
- `LTP_CASE_BATCHES` — named targeted batches.
- `selected_ltp_cases()` — runtime/build-time case selection.
- `run_ltp_suite()` — per-case execution, timeout handling, wrapper markers,
  cleanup, and summary output.

Current stable count must be read from `LTP_STABLE_CASES`, not from old reports.
At this document update it is 383 unique cases, executed for both `/musl` and
`/glibc`.

## Case selection controls

Inside the guest, either file overrides the selected cases:

```text
/ltp_cases.txt
/tmp/ltp_cases.txt
```

At build time, `LTP_CASES` can select:

```text
stable
core
batch:<name>
file:<path>
case1,case2,case3
```

Timeout can be overridden by `/ltp_case_timeout_secs` in the guest or by the
build-time `LTP_CASE_TIMEOUT_SECS` option.  The default per-case timeout is
currently 15 seconds.

## Required parser discipline

Always parse evaluator logs with `scripts/ltp_summary.py` before calling a run
healthy:

```bash
python3 scripts/ltp_summary.py output_rv.md
python3 scripts/ltp_summary.py output_la.md
python3 scripts/ltp_summary.py --promotion-candidates rv.log la.log
```

The parser counts wrapper result lines and internal quality signals:

- `TFAIL`
- `TBROK`
- `TCONF`
- `TIMEOUT LTP CASE` / timeout text
- `ENOSYS` / not implemented
- panic/trap signals

A clean outer `run-eval` or QEMU exit code is not enough.  Keep `TCONF`, timeout,
ENOSYS, and panic/trap caveats visible in reports.

## Wrapper marker format

The runner preserves the remote scorer wire format for completed LTP cases:

```text
FAIL LTP CASE <case> : <status>
```

Status `0` means wrapper-level pass; non-zero means failure.  This is why the
summary parser treats the numeric status as the source of truth.  Do not rewrite
marker format casually; remote scoring depends on it.

## Recommended fix loop

1. Pick candidate cases by contest value and subsystem relevance.
2. Inspect the contest testsuite `runtest` entry and matching LTP source.
3. Run a small targeted batch, not the full evaluator, while diagnosing.
4. Parse the log with `scripts/ltp_summary.py`.
5. Fix the real syscall/VFS/FD/process/signal/memory semantics.
6. Run adjacent regression cases from the same subsystem.
7. Validate both RV and LA when promoting a case.
8. Only then consider adding cases to `LTP_STABLE_CASES`.

## Red lines

Do not:

- hardcode behavior by testcase name, path, or process name;
- fake `TPASS` or wrapper PASS output;
- edit LTP testsuite sources to bypass failures;
- hide real failures as `SKIP`/`TCONF`;
- treat timeout as a pass;
- promote a case from one architecture/libc variant without naming the missing
  validation.
