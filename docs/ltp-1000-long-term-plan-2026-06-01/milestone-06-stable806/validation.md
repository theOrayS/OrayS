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



## Timerslack blocker repair and targeted retest

A generic timerslack implementation was added after the RV scouts identified `prctl08`/`prctl09` blockers. The implementation covers:

- `prctl(PR_SET_TIMERSLACK, value)` and `prctl(PR_GET_TIMERSLACK)`.
- Per-process current/default timerslack state, with initial default `50000` ns.
- Fork inheritance where the child current and default timerslack values are both initialized from the creating thread current value.
- `/proc/self/timerslack_ns` and `/proc/<pid>/timerslack_ns` read/write synthetic proc entries.

An intermediate RV run before the default/current split is retained as diagnostic evidence only:

- Raw log: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-20260603T182915+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-20260603T182915+0800.summary.txt`
- Result: `prctl09` passed on RV × musl/glibc, but `prctl08` still failed because `PR_SET_TIMERSLACK(0)` reset the child default to the global `50000` ns instead of the creating thread current value.

Final RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='prctl08,prctl09' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.log
```

Final RV artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.summary.json`
- RV-only candidate report: `target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.promotion-candidates.txt`

Final RV parser result:

- PASS LTP CASE: 4
- FAIL LTP CASE: 0
- Internal signals: `{}`
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- RV-only promotion candidates: 2 (`prctl08`, `prctl09`)

Final LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='prctl08,prctl09' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.log
```

Final LA artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.summary.json`
- LA-only candidate report: `target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.promotion-candidates.txt`

Final LA parser result:

- PASS LTP CASE: 4
- FAIL LTP CASE: 0
- Internal signals: `{}`
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- LA-only promotion candidates: 2 (`prctl08`, `prctl09`)

## Leader local verification

After the documentation refresh, the leader reran local non-QEMU checks plus parser replays:

```bash
df -h / /root
cargo fmt -- --check
cargo check -p arceos-shell
git diff --check -- <timerslack source files and milestone-06 docs>
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-prctl08-09-after-timerslack-default-inherit-20260603T183244+0800.log
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-prctl08-09-after-timerslack-default-inherit-20260603T183438+0800.log
```

Result:

- Disk headroom before local checks: `/` and `/root` 59G total, 25G used, 32G available, 44%.
- `cargo fmt -- --check`: passed.
- `cargo check -p arceos-shell`: passed.
- `git diff --check` for the timerslack source/doc patch: passed.
- Stable list recount: `756 total / 756 unique / 0 duplicate`.
- Final RV/LA parser replays match the stored summaries: both logs remain `4 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.


## Validation conclusion

The original two RV scouting batches remain blocker maps. After the generic timerslack repair, `prctl08` and `prctl09` are parser-clean on RV + LA × musl + glibc and are recorded as promotion candidates for a later stable806 batch. They are not added to `LTP_STABLE_CASES` in this commit because milestone-06 still lacks the next 50-case clean cohort and adjacent stable-regression gate. No blacklist/SKIP/status0/full-sweep partial row is counted toward stable806.

## RV socket broad scout (partial/blocker evidence only)

A broad socket-core scout was attempted after the timerslack repair to identify the next lane. The run is explicitly **not promotion evidence** because glibc `accept02` reached `The futex facility returned an unexpected error code.` and the QEMU process was terminated by the leader to avoid an unbounded hang.

Command shape:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='accept02,accept03,accept4_01,bind01,bind02,bind03,bind04,bind05,bind06,connect01,connect02,getsockopt02,setsockopt02,setsockopt03,setsockopt04,setsockopt05,setsockopt06,recv01,recvfrom01,recvmsg01,send01,send02,sendto01,sendto02,sendto03,sockioctl01' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh rv
```

Artifacts:

- Partial raw log: `target/ltp-1000-milestone-06-stable806/rv-socket-core-scout-20260603T184807+0800.log`
- Partial summary: `target/ltp-1000-milestone-06-stable806/rv-socket-core-scout-20260603T184807+0800.partial-summary.txt`
- Partial JSON: `target/ltp-1000-milestone-06-stable806/rv-socket-core-scout-20260603T184807+0800.partial-summary.json`
- Partial candidate report: `target/ltp-1000-milestone-06-stable806/rv-socket-core-scout-20260603T184807+0800.partial-promotion-candidates.txt`

Parser result from the partial log:

- PASS LTP CASE: 4
- FAIL LTP CASE: 22
- Internal signals: `{'TCONF': 23, 'TFAIL': 12, 'TBROK': 17}`
- timeout matches: 0
- ENOSYS/not implemented matches: 12
- panic/trap matches: 0
- Promotion candidates: 0

Conclusion: socket rows remain blocker/scouting material only; no partial TPASS, blacklist, or unknown glibc state is counted.

## UTS shared-hostname repair and targeted retest

The upstream LTP `utsname02` container test checks that two plain forked processes share the same UTS hostname: child 1 calls `sethostname("LTP_HOSTNAME")`, then child 2 must observe that hostname through `gethostname()`. The local model previously copied `hostname: Mutex<String>` during `fork()`, so sibling processes saw stale per-process hostnames.

The repair changes `UserProcess::hostname` to `Arc<Mutex<String>>` and makes `fork()` share the same Arc for plain processes. This models the default shared UTS namespace without implementing `CLONE_NEWUTS`.

RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='utsname01,utsname02' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.log
```

