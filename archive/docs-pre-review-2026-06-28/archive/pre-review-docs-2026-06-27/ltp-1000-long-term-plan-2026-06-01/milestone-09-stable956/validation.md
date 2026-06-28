# stable956 validation

## Final commands

Both final gates used inline new50 `LTP_CASES`, `OSCOMP_TEST_GROUPS=ltp`, absolute suite image paths, and a 600s outer timeout so evaluator tail work could not contaminate the LTP evidence.

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$NEW50" RV_TESTSUITE_IMG="$PWD/sdcard-rv.img" timeout 600s make run-rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$NEW50" LA_TESTSUITE_IMG="$PWD/sdcard-la.img" timeout 600s make run-la
python3 scripts/ltp_summary.py "$OUT.log" > "$OUT.summary.txt"
python3 scripts/ltp_summary.py --json "$OUT.log" > "$OUT.summary.json"
sha256sum "$OUT.log" "$OUT.summary.txt" "$OUT.summary.json" > "$OUT.sha256.txt"
```

## Final evidence paths

| Scope | Log | Summary | JSON | SHA256 |
| --- | --- | --- | --- | --- |
| RV new50 final | `target/ltp-1000-milestone-09-stable956/rv-stable956-new50-final-gate-20260605T222350+0800.log` | `target/ltp-1000-milestone-09-stable956/rv-stable956-new50-final-gate-20260605T222350+0800.summary.txt` | `target/ltp-1000-milestone-09-stable956/rv-stable956-new50-final-gate-20260605T222350+0800.summary.json` | `target/ltp-1000-milestone-09-stable956/rv-stable956-new50-final-gate-20260605T222350+0800.sha256.txt` |
| LA new50 final | `target/ltp-1000-milestone-09-stable956/la-stable956-new50-final-gate-20260605T222730+0800.log` | `target/ltp-1000-milestone-09-stable956/la-stable956-new50-final-gate-20260605T222730+0800.summary.txt` | `target/ltp-1000-milestone-09-stable956/la-stable956-new50-final-gate-20260605T222730+0800.summary.json` | `target/ltp-1000-milestone-09-stable956/la-stable956-new50-final-gate-20260605T222730+0800.sha256.txt` |

## Final parser results

- RV: RUN_RC=0, PASS LTP CASE 100, FAIL 0, internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
- LA: RUN_RC=0, PASS LTP CASE 100, FAIL 0, internal TFAIL/TBROK/TCONF 0, timeout 0, ENOSYS 0, panic/trap 0.
- Stable list count after promotion: 956 total / 956 unique / 0 duplicate.

## Supporting lane evidence

- POSIX mqueue: RV `rv-posix-mq-targeted-20260605T204144+0800.summary.txt`; LA `la-posix-mq-basic-targeted-20260605T204347+0800.summary.txt`.
- pidfd open/send-signal: RV `rv-pidfd-basic-gate-20260605T213747+0800.summary.txt`; LA `la-pidfd-open-signal01-ltp-only-gate-20260605T215327+0800.summary.txt` and `la-pidfd-send-signal02-ltp-only-gate-20260605T215438+0800.summary.txt`.
- pidfd_getfd/kcmp support: RV `rv-pidfd-getfd-ltp-only-scout-20260605T220945+0800.summary.txt`; LA `la-pidfd-getfd-ltp-only-gate-20260605T221046+0800.summary.txt`.
- inotify init flags: RV `rv-inotify-init1-ltp-only-gate-20260605T222121+0800.summary.txt`; LA `la-inotify-init1-ltp-only-gate-20260605T222225+0800.summary.txt`.

## Validation caveats

- Full all-minus-blacklist sweep was not run for this milestone.
- Diagnostic runs with outer RUN_RC=124 were excluded from promotion evidence.
- Higher inotify event-delivery cases are not claimed; only `inotify_init1` FD/flag semantics are promoted.
