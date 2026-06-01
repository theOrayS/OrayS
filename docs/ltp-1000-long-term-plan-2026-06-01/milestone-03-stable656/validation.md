# Milestone 03 stable656 validation

Date: 2026-06-02
Branch: `dev/1000ltp-plan`
Head at run: `e9a64d35`

## Stable count before/after

Command:

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
```

Result after this scout documentation: `606 606 0`.

## RV targeted scout

Command captured in run meta:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=mmap05,munmap01,mmap10_1,mmap13,vma02,futex_wait03 \
LTP_CASE_TIMEOUT_SECS=90 \
timeout 60m ./run-eval.sh rv
```

Run meta:

```text
RUN_META started_at=2026-06-02T06:02:25+08:00
RUN_META branch=dev/1000ltp-plan
RUN_META head=e9a64d35
RUN_META cases=mmap05,munmap01,mmap10_1,mmap13,vma02,futex_wait03
RUN_META command=OSCOMP_TEST_GROUPS=ltp LTP_CASES=mmap05,munmap01,mmap10_1,mmap13,vma02,futex_wait03 LTP_CASE_TIMEOUT_SECS=90 timeout 60m ./run-eval.sh rv
RUN_META finished_at=2026-06-02T06:06:03+08:00
RUN_META exit_code=0
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.log`
- Summary: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.summary.txt`
- JSON: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.summary.json`
- Promotion report: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-03-stable656/rv-g009-mm-futex-scout-20260602T060225Z.derived.sha256`

Parser summary:

```text
PASS LTP CASE: 0
FAIL LTP CASE: 12
Internal TFAIL/TBROK/TCONF: 8 ({'TBROK': 2, 'TFAIL': 2, 'TCONF': 4})
timeout matches: 2
ENOSYS/not implemented matches: 0
panic/trap matches: 0
ltp-musl: 0 passed, 6 failed
ltp-glibc: 0 passed, 6 failed
Promotion candidates: 0
Blocked/incomplete cases: 6
```

## Gate outcome

- Targeted RV: **failed for all 6 candidate cases**.
- Adjacent stable regression subset: not run in this scout because no candidate reached RV PASS.
- LA confirmation: not run; RV already blocked.
- musl + glibc: both failed for every candidate.
- Parser blockers: `TFAIL`, `TBROK`, `TCONF`, and timeout are present.
- Stable list: unchanged at `606/606/0`.

## Unverified items

- No LA run for these blocked rows.
- No code fix for recoverable SIGSEGV/SIGBUS or futex timeout semantics in this documentation checkpoint.
- No broad all-minus-blacklist sweep in this checkpoint.