RV artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.summary.json`
- RV-only candidate report: `target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.promotion-candidates.txt`

RV parser result:

- PASS LTP CASE: 4
- FAIL LTP CASE: 0
- Internal signals: `{}`
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- RV-only promotion candidates: 2 (`utsname01`, `utsname02`); `utsname01` is already stable and is regression coverage, so only `utsname02` is a new unique candidate.

LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='utsname01,utsname02' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.log
```

LA artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.summary.json`
- LA-only candidate report: `target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.promotion-candidates.txt`

LA parser result:

- PASS LTP CASE: 4
- FAIL LTP CASE: 0
- Internal signals: `{}`
- timeout matches: 0
- ENOSYS/not implemented matches: 0
- panic/trap matches: 0
- LA-only promotion candidates: 2 (`utsname01`, `utsname02`); only `utsname02` is new unique evidence.

Combined four-combo candidate report:

```bash
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la \
  target/ltp-1000-milestone-06-stable806/rv-utsname-shared-hostname-20260603T190100+0800.log \
  target/ltp-1000-milestone-06-stable806/la-utsname-shared-hostname-20260603T190234+0800.log
```

Artifact: `target/ltp-1000-milestone-06-stable806/rv-la-utsname-shared-hostname-20260603T190408+0800.promotion-candidates.txt`

Result: 2 four-combo clean rows (`utsname01`, `utsname02`), where `utsname02` is the new unique stable806 candidate.

## UTS adjacent stable regression subset

The UTS/hostname repair affects visible `sethostname`, `gethostname`, and `uname` nodename behavior. The leader therefore ran the adjacent stable subset on RV and LA before committing the patch:

```bash
CASES='gethostname01,sethostname01,sethostname02,sethostname03,uname01,uname02,uname04,newuname01,utsname01,utsname04'
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$CASES" LTP_CASE_TIMEOUT_SECS=45 timeout 60m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$CASES" LTP_CASE_TIMEOUT_SECS=45 timeout 60m ./run-eval.sh la
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-06-stable806/rv-utsname-adjacent-regression-20260603T190435+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-utsname-adjacent-regression-20260603T190435+0800.summary.txt`
- LA raw log: `target/ltp-1000-milestone-06-stable806/la-utsname-adjacent-regression-20260603T190701+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-utsname-adjacent-regression-20260603T190701+0800.summary.txt`

Parser result:

- RV: `20 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `20 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.

## UTS local verification

```bash
cargo fmt -- --check
cargo check -p arceos-shell
```

Result: both passed before QEMU targeted gates. Stable list remained `756 total / 756 unique / 0 duplicate`; no stable list edit was made.


## Updated validation conclusion after UTS repair

`utsname02` is now a new unique stable806 candidate with RV + LA × musl + glibc parser-clean evidence. The adjacent stable UTS/hostname/uname subset is parser-clean on both architectures. At that point, before the later VFS repair, the candidate pool was `prctl08`, `prctl09`, and `utsname02` (3 new unique cases). `LTP_STABLE_CASES` remains `756 total / 756 unique / 0 duplicate` because the next milestone commit requires a full +50 unique clean cohort. Socket scout evidence remains blocker-only and contributes zero candidates.

## Readlink/readlinkat near-clean blocker triage

`readlink03` and `readlinkat02` were rechecked because old summary aggregation showed them as three-combo clean rows: RV × musl/glibc and LA × glibc were clean, while LA × musl remained failing.

Commands:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='readlink03,readlinkat02' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES='readlink03,readlinkat02' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la \
  target/ltp-1000-milestone-06-stable806/rv-readlink03-readlinkat02-20260603T191956+0800.log \
  target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-20260603T192126+0800.log
```

Artifacts:

- RV raw log: `target/ltp-1000-milestone-06-stable806/rv-readlink03-readlinkat02-20260603T191956+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-readlink03-readlinkat02-20260603T191956+0800.summary.txt`
- LA raw log: `target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-20260603T192126+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-20260603T192126+0800.summary.txt`
- Combined report: `target/ltp-1000-milestone-06-stable806/la-readlink03-readlinkat02-20260603T192126+0800.combined-promotion-candidates.txt`
- Temporary debug trace: `target/ltp-1000-milestone-06-stable806/la-readlinkat02-debug-20260603T192649+0800.log`

Parser result:

