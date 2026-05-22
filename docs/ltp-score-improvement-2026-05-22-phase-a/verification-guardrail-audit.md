# Verification Guardrail Audit

Scope: `scripts/ltp_summary.py`, `examples/shell/src/cmd.rs`, plus quick output scans in `output_la.md` / `output_rv.md`.

## Findings

- **Numeric status is the source of truth; fake PASS token is intentionally preserved for compatibility.**
  - `scripts/ltp_summary.py:4-8` — docstring says wrapper logs may print `FAIL LTP CASE <case> : 0` for a successful case and that the numeric status is the source of truth.
  - `scripts/ltp_summary.py:108-118` — `normalize_wrapper_status()` returns PASS only when `code == 0`, otherwise FAIL.
  - `examples/shell/src/cmd.rs:1475-1489` — success prints `FAIL LTP CASE {case} : 0` and timeout prints both `FAIL ... : 137/143` and `TIMEOUT LTP CASE ...`.
  - **Risk:** downstream consumers that key off the literal `FAIL` token instead of the numeric status can misread healthy cases as failures.

- **Case-name hardcode guardrails are present.**
  - `examples/shell/src/cmd.rs:44-49` — core and stable LTP case sets are explicit constants, not ad hoc string literals at the call site.
  - `examples/shell/src/cmd.rs:533-549` — `valid_ltp_case_name()` rejects invalid names and `push_ltp_case()` dedupes accepted cases.
  - `examples/shell/src/cmd.rs:588-637` — selection supports `stable`, `core`, `batch:<name>`, `file:<path>`, and inline lists; no single-case hardcode path found.
  - **Risk:** the shipped default sets are still static lists, so coverage changes require source edits by design.

- **Silent SKIP was not found in the LTP runner path; the only visible skip is explicit.**
  - `examples/shell/src/cmd.rs:1570-1572` — disabled official groups emit `autorun: skip disabled test group ...`.
  - **Risk:** this is an explicit skip message, not a silent one; I did not find an LTP-specific silent skip path in this pass.

- **Timeout is not treated as PASS.**
  - `scripts/ltp_summary.py:25, 31-33, 165-170, 204-218, 267-301, 330-346, 492-567` — timeout markers are tracked separately and surfaced in categories / tables.
  - `examples/shell/src/cmd.rs:1487-1501` — timeout cases print a failure status plus `TIMEOUT LTP CASE` and increment timeout counters.
  - **Risk:** if a consumer ignores timeout fields and reads only top-level pass/fail counts, it can underreport failures.

- **Promotion-candidate filtering is conservative and blocks on hidden issues.**
  - `scripts/ltp_summary.py:267-303` — categories separate clean passes from `pass_with_tconf`, wrapper fail, `TFAIL`, `TBROK`, timeout, `ENOSYS`, panic/trap, and unknown.
  - `scripts/ltp_summary.py:330-347` — row-level blockers include `TFAIL`, `TBROK`, `TCONF`, timeout, `ENOSYS`, panic/trap, and non-PASS status.
  - `scripts/ltp_summary.py:350-419` — a case only becomes a candidate when every required arch/libc combo is present and no blockers remain.
  - **Risk:** this is intentionally strict; it will exclude cases with caveats even if the wrapper count looks green.

## Output scan notes

- `output_la.md:632-644` — repeated futex warnings: `The futex facility returned an unexpected error code.`
- `output_la.md:6004-6159` — iperf split is clear: `iperf-musl` succeeds (`... end: success`), while `iperf-glibc` fails with `control socket has closed unexpectedly` and repeated `Connection refused`.
- `output_rv.md:975-982, 5956-5987, 6004-6159` — LTP section shows `oom_score_adj does not exist, skipping the adjustment`; libcbench also has futex warnings; iperf-musl succeeds and iperf-glibc fails with `ECONNREFUSED` / loopback connect errors.
- `output_rv.md:6162-6165` — `lmbench-musl` timed out; `lmbench-glibc` continued afterward.
- **Quick scan result:** I did not find obvious `panic`, `crash01`, `free_frames=0`, or `memory-pressure` markers in the visible RV scan; the most obvious non-LTP signals here were futex warnings, iperf failures, and the `oom_score_adj` TINFO lines.

## Conclusion

The two main LTP guardrails are already in place: numeric-status truth for wrapper classification and explicit separation of timeout / internal-error / panic markers from clean PASS. The main residual risk is downstream misinterpretation of the legacy `FAIL ... : 0` compatibility line or top-level counts without consulting the detailed markers.
