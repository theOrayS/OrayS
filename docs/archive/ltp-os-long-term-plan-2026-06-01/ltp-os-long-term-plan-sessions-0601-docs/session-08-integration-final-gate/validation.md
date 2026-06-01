# Session 8 validation

## Commands

```bash
# live stable count
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY

# RV final gate
{
  echo '## df before RV stable506 final gate'
  df -h / /root
  echo '## stable count'
  cat target/ltp-long-term-session8/session8-stable-count-before.txt
} > target/ltp-long-term-session8/session8-rv-stable506.preflight.txt
OSCOMP_TEST_GROUPS="ltp" LTP_CASES="stable" ./run-eval.sh rv > target/ltp-long-term-session8/session8-rv-stable506.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session8/session8-rv-stable506.log
python3 -B scripts/ltp_summary.py --json target/ltp-long-term-session8/session8-rv-stable506.log

# LA final gate
{
  echo '## df before LA stable506 final gate'
  df -h / /root
  echo '## stable count'
  cat target/ltp-long-term-session8/session8-stable-count-before.txt
} > target/ltp-long-term-session8/session8-la-stable506.preflight.txt
OSCOMP_TEST_GROUPS="ltp" LTP_CASES="stable" ./run-eval.sh la > target/ltp-long-term-session8/session8-la-stable506.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session8/session8-la-stable506.log
python3 -B scripts/ltp_summary.py --json target/ltp-long-term-session8/session8-la-stable506.log

# marker-prefix audit and final diff check
python3 - <<'PY'
from pathlib import Path
for path in ['target/ltp-long-term-session8/session8-rv-stable506.log','target/ltp-long-term-session8/session8-la-stable506.log']:
    bad=[]
    for i,line in enumerate(Path(path).read_text(errors='ignore').splitlines(),1):
        if 'LTP CASE' in line and not (line.startswith('RUN LTP CASE ') or line.startswith('PASS LTP CASE ') or line.startswith('FAIL LTP CASE ') or line.startswith('LTP CASE RUNTIME ')):
            bad.append((i,line[:220]))
    print(path, 'non_prefix_ltp_case_lines', len(bad))
PY
git diff --check
```

## Parser-backed final gate results

| Arch | Log | run_status | PASS | FAIL | Suite summaries | Internal | timeout | ENOSYS | panic/trap | Conclusion |
| --- | --- | ---: | ---: | ---: | --- | --- | ---: | ---: | ---: | --- |
| RV | `target/ltp-long-term-session8/session8-rv-stable506.log` | 0 | 1012 | 0 | `ltp-musl 506/0`, `ltp-glibc 506/0` | `{'TCONF': 4}` from inherited `read02` only | 0 | 0 | 0 | final gate accepted with known caveat |
| LA | `target/ltp-long-term-session8/session8-la-stable506.log` | 0 | 1012 | 0 | `ltp-musl 506/0`, `ltp-glibc 506/0` | `{'TCONF': 4}` from inherited `read02` only | 0 | 0 | 0 | final gate accepted with known caveat |

Known caveat detail:

- RV `pass_with_tconf`: `rv:glibc:read02`, `rv:musl:read02`
- LA `pass_with_tconf`: `la:glibc:read02`, `la:musl:read02`

## Disk checks

All pre/post QEMU checks reported:

```text
/dev/vda2 59G used 23G avail 34G use 41% /
```

## Marker-prefix audit

```text
target/ltp-long-term-session8/session8-rv-stable506.log non_prefix_ltp_case_lines 0
target/ltp-long-term-session8/session8-la-stable506.log non_prefix_ltp_case_lines 0
```

## Checksums for retained local evidence

```text
82e127d81af686b81c8f9485e2fb6f4e18aa30bf9bb8f42d6b2ff67e6382bb65  target/ltp-long-term-session8/session8-stable-count-before.txt
8d0348ffedf3abeabe9a6bc9486814e47519e564a9848ba58ca12c47b9888a70  target/ltp-long-term-session8/session8-rv-stable506.preflight.txt
f5813bcf3f6d53a65305cc198ddc3ac5ff29844813dee2a5a0a1688cb5303ee3  target/ltp-long-term-session8/session8-rv-stable506.log
ad886ce930000fb97d78260e18ee0f187c5452ec8e7ebdece8f98e43e1c1d7b3  target/ltp-long-term-session8/session8-rv-stable506-summary.txt
6767a7ec07e78e4f79432aab23895eb65c6e53a71e99665c8ce5f4dbd06cf9ac  target/ltp-long-term-session8/session8-rv-stable506-summary.json
ac5174c240584171f05abaf9c7d3a4d7408942b72122481e57b4e009e4a59f6b  target/ltp-long-term-session8/session8-rv-stable506.status
82e18e49b4ad49aa9a03bceed65cd04f73ce526f26269b96d588e5a90d7e2b97  target/ltp-long-term-session8/session8-la-stable506.preflight.txt
02989e4f344d1530d464975b7a3da7208a2201102c2f56d257ab450f53284818  target/ltp-long-term-session8/session8-la-stable506.log
b31cd817141adeb8a30fa0b5ef63438f79deed79beb58fb9f5a7c600131705c0  target/ltp-long-term-session8/session8-la-stable506-summary.txt
4fad6b374d1dfc84bc412f00524d5007eb75dd5fb0eeb68e087168e30a06332f  target/ltp-long-term-session8/session8-la-stable506-summary.json
388b85b5d5877bbb89fc256b43ad7036cc94511db57cd3a41524e2f7ddbb50e3  target/ltp-long-term-session8/session8-la-stable506.status
```

## Unverified items

- Optional Session 9/10 未执行：没有新的 network/proc/syntheticfs 修复，也没有 all-minus-blacklist/full-sweep 再闭合。
- 未冲 stable520；stable506 是本轮最高可信值。
