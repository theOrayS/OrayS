# milestone-06 stable806 validation notes

Date: 2026-06-03. This is an early stable806 scouting checkpoint, not a promotion gate.
`LTP_STABLE_CASES` remains `756 total / 756 unique / 0 duplicate`; no stable-list edit was made.

## Current baseline check

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
# 756 756 0
```

## Four-image testcase universe refresh

The leader used read-only `debugfs` listing against `sdcard-rv.img` and `sdcard-la.img` to refresh the available LTP binary names for RV/LA × musl/glibc under `target/ltp-1000-milestone-06-stable806/`:

| List | Count |
| --- | ---: |
| `rv-musl-ltp-bin-list.txt` | 2824 |
| `rv-glibc-ltp-bin-list.txt` | 2844 |
| `la-musl-ltp-bin-list.txt` | 2824 |
| `la-glibc-ltp-bin-list.txt` | 2844 |
| common to all four images, before filtering current stable | 2824 |

The archived Session-1 4/4 clean-not-stable seed list is now exhausted against the current stable756 list, so milestone-06 must mine near-clean/semantic-blocker rows rather than reusing old clean seeds.

## RV proc/synthetic/sched scout

Command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='utsname02,utsname03,sysinfo03,getrusage02,getrusage03,getrusage04,prctl02,prctl03,prctl04,prctl06,prctl07,prctl08,prctl09,prctl10,setrlimit06,setpriority01,nice04' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-proc-synthetic-sched-scout-20260603T175622+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-proc-synthetic-sched-scout-20260603T175622+0800.log
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/rv-proc-synthetic-sched-scout-20260603T175622+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/rv-proc-synthetic-sched-scout-20260603T175622+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/rv-proc-synthetic-sched-scout-20260603T175622+0800.summary.json`
- RV-only candidate report: `target/ltp-1000-milestone-06-stable806/rv-proc-synthetic-sched-scout-20260603T175622+0800.promotion-candidates.txt`

Parser result:

- PASS LTP CASE: 5
- FAIL LTP CASE: 29
- Internal signals: `{'TFAIL': 15, 'TBROK': 4, 'TCONF': 22}`
- timeout matches: 2
- ENOSYS/not implemented matches: 2
- panic/trap matches: 0
- RV-only promotion candidates: 0

Notable blockers:

- `prctl08`/`prctl09`: timerslack support is missing (`PR_SET_TIMERSLACK`, proc `timerslack_ns`), producing visible `TFAIL`/`TBROK`.
- `utsname03`: UTS namespace clone path is unsupported (`ENOSYS`), visible `TBROK`.
- `getrusage04`: timed out on both libc variants.
- `getrusage02` and `setpriority01`: wrapper PASS but visible `TCONF`, so not promotable.
- `nice04`: glibc clean, musl still fails with an errno mismatch; not promotable from 1/2 clean RV evidence.

## RV time/fd/signal scout

Command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='epoll_create01,epoll_create02,eventfd06,timerfd04,timer_delete01,timer_delete02,timer_getoverrun01,timer_gettime01,timer_settime01,timer_settime02,timer_settime03,clock_gettime01,clock_gettime03,clock_nanosleep01,clock_nanosleep03,setitimer01,sigtimedwait01,sigwait01,sigwaitinfo01' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-time-fd-signal-scout-20260603T180127+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-time-fd-signal-scout-20260603T180127+0800.log
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/rv-time-fd-signal-scout-20260603T180127+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/rv-time-fd-signal-scout-20260603T180127+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/rv-time-fd-signal-scout-20260603T180127+0800.summary.json`
- RV-only candidate report: `target/ltp-1000-milestone-06-stable806/rv-time-fd-signal-scout-20260603T180127+0800.promotion-candidates.txt`

Parser result:

- PASS LTP CASE: 3
- FAIL LTP CASE: 35
- Internal signals: `{'TCONF': 28, 'TFAIL': 12, 'TBROK': 6}`
- timeout matches: 6
- ENOSYS/not implemented matches: 3
- panic/trap matches: 0
- RV-only promotion candidates: 0

Notable blockers:

- POSIX timer syscalls (`timer_create`, `timer_delete`, `timer_gettime`, `timer_settime`) are not implemented, so timer rows remain visible `TCONF`/`TBROK`/`ENOSYS` blockers.
- `epoll_create01`/`epoll_create02` are not promotable because the syscall variant is `TCONF`; `epoll_create02` also has a musl `TFAIL` before the legacy syscall boundary is solved.
- `clock_gettime01`, `setitimer01`, `sigtimedwait01`, and `sigwaitinfo01` include timeout evidence; they must not be batched into promotion until isolated.

## Validation conclusion

Both RV scouting batches are useful blocker maps but produce zero promotion candidates. No LA confirmation was run because the RV side is not clean. No blacklist/SKIP/status0/full-sweep partial row is counted toward stable806.
