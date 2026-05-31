# la-arch001 compact parser summary

- Raw log: `target/ltp-full-sweep-blacklist-2026-05-30-arch/raw/la-arch001.log`
- Parser source: `scripts/ltp_summary.py --json`
- Closure: **not closed**; leader terminated after LA resource exhaustion polluted later cases.
- run-eval status: `143`
- Selection lines:
  - line 932: `ltp case list: all-minus-blacklist skipped=41 (2327 cases, timeout 15s)`
  - line 42926: `ltp case list: all-minus-blacklist skipped=44 (2331 cases, timeout 15s)`

## Wrapper counts

- PASS LTP CASE: 555
- FAIL LTP CASE: 1861
- TIMEOUT matches: 26

## Internal LTP signals

- TBROK: 663
- TCONF: 1031
- TFAIL: 1914
- ENOSYS/not implemented matches: 601
- panic/trap matches from parser: 0

## Suite summaries

- ltp-musl: passed=553 failed=1774 timed_out=26

## Marker audit

- RUN markers: 2417
- FAIL markers: 2416
- TIMEOUT markers: 26
- SKIP markers: 0
- Incomplete RUN count at termination: 1
- Incomplete tail: `[{'case': 'check_netem', 'line': 44447}]`
- First true resource failure: `{'line': 36462, 'case': 'tcp4-multi-sameport09', 'text': 'sh: fork: Resource temporarily unavailable'}`
- Panic count: 0
- Trap-like count (strict scan, excluding TBROK/TFAIL text): 11

## Blacklist consequence

`la-arch001` is evidence for an LA-only network-stress-family blacklist, not for a common/RV blacklist.  The first true resource failure appears during `tcp4-multi-sameport09`; later glibc cases report `fork(): EAGAIN/EWOULDBLOCK`, proving environment pollution rather than ordinary isolated TFAIL.