- RV: `4 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `2 PASS / 2 FAIL / 2 TFAIL / 0 timeout / 0 ENOSYS / 0 panic/trap`; both failures are LA musl rows.
- Combined RV+LA candidate report: `0` candidates; `readlink03` and `readlinkat02` are blocked by LA musl `TFAIL`.

Boundary finding:

- The current kernel path already returns `EINVAL` when the kernel actually receives `bufsiz == 0`.
- Temporary LA debug instrumentation showed the failing musl `readlinkat(3, symlink_file, NULL, 0)` test reached the kernel as `bufsiz=0x1` with a non-null buffer, while the LA glibc row reached the kernel as `bufsiz=0x0` and passed.
- Treating `bufsiz=1` as `EINVAL` would break real Linux/POSIX `readlink/readlinkat` semantics for legitimate one-byte buffers, so no kernel patch was made and these rows are not promotion candidates.

## RV statx VFS scout

Command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='statx01,statx04,statx05,statx06,statx07,statx08,statx09,statx10,statx11,statx12' \
LTP_CASE_TIMEOUT_SECS=45 timeout 60m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-statx-vfs-scout-20260603T193211+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-statx-vfs-scout-20260603T193211+0800.log
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/rv-statx-vfs-scout-20260603T193211+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/rv-statx-vfs-scout-20260603T193211+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/rv-statx-vfs-scout-20260603T193211+0800.summary.json`
- RV-only candidate report: `target/ltp-1000-milestone-06-stable806/rv-statx-vfs-scout-20260603T193211+0800.promotion-candidates.txt`

Parser result:

- PASS LTP CASE: 2 (`statx01` on both libcs), but both are `pass_with_tconf`.
- FAIL LTP CASE: 18.
- Internal signals: `{'TCONF': 32}`.
- timeout matches: 2 (`statx11` on both libcs).
- ENOSYS/not implemented matches: 0.
- panic/trap matches: 0.
- RV-only promotion candidates: 0.

Conclusion: the statx lane remains blocker/scout material only. `statx01` cannot be promoted because parser-visible `TCONF` disqualifies it, and `statx11` timeout evidence makes the batch unsuitable for promotion.

## RV credential/capability scout

Command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='gettid02,getuid01_16,getuid03_16,setuid01_16,setuid03_16,setuid04_16,capget01,capget02,capset01,capset02,capset03,capset04' \
LTP_CASE_TIMEOUT_SECS=45 timeout 60m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-cred-cap-scout-20260603T193548+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-cred-cap-scout-20260603T193548+0800.log
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/rv-cred-cap-scout-20260603T193548+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/rv-cred-cap-scout-20260603T193548+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/rv-cred-cap-scout-20260603T193548+0800.summary.json`
- RV-only candidate report: `target/ltp-1000-milestone-06-stable806/rv-cred-cap-scout-20260603T193548+0800.promotion-candidates.txt`

Parser result:

- PASS LTP CASE: 1 (`gettid02` on RV musl only).
- FAIL LTP CASE: 23.
- Internal signals: `{'TCONF': 22, 'TBROK': 1}`.
- timeout matches: 0.
- ENOSYS/not implemented matches: 0.
- panic/trap matches: 0.
- RV-only promotion candidates: 0.

Conclusion at this scout point: 16-bit UID and capability rows are currently visible `TCONF` blockers, not promotion evidence. `gettid02` was only musl-clean in this earlier RV-only scout because the RV glibc row hit a `TBROK` futex-abort path; that `gettid02` blocker is superseded by the later futex/glibc follow-up evidence below.

## Nice errno boundary

`nice04` remains blocked by a libc-visible errno split in the existing RV proc/synthetic/sched scout:

- RV glibc: `nice(-10) failed with EPERM` and wrapper PASS.
- RV musl: `nice(-10) should fail with EPERM: EACCES (13)` and wrapper FAIL/TFAIL.

The shared priority implementation currently returns `EACCES` when a non-root caller attempts to lower a nice value through `setpriority`. Other stable setpriority rows rely on that generic boundary, and the kernel cannot safely distinguish musl `nice(-10)` wrapper traffic from a direct `setpriority(2)` call without introducing case/libc-specific behavior. No source change was made; `nice04` stays blocked until a principled libc/ABI-compatible fix is available.

## Updated validation conclusion after blocker triage

The post-UTS blocker triage added zero promotion candidates at that point. `readlink03`, `readlinkat02`, `nice04`, the statx scout, and the credential/capability scout all remained excluded because their evidence contained visible `TFAIL`, `TBROK`, `TCONF`, or timeout markers, or would require a semantically unsafe kernel-only workaround. The `gettid02` row from that scout is superseded by the later futex/glibc follow-up evidence below. At that pre-VFS-repair point, the milestone-06 candidate pool remained `prctl08`, `prctl09`, and `utsname02` (3 new unique cases), and `LTP_STABLE_CASES` remained `756 total / 756 unique / 0 duplicate`.

## RV VFS/FD/select scout

After the readlink/statx/credential triage produced no safe candidates, the leader ran a small four-image-present, non-stable VFS/FD/select RV scout. This was a scout-only run; visible parser blockers disqualify every row from promotion.

Command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='unlink09,rename11,renameat01,symlink03,mknod07,mknodat02,mkdir02,mkdir03,mkdir09,mkdirat02,rmdir02,openat03,openat04,fcntl17,fcntl17_64,fcntl24,fcntl24_64,fcntl25,fcntl25_64,fcntl26,fcntl26_64,fcntl27,fcntl27_64,select01,select02,select03,select04' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.log
```

Artifacts:

- Raw log: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.log`
- Summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.summary.txt`
- JSON: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.summary.json`
- RV-only candidate report: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.promotion-candidates.txt`
- Case list: `target/ltp-1000-milestone-06-stable806/rv-vfs-fd-select-scout-20260603T194925+0800.cases.txt`

