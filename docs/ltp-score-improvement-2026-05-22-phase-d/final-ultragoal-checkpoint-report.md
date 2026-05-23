# Final ultragoal checkpoint: stable250 and remote scorer follow-up

Created: 2026-05-23T17:56:47Z

## Final status

- stable250: complete.
- Remote LTP score-zero bug: local root cause fixed by keeping LTP marker lines at console column 1 after colored kernel logs.
- Optional stable260 stretch: closed without promotion because the accepted target was stable250 and no clean stable260 promotion was required after stable250 passed.

## Final gates

| Arch | PASS LTP CASE | FAIL LTP CASE | musl | glibc | Internal | timeout | ENOSYS | panic/trap | marker prefixes |
| --- | ---: | ---: | --- | --- | --- | ---: | ---: | ---: | --- |
| RV | 500 | 0 | 250/0 | 250/0 | TCONF 4 known `read02` | 0 | 0 | 0 | 500 clean / 0 bad |
| LA | 500 | 0 | 250/0 | 250/0 | TCONF 4 known `read02` | 0 | 0 | 0 | 500 clean / 0 bad |

## Durable evidence

- `stable250-post-ansi-rv-summary.txt`
- `stable250-post-ansi-la-summary.txt`
- `final-gate-quality-gate.json`
- `final-gate-ai-slop-cleaner-report.md`
- `final-gate-code-review-report.md`
- `remote-ltp-score-zero-ansi-fix-report.md`
- `stable250-delivery-report.md`

## Remote note

Remote evaluator score display is still not directly runnable here. The commit to test remotely is the current branch HEAD after the final evidence commit; the functional scorer-facing fix is `99f11921`.
