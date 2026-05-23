# Final code review report: stable250 + remote score-zero fix

Created: 2026-05-23T17:56:47Z

Verdict: **APPROVE**. I found no blocking issues in the delivered stable250 work or the corrected remote score-zero ANSI marker fix.

## Reviewed scope

- `examples/shell/src/cmd.rs`: `LTP_STABLE_CASES` final stable250 list.
- `examples/shell/src/uspace/credentials.rs`: real/effective/saved/fs uid/gid and group syscall semantics.
- `examples/shell/src/uspace/fd_table.rs`: fsuid/fsgid-aware open permission checks and creation ownership.
- `examples/shell/src/uspace/metadata.rs`: `faccessat` real/effective-id behavior.
- `examples/shell/src/uspace/synthetic_fs.rs`: `/proc/*/status` uid/gid/fs-id reporting.
- `kernel/diagnostics/axlog/src/lib.rs`: ANSI-colored log newline framing for remote LTP marker parsers.
- Durable reports and summaries under this directory.

## Findings

| Severity | Finding | Status |
| --- | --- | --- |
| Blocking | None. | Closed |
| Non-blocking risk | External remote evaluator scoring still needs user-side confirmation after submitting these commits. Local logs now prove marker lines are unprefixed, but the remote scorer itself is not available here. | Disclosed |
| Non-blocking future hardening | File permission helpers now handle real/effective/fs uid/gid for the promoted gates. Supplementary-group matching should remain a future hardening area if broader LTP permission cases are promoted. | Disclosed |

## Important code anchors

- `kernel/diagnostics/axlog/src/lib.rs:132-142`: keeps newline outside ANSI-colored fragment so result markers start at column 1.
- `examples/shell/src/uspace/credentials.rs:28-45`: fs uid/gid getters.
- `examples/shell/src/uspace/credentials.rs:48-103`: real/effective/saved id setters synchronize fs ids where Linux semantics require it.
- `examples/shell/src/uspace/credentials.rs:124-191`: privilege/current-id/fs-id helpers and old-id return semantics for fsuid/fsgid calls.
- `examples/shell/src/uspace/credentials.rs:247-259`: `setuid` permission model rejects unauthorized id changes with EPERM.
- `examples/shell/src/uspace/fd_table.rs:1444-1467`: open permission uses recorded metadata plus fs uid/gid.
- `examples/shell/src/uspace/metadata.rs:173-230`: `faccessat` validates flags and chooses real vs effective ids according to `AT_EACCESS`.
- `examples/shell/src/uspace/synthetic_fs.rs:177-198`: `/proc/*/status` reports uid/gid triplets plus fs ids.

## Validation evidence

| Command | Result |
| --- | --- |
| `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv` | PASS: 500 wrapper PASS, 0 FAIL; ltp-musl 250/0; ltp-glibc 250/0; known `read02` TCONF only |
| `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la` | PASS: 500 wrapper PASS, 0 FAIL; ltp-musl 250/0; ltp-glibc 250/0; known `read02` TCONF only |
| `python3 -B scripts/ltp_summary.py ...stable250-post-ansi-rv.log` | PASS summary generated |
| `python3 -B scripts/ltp_summary.py ...stable250-post-ansi-la.log` | PASS summary generated |
| marker-line check on RV/LA logs | PASS: each log has 500 marker lines and 0 bad marker prefixes |
| `python3 -B scripts/test_ltp_summary.py` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `make A=examples/shell ARCH=riscv64` | PASS |
| `make all` | PASS; remote-submission `kernel-rv` and `kernel-la` built |
| `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all` | PASS; offline remote-submission build |
| `git diff --check 8ea57c4f..HEAD` | PASS |

## Conclusion

The stable250 target is delivered and the previous failed remote score-zero fix is superseded by the ANSI newline-framing fix. The only remaining verification is external: submit the committed outputs to the remote evaluator and confirm the scoreboard no longer reports 0 for LTP despite local PASS output.
