# milestone-07 stable856 validation notes

Date: 2026-06-04.

## Baseline and stable-list check

```bash
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY
# 856 856 0
```

## Current pool audit

Artifact:

- `target/ltp-1000-milestone-07-stable856/la-stable856-new50-currentpool-gate-20260604T205650+0800/pool-audit.txt`

Parser/audit output:

```text
pool_count 50
pool_unique 50
pool_duplicates 0
pool_sha256 ae97ecb3975f7fc79fbb29b2532828b48f39011fbca30bc5f628aea634bfcd42
```

The same pool audit checksum is present in the RV and LA new50 current-pool gate directories.

## Targeted regression probe after msync EOF-tail repair

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='mmap01,setpriority02' LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-07-stable856/rv-regression-probe-mmap01-setpriority02-after-msync-eof-20260604T204927+0800/rv-raw.log

OSCOMP_TEST_GROUPS=ltp LTP_CASES='mmap01,setpriority02' LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-07-stable856/la-regression-probe-mmap01-setpriority02-after-msync-eof-20260604T205035+0800/la-raw.log
```

Results:

- RV: `4 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `4 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.

This probe closed the two known regressions from the obsolete partial RV final gate: `mmap01` after MAP_SHARED EOF-tail msync handling, and `setpriority02` after reverting the unsafe EPERM-only `setpriority` experiment.

## New50 current-pool gate

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=@target/ltp-1000-milestone-07-stable856/current-candidate-pool-latest.txt LTP_CASE_TIMEOUT_SECS=90 timeout 90m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-07-stable856/rv-stable856-new50-currentpool-gate-20260604T205151+0800/rv-raw.log

OSCOMP_TEST_GROUPS=ltp LTP_CASES=@target/ltp-1000-milestone-07-stable856/current-candidate-pool-latest.txt LTP_CASE_TIMEOUT_SECS=90 timeout 120m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-07-stable856/la-stable856-new50-currentpool-gate-20260604T205650+0800/la-raw.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la \
  target/ltp-1000-milestone-07-stable856/rv-stable856-new50-currentpool-gate-20260604T205151+0800/rv-raw.log \
  target/ltp-1000-milestone-07-stable856/la-stable856-new50-currentpool-gate-20260604T205650+0800/la-raw.log
```

Results:

- RV new50: `100 PASS / 0 FAIL / 0 internal markers`; ltp-musl `50/0`, ltp-glibc `50/0`.
- LA new50: `100 PASS / 0 FAIL / 0 internal markers`; ltp-musl `50/0`, ltp-glibc `50/0`.
- Four-way promotion candidates: `50`; blocked/incomplete cases: `0`.

## Full stable856 final gates

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 180m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-07-stable856/rv-stable856-final-gate-20260604T210444+0800/rv-raw.log

OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 220m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-07-stable856/la-stable856-final-gate-20260604T215235+0800/la-raw.log
```

Both `run-eval.sh` invocations exited with status `0`.

Results:

- RV full stable856: `1712 PASS / 0 FAIL`; ltp-musl `856 PASS / 0 FAIL`, ltp-glibc `856 PASS / 0 FAIL`; `timeout=0`, `ENOSYS=0`, `panic/trap=0`, `internal_tfail=0`, `internal_tbrok=0`.
- LA full stable856: `1712 PASS / 0 FAIL`; ltp-musl `856 PASS / 0 FAIL`, ltp-glibc `856 PASS / 0 FAIL`; `timeout=0`, `ENOSYS=0`, `panic/trap=0`, `internal_tfail=0`, `internal_tbrok=0`.
- Inherited caveat: both full-stable gates report `pass_with_tconf` for `read02` only (`rv:glibc:read02`, `rv:musl:read02`, `la:glibc:read02`, `la:musl:read02`) due `O_DIRECT not supported on tmpfs`. This caveat predates milestone-07 and is not used as new promotion evidence.

## Final local checks before commit

Required before the milestone commit:

```bash
cargo fmt -- --check
git diff --check
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY
```

## Not verified in this milestone

- No full `all-minus-blacklist` sweep was run for this milestone.
- No raw logs from obsolete/aborted gates are counted as promotion evidence.