Parser result:

- PASS LTP CASE: 9.
- FAIL LTP CASE: 45.
- Internal signals: `{'TBROK': 7, 'TCONF': 112, 'TFAIL': 26}`.
- timeout matches: 4 (`fcntl17`, `fcntl17_64` on both libcs).
- ENOSYS/not implemented matches: 0.
- panic/trap matches: 0.
- RV-only promotion candidates: 0.

Notable boundaries:

- `select01`..`select04` have wrapper PASS but visible `TCONF` markers for unsupported select syscall variants, so they are `pass_with_tconf` and not promotable.
- `fcntl17`/`fcntl17_64` timed out on both libcs and should be isolated before any fcntl locking work.
- `fcntl24`..`fcntl26_64`, `mknod07`, `mknodat02`, `openat03`, `openat04`, `rename11`, and `renameat01` are mostly environment/unsupported-feature `TCONF` rows.
- `mkdir02`, `mkdir03`, `mkdirat02`, and `rmdir02` expose real path/errno `TFAIL` blockers; `unlink09` and `symlink03` expose `TBROK` blockers.

Conclusion: this scout adds zero stable806 candidates and updates the next-lane map toward small, isolated VFS errno fixes or fcntl-lock lifetime fixes rather than broad-batch promotion.


## VFS parent-symlink/rmdir errno repair and targeted retest

The RV VFS/FD/select scout exposed real `mkdirat02`/`rmdir02` path and errno blockers. The source repair was generic: creation/removal syscalls now resolve symlink parents before operating, `unlinkat(..., AT_REMOVEDIR)` rejects a final `.` component with `EINVAL`, and rmdir of a process mountpoint returns `EBUSY`. No LTP case/path/process/output hardcoding was added.

Commands:

```bash
cargo fmt -- --check
cargo check -p arceos-shell
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='unlink09,symlink03,mkdir02,mkdir03,mkdir09,mkdirat02,rmdir02,mknod07,mknodat02' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv \
  target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES='mkdirat02,rmdir02' \
LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la \
  target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.log \
  target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.log
```

Artifacts:

- RV case list: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.cases.txt`
- RV raw log: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.summary.json`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-fix-20260603T200303+0800.promotion-candidates.txt`
- LA case list: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.cases.txt`
- LA raw log: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.summary.json`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.promotion-candidates.txt`
- Combined RV+LA candidate report: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-fix-candidates-20260603T200510+0800.combined-promotion-candidates.txt`

Parser result:

- RV targeted run: `5 PASS / 13 FAIL`, internal `{'TBROK': 7, 'TFAIL': 6, 'TCONF': 8}`, `0 timeout`, `0 ENOSYS`, `0 panic/trap`. `mkdirat02` and `rmdir02` are RV-clean; the other rows remain visibly blocked and excluded.
- LA targeted run for the two RV-clean candidates: `4 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined RV+LA report: `2` four-combo candidates, `mkdirat02` and `rmdir02`.

Adjacent stable regression subset:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='mkdir04,mkdir05,mkdirat01,rmdir01,rmdir03,unlink05,unlink07,unlink08,unlinkat01,symlink01,symlink02,symlink04,symlinkat01,mknod01,mknod02,mknod03,mknod04,mknod05,mknod06,mknod08,mknod09,mknodat01,rename01,rename03,rename04,rename05,rename06,rename07,rename08,rename09,rename10,rename12,rename13,rename14,renameat201,renameat202' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='mkdir04,mkdir05,mkdirat01,rmdir01,rmdir03,unlink05,unlink07,unlink08,unlinkat01,symlink01,symlink02,symlink04,symlinkat01,mknod01,mknod02,mknod03,mknod04,mknod05,mknod06,mknod08,mknod09,mknodat01,rename01,rename03,rename04,rename05,rename06,rename07,rename08,rename09,rename10,rename12,rename13,rename14,renameat201,renameat202' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh la
```

Artifacts:

- RV adjacent case list: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.cases.txt`
- RV adjacent raw log: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.log`
- RV adjacent summary: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.summary.txt`
- RV adjacent JSON: `target/ltp-1000-milestone-06-stable806/rv-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.summary.json`
- LA adjacent case list: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.cases.txt`
- LA adjacent raw log: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.log`
- LA adjacent summary: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.summary.txt`
- LA adjacent JSON: `target/ltp-1000-milestone-06-stable806/la-vfs-parent-symlink-rmdir-adjacent-regression-20260603T200657+0800.summary.json`

Adjacent parser result:

- RV adjacent subset: `72 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA adjacent subset: `72 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.

Updated validation conclusion after VFS repair: `mkdirat02` and `rmdir02` are new four-combo stable806 candidates. The candidate pool is now `prctl08`, `prctl09`, `utsname02`, `mkdirat02`, and `rmdir02` (5 new unique cases). `LTP_STABLE_CASES` remains `756 total / 756 unique / 0 duplicate` because the next milestone promotion requires a complete +50 unique cohort.

## mkdir setgid/final-symlink repair and targeted retest

A generic metadata/path repair was added after the VFS scout showed `mkdir02`/`mkdir03` blockers:

- Directory `chown` now preserves `S_ISGID`, allowing setgid directories to keep inheritance semantics after the test setup changes group ownership.
- `mkdirat` and `mknodat` now treat a final process-visible synthetic symlink as an existing path and return `EEXIST` before creation.

Targeted RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='mkdir02,mkdir03,mkdirat02,rmdir02' LTP_CASE_TIMEOUT_SECS=45 timeout 60m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log
```

