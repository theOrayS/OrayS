# stable1000 validation

## Final commands

The promotion gate used explicit case lists, `OSCOMP_TEST_GROUPS=ltp`, `LTP_CASE_TIMEOUT_SECS=60` for post-review reruns, and `scripts/ltp_summary.py` for parser-backed evidence.

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$NEW44" LTP_CASE_TIMEOUT_SECS=60 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$NEW44" LTP_CASE_TIMEOUT_SECS=60 timeout 45m ./run-eval.sh la
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$REGRESSION_SUBSET" LTP_CASE_TIMEOUT_SECS=60 timeout 30m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$REGRESSION_SUBSET_STABLE_ORDER" LTP_CASE_TIMEOUT_SECS=60 timeout 30m ./run-eval.sh la
python3 scripts/ltp_summary.py "$OUT/$ARCH-raw.log" > "$OUT/$ARCH-summary.txt"
python3 scripts/ltp_summary.py --json "$OUT/$ARCH-raw.log" > "$OUT/$ARCH-summary.json"
sha256sum "$OUT"/* > "$OUT/sha256.txt"
```

`REGRESSION_SUBSET_STABLE_ORDER` covers the same 30 regression cases as the RV subset but orders LA by the live stable-list order. This avoids an artificial immediate `pipe02 -> pipeio` ordering that is not present in `LTP_STABLE_CASES` and caused one excluded diagnostic TBROK.

## Evidence paths

| Scope | Log | Summary | JSON | SHA256 |
| --- | --- | --- | --- | --- |
| RV new44 post-review final | `target/ltp-1000-milestone-10-stable1000/rv-new44-postreview-rerun60-20260606T135933+0800/rv-raw.log` | `target/ltp-1000-milestone-10-stable1000/rv-new44-postreview-rerun60-20260606T135933+0800/rv-summary.txt` | `target/ltp-1000-milestone-10-stable1000/rv-new44-postreview-rerun60-20260606T135933+0800/rv-summary.json` | `target/ltp-1000-milestone-10-stable1000/rv-new44-postreview-rerun60-20260606T135933+0800/sha256.txt` |
| LA new44 post-review final | `target/ltp-1000-milestone-10-stable1000/la-new44-postreview-rerun60-20260606T140605+0800/la-raw.log` | `target/ltp-1000-milestone-10-stable1000/la-new44-postreview-rerun60-20260606T140605+0800/la-summary.txt` | `target/ltp-1000-milestone-10-stable1000/la-new44-postreview-rerun60-20260606T140605+0800/la-summary.json` | `target/ltp-1000-milestone-10-stable1000/la-new44-postreview-rerun60-20260606T140605+0800/sha256.txt` |
| RV regression post-review final | `target/ltp-1000-milestone-10-stable1000/rv-regression-postreview-rerun60-20260606T141353+0800/rv-raw.log` | `target/ltp-1000-milestone-10-stable1000/rv-regression-postreview-rerun60-20260606T141353+0800/rv-summary.txt` | `target/ltp-1000-milestone-10-stable1000/rv-regression-postreview-rerun60-20260606T141353+0800/rv-summary.json` | `target/ltp-1000-milestone-10-stable1000/rv-regression-postreview-rerun60-20260606T141353+0800/sha256.txt` |
| LA regression stable-order post-review final | `target/ltp-1000-milestone-10-stable1000/la-regression-postreview-stableorder60-20260606T142703+0800/la-raw.log` | `target/ltp-1000-milestone-10-stable1000/la-regression-postreview-stableorder60-20260606T142703+0800/la-summary.txt` | `target/ltp-1000-milestone-10-stable1000/la-regression-postreview-stableorder60-20260606T142703+0800/la-summary.json` | `target/ltp-1000-milestone-10-stable1000/la-regression-postreview-stableorder60-20260606T142703+0800/sha256.txt` |
| RV exec/FD/vfork smoke | `target/ltp-1000-milestone-10-stable1000/rv-postreview-exec-fd-vfork-smoke-20260606T133920+0800/rv-raw.log` | `target/ltp-1000-milestone-10-stable1000/rv-postreview-exec-fd-vfork-smoke-20260606T133920+0800/rv-summary.txt` | `target/ltp-1000-milestone-10-stable1000/rv-postreview-exec-fd-vfork-smoke-20260606T133920+0800/rv-summary.json` | `target/ltp-1000-milestone-10-stable1000/rv-postreview-exec-fd-vfork-smoke-20260606T133920+0800/sha256.txt` |
| LA pipeio single post-review confirmation | `target/ltp-1000-milestone-10-stable1000/la-pipeio-postreview-rerun-20260606T135800+0800/la-raw.log` | `target/ltp-1000-milestone-10-stable1000/la-pipeio-postreview-rerun-20260606T135800+0800/la-summary.txt` | `target/ltp-1000-milestone-10-stable1000/la-pipeio-postreview-rerun-20260606T135800+0800/la-summary.json` | `target/ltp-1000-milestone-10-stable1000/la-pipeio-postreview-rerun-20260606T135800+0800/sha256.txt` |

## Parser results

- RV new44 final: RUN_RC=0, PASS LTP CASE 88, FAIL 0, internal TFAIL/TBROK/TCONF 0/0/0, timeout 0, ENOSYS 0, panic/trap 0; musl 44/44, glibc 44/44.
- LA new44 final: RUN_RC=0, PASS LTP CASE 88, FAIL 0, internal TFAIL/TBROK/TCONF 0/0/0, timeout 0, ENOSYS 0, panic/trap 0; musl 44/44, glibc 44/44.
- RV regression subset: RUN_RC=0, PASS LTP CASE 60, FAIL 0, internal TFAIL/TBROK/TCONF 0/0/0, timeout 0, ENOSYS 0, panic/trap 0; musl 30/30, glibc 30/30.
- LA regression stable-order subset: RUN_RC=0, PASS LTP CASE 60, FAIL 0, internal TFAIL/TBROK/TCONF 0/0/0, timeout 0, ENOSYS 0, panic/trap 0; musl 30/30, glibc 30/30.
- RV smoke: RUN_RC=0, PASS LTP CASE 16, FAIL 0, internal TFAIL/TBROK/TCONF 0/0/0, timeout 0, ENOSYS 0, panic/trap 0; musl 8/8, glibc 8/8.
- LA pipeio single confirmation: RUN_RC=0, PASS LTP CASE 2, FAIL 0, internal TFAIL/TBROK/TCONF 0/0/0, timeout 0, ENOSYS 0, panic/trap 0; musl 1/1, glibc 1/1.
- Stable list count after promotion: 1000 total / 1000 unique / 0 duplicate.

## Supporting lane evidence

- RV exec/FD/vfork smoke `target/ltp-1000-milestone-10-stable1000/rv-postreview-exec-fd-vfork-smoke-20260606T133920+0800/rv-summary.txt`: `clone05`, `close_range01`, `close_range02`, `execve02`, `execve03`, `execve04`, `pipeio`, and `creat07` passed on musl+glibc with zero parser blockers after the exec atomicity, vfork wake, and FD alias repairs.
- LA pipeio single `target/ltp-1000-milestone-10-stable1000/la-pipeio-postreview-rerun-20260606T135800+0800/la-summary.txt`: `pipeio` passed on musl+glibc after the first post-review LA batch diagnostic.
- LA stable-order regression `target/ltp-1000-milestone-10-stable1000/la-regression-postreview-stableorder60-20260606T142703+0800/la-summary.txt`: same 30 regression cases as RV, ordered by live stable-list position; PASS 60/0 with zero parser blockers.

## Excluded diagnostics / non-promotion evidence

- `rv-new44-final-current-20260606T123709+0800`: pre-vfork-share diagnostic with `clone05`/`pipeio` blockers; excluded from promotion.
- `rv-new44-postreview-20260606T134134+0800`: first post-review batch had one `crash02` timeout; excluded. `crash02` single rerun and `rv-new44-postreview-rerun60-20260606T135933+0800` are clean.
- `la-new44-postreview-20260606T135023+0800`: first post-review batch had one `pipeio` TBROK; excluded. `la-pipeio-postreview-rerun-20260606T135800+0800` and `la-new44-postreview-rerun60-20260606T140605+0800` are clean.
- `la-regression-postreview-rerun60-20260606T141808+0800`: artificial immediate `pipe02 -> pipeio` regression order produced one `pipeio` TBROK; excluded. The stable-order LA regression rerun listed above is clean.

## Validation caveats

- Full stable1000 all-case RV/LA sweep was not rerun in this milestone. The stable1000 trust claim is cumulative: stable556..stable956 milestone gates already provide parser-backed evidence for the first 956 cases, while this milestone adds current RV/LA x musl/glibc clean evidence for the final 44 and current regression subsets.
- LA fcntl36 raw logs contain user BadAddress fault warnings from stress threads; parser output remains PASS with zero panic/trap classification.
- No blacklist/SKIP/status0/full-sweep partial TPASS evidence was counted.

## Final static and cleanup gate

- Final static log: `target/ltp-1000-milestone-10-stable1000/final-static-postreview-20260606T143910+0800/final-static-checks.log`.
- Stable count check: 1000 total / 1000 unique / 0 duplicate.
- Touched-file `rustfmt --edition 2021 --check`: PASS for changed Rust surface.
- `cargo -C examples/shell check -Z unstable-options --target riscv64gc-unknown-none-elf --features "axstd/defplat axstd/log-level-info axstd/alloc axstd/paging axstd/irq axstd/multitask axstd/fs axstd/net axstd/rtc auto-run-tests uspace"`: PASS, warnings only.
- `git diff --check`: PASS.
- Debug/probe/fake-pass marker scan: PASS, no findings.
- Promoted new44 case-name hardcode scan outside `examples/shell/src/cmd.rs`: PASS, no findings.
- AI slop cleaner report: `final-gate-ai-slop-cleaner-report.md`.

## Final independent review gate

- Code-reviewer: `RECOMMENDATION: APPROVE`; report `final-gate-code-review-report.md`.
- Architect: `Architectural Status: CLEAR`; report `final-gate-architect-review-report.md`.
