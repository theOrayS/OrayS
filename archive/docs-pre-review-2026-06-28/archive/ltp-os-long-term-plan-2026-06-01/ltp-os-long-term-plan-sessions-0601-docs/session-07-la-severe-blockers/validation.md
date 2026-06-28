# Session 7 validation

## Commands

```bash
# active blacklist counts before/after edit
python3 - <<'PY'
from pathlib import Path
for p in ['blacklist-la.txt', 'blacklist-rv.txt', 'blacklist-common.txt']:
    path = Path('docs/ltp-full-sweep-blacklist-2026-05-30-arch') / p
    cases = [line.strip() for line in path.read_text().splitlines() if line.strip() and not line.lstrip().startswith('#')]
    print(p, len(cases))
PY

# LTP-only targeted checks; earlier non-LTP-only 240s attempts are not promotion/removal evidence.
OSCOMP_TEST_GROUPS="ltp" LTP_CASES="creat07" ./run-eval.sh la > target/ltp-long-term-session7/session7-la-creat07-ltp-only.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session7/session7-la-creat07-ltp-only.log
python3 -B scripts/ltp_summary.py --json target/ltp-long-term-session7/session7-la-creat07-ltp-only.log

OSCOMP_TEST_GROUPS="ltp" LTP_CASES="tcp4-uni-basic01" ./run-eval.sh la > target/ltp-long-term-session7/session7-la-tcp4-uni-basic01-ltp-only.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session7/session7-la-tcp4-uni-basic01-ltp-only.log
python3 -B scripts/ltp_summary.py --json target/ltp-long-term-session7/session7-la-tcp4-uni-basic01-ltp-only.log

OSCOMP_TEST_GROUPS="ltp" LTP_CASES="creat07,tcp4-uni-basic01" ./run-eval.sh la > target/ltp-long-term-session7/session7-la-removal-shard.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session7/session7-la-removal-shard.log
python3 -B scripts/ltp_summary.py --json target/ltp-long-term-session7/session7-la-removal-shard.log
```

长跑/QEMU 前后均按仓库规则检查 `df -h / /root`；记录值保持 `/dev/vda2 59G used 23G avail 34G use 41%`。raw log 留在 `target/ltp-long-term-session7/`，不提交。

## Parser-backed results

| Gate | Log | PASS | FAIL | Internal | timeout | ENOSYS | panic/trap | Conclusion |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| LA `creat07` LTP-only | `target/ltp-long-term-session7/session7-la-creat07-ltp-only.log` | 0 | 2 | `{'TBROK': 2}` | 0 | 0 | 0 | 正常闭合为普通 FAIL/TBROK；可从 severe blacklist 移除，但不算 PASS |
| LA `tcp4-uni-basic01` LTP-only | `target/ltp-long-term-session7/session7-la-tcp4-uni-basic01-ltp-only.log` | 0 | 2 | `{'TCONF': 2}` | 0 | 0 | 0 | 正常闭合为普通 FAIL/TCONF；可单项移除，但不代表 network family 全部解除 |
| LA removal shard | `target/ltp-long-term-session7/session7-la-removal-shard.log` | 0 | 4 | `{'TBROK': 2, 'TCONF': 2}` | 0 | 0 | 0 | 两个移除项一起运行仍闭合，无 severe marker |

## Checksums for retained local evidence

```text
f7e14b493e0e5e706a2207392fe59559bc35b7571cac2410356fdc079ee5c1b7  target/ltp-long-term-session7/session7-la-creat07-ltp-only.log
09f1902af47dc87b3ee1655c72d52286db21d56ad00c31e483c88e7df7675583  target/ltp-long-term-session7/session7-la-creat07-ltp-only-summary.txt
feca8c55f11b045557db6d7bd08921514bfe6d0d257f389ae465c3a9fc03580b  target/ltp-long-term-session7/session7-la-creat07-ltp-only-summary.json
42d73eb19d1ea148686816828ba24227cb5d3e427fdd94ae1eb993d9de6b3614  target/ltp-long-term-session7/session7-la-creat07-ltp-only.status
7b6231415800cb94ae25206ff4b3cee6f18b044f166b1df6da0764245dccbedf  target/ltp-long-term-session7/session7-la-tcp4-uni-basic01-ltp-only.log
0d20b09e51f3a41048a9ff38ece458fe48f47dafa367890d04b97e6b44a8f166  target/ltp-long-term-session7/session7-la-tcp4-uni-basic01-ltp-only-summary.txt
4b4a113b09b7b436f6ba6a53c180d55b3be6250f42132424dba3c3d7a5ebb049  target/ltp-long-term-session7/session7-la-tcp4-uni-basic01-ltp-only-summary.json
03163f3eab065f6edfb6b72701211dcdf65f7024fc76a6fea7912656829c1eb5  target/ltp-long-term-session7/session7-la-tcp4-uni-basic01-ltp-only.status
3e0deff4dfa22aeb98ae530b76bef133f300910285950445ddf3f0676741041c  target/ltp-long-term-session7/session7-la-removal-shard.log
ef59df265590b99fb4b0c1243462f677f5cab76229a21b8f988ed040368a9581  target/ltp-long-term-session7/session7-la-removal-shard-summary.txt
73658bfcd2a5d8da9c9b0f5a5c94d94f5be9001317bf1a88cdc9923399fd402e  target/ltp-long-term-session7/session7-la-removal-shard-summary.json
4a951c35cb675c26bbf2371d150d6b6862c95833bde59f9df563a93e62fc1c18  target/ltp-long-term-session7/session7-la-removal-shard.status
```

## Not used as removal evidence

The following exploratory logs timed out before reaching the LTP group because `OSCOMP_TEST_GROUPS=ltp` was not set. They are retained locally only as discarded attempts and are not cited as proof:

```text
target/ltp-long-term-session7/session7-la-creat07.log
target/ltp-long-term-session7/session7-la-tcp4-uni-basic01.log
```

## Final local checks before commit

```bash
python3 - <<'PY'
from pathlib import Path
for p in ['blacklist-la.txt', 'blacklist-rv.txt', 'blacklist-common.txt']:
    path = Path('docs/ltp-full-sweep-blacklist-2026-05-30-arch') / p
    cases = [line.strip() for line in path.read_text().splitlines() if line.strip() and not line.lstrip().startswith('#')]
    print(p, len(cases), 'creat07' in cases, 'tcp4-uni-basic01' in cases)
PY
git diff --check
```

Results:

```text
blacklist-la.txt 374 False False
blacklist-rv.txt 1 False False
blacklist-common.txt 5 False False
```

- `git diff --check`: no output / exit `0`.