Targeted LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='mkdir02,mkdir03,mkdirat02,rmdir02' LTP_CASE_TIMEOUT_SECS=45 timeout 60m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log
```

Targeted artifacts:

- RV log: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.summary.json`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.promotion-candidates.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.summary.json`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.promotion-candidates.txt`
- Combined candidate report: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-fix-20260603T202536+08:00.combined-promotion-candidates.txt`

Targeted parser result:

- RV: `8 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `8 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined RV+LA report: four-combo candidates `mkdir02`, `mkdir03`, `mkdirat02`, and `rmdir02`; only `mkdir02` and `mkdir03` are new unique candidates in this checkpoint.

Adjacent stable regression command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='chmod01,chmod05,fchmod01,fchmod05,fchmodat02,chown01,chown02,chown03,chown05,fchown01,fchown02,fchown03,fchown05,fchownat01,open10,creat08,creat09,mkdir04,mkdir05,mkdirat01,mknod01,mknod02,mknod03,mknod04,mknod05,mknod06,mknod08,mknod09,mknodat01,symlink01,symlink02,symlink04,symlinkat01,rmdir01,rmdir03' \
LTP_CASE_TIMEOUT_SECS=45 timeout 120m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.log

OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='chmod01,chmod05,fchmod01,fchmod05,fchmodat02,chown01,chown02,chown03,chown05,fchown01,fchown02,fchown03,fchown05,fchownat01,open10,creat08,creat09,mkdir04,mkdir05,mkdirat01,mknod01,mknod02,mknod03,mknod04,mknod05,mknod06,mknod08,mknod09,mknodat01,symlink01,symlink02,symlink04,symlinkat01,rmdir01,rmdir03' \
LTP_CASE_TIMEOUT_SECS=45 timeout 120m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.log
```

Adjacent regression artifacts:

- RV cases: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.cases.txt`
- RV log: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.summary.json`
- LA cases: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.cases.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-mkdir-setgid-symlink-exist-adjacent-regression-20260603T202536+08:00.summary.json`

Adjacent parser result:

- RV: `70 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `70 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.

Validation conclusion for this repair:

`mkdir02` and `mkdir03` are added to the stable806 candidate pool with four-combo clean evidence. `mkdirat02` and `rmdir02` remain clean after the same patch. The stable list remains `756 total / 756 unique / 0 duplicate`; no promotion commit is made because the candidate pool is only 7/50 for stable806.

## fcntl27 read-lease access repair and targeted retest

A generic `fcntl(F_SETLEASE)` access-mode repair was added after the RV VFS/FD isolation scout confirmed `fcntl27` failed because `F_SETLEASE,F_RDLCK` on write-open descriptors incorrectly succeeded. The same isolation scout also exposed `symlink03` and glibc `mkdir09` as blocker rows; later `symlink03` evidence in this file supersedes only the `symlink03` blocker, while `mkdir09` remains excluded.

Isolation scout command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='symlink03,mkdir09,fcntl27' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-vfs-fd-isolation-scout-20260603T211800+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-vfs-fd-isolation-scout-20260603T211800+0800.log
```

Isolation scout result:

- RV: `1 PASS / 5 FAIL / TBROK=5 / TFAIL=4 / 0 timeout / 0 ENOSYS / 0 panic/trap`; zero promotion candidates. `fcntl27` was selected for a real access-mode fix; `symlink03` required a later tmpdir/parent-permission repair, and glibc `mkdir09` remained excluded until the futex bitset repair below.

Targeted RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='fcntl27' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.log
```

Targeted LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='fcntl27' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.log target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.log
```

Targeted artifacts:

- RV log: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.summary.json`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-access-fix-20260603T212200+0800.promotion-candidates.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.summary.json`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.promotion-candidates.txt`
- Combined candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-access-fix-20260603T212200+0800.combined-promotion-candidates.txt`

Targeted parser result:

- RV: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined RV+LA report: four-combo candidate `fcntl27`.

Adjacent stable regression command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='fcntl01,fcntl02,fcntl03,fcntl04,fcntl08,fcntl09,fcntl10,fcntl16,fcntl29,fcntl01_64,fcntl02_64,fcntl03_64,fcntl04_64,fcntl08_64,fcntl09_64,fcntl10_64,fcntl16_64,fcntl29_64,fcntl23,fcntl23_64,fcntl05,fcntl05_64,fcntl12,fcntl12_64,fcntl13,fcntl13_64,fcntl07,fcntl07_64,fcntl18,fcntl18_64,fcntl11,fcntl14,fcntl19,fcntl22,fcntl19_64,fcntl20,fcntl20_64,fcntl21,fcntl21_64,fcntl22_64,fcntl30,fcntl11_64,fcntl15,fcntl14_64,fcntl15_64,fcntl30_64,fcntl35,fcntl35_64,fcntl27' \
LTP_CASE_TIMEOUT_SECS=45 timeout 150m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.log

OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='fcntl01,fcntl02,fcntl03,fcntl04,fcntl08,fcntl09,fcntl10,fcntl16,fcntl29,fcntl01_64,fcntl02_64,fcntl03_64,fcntl04_64,fcntl08_64,fcntl09_64,fcntl10_64,fcntl16_64,fcntl29_64,fcntl23,fcntl23_64,fcntl05,fcntl05_64,fcntl12,fcntl12_64,fcntl13,fcntl13_64,fcntl07,fcntl07_64,fcntl18,fcntl18_64,fcntl11,fcntl14,fcntl19,fcntl22,fcntl19_64,fcntl20,fcntl20_64,fcntl21,fcntl21_64,fcntl22_64,fcntl30,fcntl11_64,fcntl15,fcntl14_64,fcntl15_64,fcntl30_64,fcntl35,fcntl35_64,fcntl27' \
LTP_CASE_TIMEOUT_SECS=45 timeout 150m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.log
```

Adjacent regression artifacts:

- RV cases: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.cases.txt`
- RV log: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.summary.json`
- LA cases: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.cases.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-fcntl27-read-lease-adjacent-regression-20260603T212200+0800.summary.json`

Adjacent parser result:

- RV: `98 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `98 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.

Validation conclusion for this repair:

`fcntl27` is added to the stable806 candidate pool with four-combo clean evidence and full current-stable fcntl adjacency. The stable list remains `756 total / 756 unique / 0 duplicate`; no promotion commit is made because the candidate pool is only 8/50 for stable806.

## fcntl27_64 same-source read-lease targeted retest

After the generic `F_SETLEASE,F_RDLCK` access-mode repair was committed for `fcntl27`, the same LTP source variant `fcntl27_64` was retested as a candidate-only follow-up. No additional source change was made; this validates the generic errno rule against the 64-bit LTP variant.

Targeted RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='fcntl27_64' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log
```

Targeted LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='fcntl27_64' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log
```

Artifacts:

- RV cases: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.cases.txt`
- RV log: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.summary.json`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-fcntl27-64-read-lease-access-scout-20260603T210950+0800.promotion-candidates.txt`
- LA cases: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.cases.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.summary.json`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.promotion-candidates.txt`
- Combined candidate report: `target/ltp-1000-milestone-06-stable806/la-fcntl27-64-read-lease-access-scout-20260603T210950+0800.combined-promotion-candidates.txt`

Parser result:

- RV: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined RV+LA report: four-combo candidate `fcntl27_64`.

Validation conclusion for this follow-up:

`fcntl27_64` is added to the stable806 candidate pool with four-combo clean evidence. The stable list remains `756 total / 756 unique / 0 duplicate`; no promotion commit is made because the candidate pool is only 9/50 for stable806.



## symlink03 tmpdir and parent permission targeted repair

The earlier VFS/FD isolation scout showed `symlink03` failing before promotion eligibility. The first real fix seeded per-process path metadata for `/tmp` and `/tmp/ltp-work` as Linux-like world-writable sticky directories (`01777`), which removed the setuid child `mkdtemp()` `TBROK` but exposed a second generic errno bug: `symlink()` could create entries under parents that should fail with `EACCES` or `ENOTDIR`. The final repair made `sys_symlinkat` call the existing generic parent write/search/type permission gate before recording the synthetic symlink.

Failed diagnostic artifacts, not promotion evidence:

- RV scratch-permission diagnostic log: `target/ltp-1000-milestone-06-stable806/rv-symlink03-ltp-scratch-perms-fix-20260603T211855+0800.log`
- RV scratch-permission diagnostic summary: `target/ltp-1000-milestone-06-stable806/rv-symlink03-ltp-scratch-perms-fix-20260603T211855+0800.summary.txt` — `0 PASS / 2 FAIL / TBROK=4`.
- RV tmp-mode-only diagnostic log: `target/ltp-1000-milestone-06-stable806/rv-symlink03-initial-tmp-mode-fix-20260603T212433+0800.log`
- RV tmp-mode-only diagnostic summary: `target/ltp-1000-milestone-06-stable806/rv-symlink03-initial-tmp-mode-fix-20260603T212433+0800.summary.txt` — `0 PASS / 2 FAIL / TFAIL=4`.

Targeted RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='symlink03' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.log
```

Targeted LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='symlink03' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.log target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.log
```

Targeted artifacts:

- RV cases: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.cases.txt`
- RV log: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.summary.json`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-fix-20260603T212914+0800.promotion-candidates.txt`
- LA cases: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.cases.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.summary.json`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.promotion-candidates.txt`
- Combined candidate report: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-fix-20260603T212914+0800.combined-promotion-candidates.txt`

Targeted parser result:

- RV: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined RV+LA report: four-combo candidate `symlink03`.

Adjacent stable regression command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='access01,access02,access03,access04,faccessat01,faccessat02,chmod01,chmod03,chmod05,symlink01,symlink02,symlink04,symlinkat01,readlink01,readlinkat01,link02,linkat01,unlinkat01,rmdir01,mkdir04' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-adjacent-regression-20260603T213226+0800.log

OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='access01,access02,access03,access04,faccessat01,faccessat02,chmod01,chmod03,chmod05,symlink01,symlink02,symlink04,symlinkat01,readlink01,readlinkat01,link02,linkat01,unlinkat01,rmdir01,mkdir04' \
LTP_CASE_TIMEOUT_SECS=45 timeout 90m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-adjacent-regression-20260603T213538+0800.log
```

Adjacent regression artifacts:

- RV cases: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-adjacent-regression-20260603T213226+0800.cases.txt`
- RV log: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-adjacent-regression-20260603T213226+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-adjacent-regression-20260603T213226+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-symlink03-parent-permission-adjacent-regression-20260603T213226+0800.summary.json`
- LA cases: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-adjacent-regression-20260603T213538+0800.cases.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-adjacent-regression-20260603T213538+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-adjacent-regression-20260603T213538+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-symlink03-parent-permission-adjacent-regression-20260603T213538+0800.summary.json`

Adjacent parser result:

- RV: `40 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `40 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.

Validation conclusion for this repair:

`symlink03` is added to the stable806 candidate pool with four-combo clean evidence and a 20-case symlink/access/readlink/link/unlink/rmdir/mkdir adjacent stable subset on both architectures. The stable list remains `756 total / 756 unique / 0 duplicate`; no promotion commit is made because the candidate pool is only 10/50 for stable806.

## unlink09 FS_IOC inode-flags repair and targeted retest

After the `symlink03` parent-permission repair, `unlink09` still failed during the immutable/append-only setup path because `ioctl(FS_IOC_GETFLAGS)` returned `ENOTTY` on regular file descriptors. The repair adds a generic in-memory per-path inode flag store and handles `FS_IOC_GETFLAGS`/`FS_IOC_SETFLAGS` for path-backed file descriptors. `unlink`/`unlinkat` now reject deletion of paths with `FS_IMMUTABLE_FL` or `FS_APPEND_FL` with `EPERM`, while the existing read-only mount path still returns `EROFS`.

Failed diagnostic evidence, not promotion evidence:

- RV pre-fix log: `target/ltp-1000-milestone-06-stable806/rv-unlink09-after-symlink03-perms-20260603T215126+0800.log`
- RV pre-fix summary: `target/ltp-1000-milestone-06-stable806/rv-unlink09-after-symlink03-perms-20260603T215126+0800.summary.txt` — `0 PASS / 2 FAIL / TBROK=2`, rooted in `FS_IOC_GETFLAGS` returning `ENOTTY`.

Targeted RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='unlink09' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.log
```

Targeted LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='unlink09' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.log target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.log
```

Targeted artifacts:

- RV log: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.summary.json`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-flags-fix-20260603T215832+0800.promotion-candidates.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.summary.json`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.promotion-candidates.txt`
- Combined RV+LA candidate report: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-flags-fix-20260603T220000+0800.combined-promotion-candidates.txt`

Targeted parser result:

- RV: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined RV+LA report: four-combo candidate `unlink09`.

Adjacent stable regression command:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='access01,access02,access03,access04,faccessat01,faccessat02,chmod01,chmod03,chmod05,symlink01,symlink02,symlink04,symlinkat01,readlink01,readlinkat01,link02,linkat01,unlink05,unlink07,unlinkat01,rmdir01,mkdir04,unlink09' \
LTP_CASE_TIMEOUT_SECS=45 timeout 60m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.log

OSCOMP_TEST_GROUPS=ltp \
LTP_CASES='access01,access02,access03,access04,faccessat01,faccessat02,chmod01,chmod03,chmod05,symlink01,symlink02,symlink04,symlinkat01,readlink01,readlinkat01,link02,linkat01,unlink05,unlink07,unlinkat01,rmdir01,mkdir04,unlink09' \
LTP_CASE_TIMEOUT_SECS=45 timeout 60m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.log target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.log
```

Adjacent regression artifacts:

- Cases list: `target/ltp-1000-milestone-06-stable806/unlink09-adjacent-regression-20260603T220147+0800.cases`
- RV log: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.summary.json`
- LA log: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.summary.json`
- Combined adjacent candidate report: `target/ltp-1000-milestone-06-stable806/la-unlink09-fs-ioc-adjacent-regression-20260603T220147+0800.combined-promotion-candidates.txt`

Adjacent parser result:

- RV: `46 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA: `46 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.

Validation conclusion for this repair:

`unlink09` is added to the stable806 candidate pool with four-combo clean evidence and a 23-case unlink/path-permission adjacent stable subset on both architectures. The stable list remains `756 total / 756 unique / 0 duplicate`; no promotion commit is made because the candidate pool is only 11/50 for stable806.

## mkdir09 futex bitset repair and targeted retest

A current RV retest before this patch reproduced the old blocker: `mkdir09` passed under musl but glibc aborted with `The futex facility returned an unexpected error code.` and an LTP `TBROK`. Upstream LTP 20240524 `mkdir09` creates several pthread workers that repeatedly run `mkdir`/`rmdir` under tmpfs; the failure therefore exposed a generic glibc pthread/futex command gap rather than a mkdir-specific VFS assertion.

The repair adds generic support for `FUTEX_WAIT_BITSET` and `FUTEX_WAKE_BITSET` in `sys_futex`, including `EINVAL` for a zero bitset and absolute timeout conversion for wait-bitset calls. The current implementation intentionally reuses the existing futex wait queue and allows harmless over-wake for nonzero bitsets; callers still recheck the futex word.

Pre-fix diagnostic artifacts:

- RV pre-fix raw log: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-current-retest-20260603T222025+0800.log`
- RV pre-fix summary: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-current-retest-20260603T222025+0800.summary.txt` — `1 PASS / 1 FAIL / TBROK=1`, not promotion evidence.

RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='mkdir09' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.log
```

LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='mkdir09' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.log target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.log
```

Targeted artifacts:

- RV log: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.summary.json`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-mkdir09-futex-bitset-fix-20260603T222513+0800.promotion-candidates.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.summary.json`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-mkdir09-futex-bitset-fix-20260603T222640+0800.promotion-candidates.txt`
- Combined RV+LA candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-mkdir09-futex-bitset-fix-promotion-candidates.txt`

Targeted parser result:

- RV summary: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA summary: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined RV+LA report: four-combo candidate `mkdir09`.

Adjacent regression command:

```bash
LTP_CASES='futex_wait01,futex_wait02,futex_wait03,futex_wait04,futex_wait05,futex_wake01,futex_wake03,clone01,clone03,clone06,clone07'
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$LTP_CASES" LTP_CASE_TIMEOUT_SECS=60 timeout 60m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-futex-bitset-adjacent-regression-20260603T222822+0800.log
OSCOMP_TEST_GROUPS=ltp LTP_CASES="$LTP_CASES" LTP_CASE_TIMEOUT_SECS=60 timeout 60m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-futex-bitset-adjacent-regression-20260603T222822+0800.log target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.log
```

Adjacent artifacts:

- Cases list: `target/ltp-1000-milestone-06-stable806/futex-bitset-adjacent-regression-20260603T223054+0800.cases`
- RV log: `target/ltp-1000-milestone-06-stable806/rv-futex-bitset-adjacent-regression-20260603T222822+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-futex-bitset-adjacent-regression-20260603T222822+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-futex-bitset-adjacent-regression-20260603T222822+0800.summary.json`
- LA log: `target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.summary.json`
- Combined adjacent candidate report: `target/ltp-1000-milestone-06-stable806/la-futex-bitset-adjacent-regression-20260603T223054+0800.combined-promotion-candidates.txt`

Adjacent parser result:

- RV adjacent summary: `22 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA adjacent summary: `22 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined adjacent report: all 11 adjacent cases are four-combo clean.

`mkdir09` is added to the stable806 candidate pool with four-combo clean evidence and an 11-case futex/clone adjacent stable subset on both architectures. The stable list remains `756 total / 756 unique / 0 duplicate`; no promotion commit is made because the candidate pool is only 12/50 for stable806.

## gettid02 futex/glibc follow-up targeted retest

The earlier credential/capability scout had only RV musl-clean `gettid02` evidence and a glibc `TBROK` caused by the same pthread/futex command gap later fixed for `mkdir09`. After the generic `FUTEX_WAIT_BITSET`/`FUTEX_WAKE_BITSET` support was committed, `gettid02` was rerun directly on both architectures. No additional source change was made for this follow-up.

RV command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='gettid02' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.log
```

LA command:

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES='gettid02' LTP_CASE_TIMEOUT_SECS=45 timeout 45m ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches la target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.log
python3 scripts/ltp_summary.py --promotion-candidates --promotion-arches rv,la target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.log target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.log
```

Artifacts:

- RV log: `target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.log`
- RV summary: `target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.summary.txt`
- RV JSON: `target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.summary.json`
- RV candidate report: `target/ltp-1000-milestone-06-stable806/rv-gettid02-after-futex-bitset-20260603T224424+0800.promotion-candidates.txt`
- LA log: `target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.log`
- LA summary: `target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.summary.txt`
- LA JSON: `target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.summary.json`
- LA candidate report: `target/ltp-1000-milestone-06-stable806/la-gettid02-after-futex-bitset-20260603T224549+0800.promotion-candidates.txt`
- Combined RV+LA candidate report: `target/ltp-1000-milestone-06-stable806/rv-la-gettid02-after-futex-bitset-20260603T224549+0800.promotion-candidates.txt`

Parser result:

- RV summary: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- LA summary: `2 PASS / 0 FAIL / 0 TFAIL/TBROK/TCONF / 0 timeout / 0 ENOSYS / 0 panic/trap`.
- Combined RV+LA report: four-combo candidate `gettid02`; `gettid02` is not already present in `LTP_STABLE_CASES`.

Regression boundary:

No new code was added for this follow-up. The underlying code change is the generic futex bitset support already protected by the RV/LA futex/clone adjacent regression above (`22 PASS / 0 FAIL` on each architecture). `gettid02` itself confirms that thread IDs remain distinct across pthread-created threads on RV and LA, musl and glibc.

`gettid02` is added to the stable806 candidate pool with four-combo clean evidence. The stable list remains `756 total / 756 unique / 0 duplicate`; no promotion commit is made because the candidate pool is only 13/50 for stable806.
