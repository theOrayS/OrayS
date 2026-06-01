# Session 5 validation

## Commands

```bash
# live stable count before/after promotion
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY

# build after mincore implementation
make A=examples/shell ARCH=riscv64 > target/ltp-long-term-session5/session5-build-mincore.log 2>&1

# RV scout/postfix/final and LA final gates; all summaries parsed with scripts/ltp_summary.py
python3 -B scripts/ltp_summary.py target/ltp-long-term-session5/session5-rv-mm-scout.log
python3 -B scripts/ltp_summary.py target/ltp-long-term-session5/session5-rv-mincore-postfix.log
python3 -B scripts/ltp_summary.py target/ltp-long-term-session5/session5-rv-mm-final-combined.log
python3 -B scripts/ltp_summary.py target/ltp-long-term-session5/session5-la-mm-final-combined.log

# final build after stable list promotion
make A=examples/shell ARCH=riscv64 > target/ltp-long-term-session5/session5-build-final-stable485.log 2>&1
```

长跑/QEMU 前后均按仓库规则检查 `df -h / /root`；final build log 记录 before/after 均为 `/dev/vda2 59G used 23G avail 34G use 41%`。raw log 留在 `target/ltp-long-term-session5/`，不提交。

## Parser-backed results

| Gate | Log | PASS | FAIL | Internal | timeout | ENOSYS | panic/trap | Conclusion |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| RV initial mmap/mm scout | `target/ltp-long-term-session5/session5-rv-mm-scout.log` | 20 | 8 | `{'TCONF': 4, 'TFAIL': 18, 'TBROK': 4}` | 0 | 8 | 0 | 只作候选分类，不作 promotion |
| RV `mincore01` postfix | `target/ltp-long-term-session5/session5-rv-mincore-postfix.log` | 2 | 0 | `{}` | 0 | 0 | 0 | `mincore01` RV musl/glibc clean |
| RV final combined | `target/ltp-long-term-session5/session5-rv-mm-final-combined.log` | 44 | 0 | `{}` | 0 | 0 | 0 | RV 22-case promotion+相邻回归 clean |
| LA final combined | `target/ltp-long-term-session5/session5-la-mm-final-combined.log` | 44 | 0 | `{}` | 0 | 0 | 0 | LA 22-case promotion+相邻回归 clean |

## Checksums for retained local evidence

```text
37abfbc670017a35fdc8c5a9b652f27db8334f287c0f8808eccd3674f1300898  target/ltp-long-term-session5/session5-build-mincore.log
4a28a03b97ce073b10593190b050f7df06cb986ac757173d10300e2eaf0b2525  target/ltp-long-term-session5/session5-build-final-stable485.log
f79173fbaeca3c05f2deee7ac73343b3747b12a6be3bef645cade68d0ba24d62  target/ltp-long-term-session5/session5-rv-mm-scout.log
9e2e00588396820e9159a169b7f447598ffb99b5a58c3d3ece7ee214cd062e6d  target/ltp-long-term-session5/session5-rv-mm-scout-summary.txt
d567d8ed52b4cdc3d72a768827094336d6716ba0daeffc3386de5cc7b0efd04e  target/ltp-long-term-session5/session5-rv-mm-scout-summary.json
1a0777f1a7798af2a595ab82455fec9785de94b0e0cb31e0a5d1cc878c9782cf  target/ltp-long-term-session5/session5-rv-mincore-postfix.log
7893c8e719b1edcc16523bab7d01f594debab1f8b4821c6796824c4b19ab94ed  target/ltp-long-term-session5/session5-rv-mincore-postfix-summary.txt
cc34902b99745db7f50b798377dae7cd910002fc0e54d9749e837c797b89b9a7  target/ltp-long-term-session5/session5-rv-mincore-postfix-summary.json
3e4e72c9eeb45fcb65a4d38bf9b29dbfb1116ad807809f17dcb8c2c866907e4f  target/ltp-long-term-session5/session5-rv-mm-final-combined.log
1390d63aa0f765fe20c7ba1eaf084b989b8efe1cdc1c0fd7361795f9d47ae3cc  target/ltp-long-term-session5/session5-rv-mm-final-combined-summary.txt
a30c4847f37c0a65b2bf6f324a135f743dbf0bdee128f85d1cac92064387c841  target/ltp-long-term-session5/session5-rv-mm-final-combined-summary.json
d548898dd73841f2e6e118f4d304f317c58e1323e0e3cd02b66ea7b614382407  target/ltp-long-term-session5/session5-la-mm-final-combined.log
b433f7151d86c9c53a7a0c0a27cc39f97aceb8cf2cafbb7f106953ca9c41cf19  target/ltp-long-term-session5/session5-la-mm-final-combined-summary.txt
5f8a8b6401cf16fdd506eabb91fbf0e50dfabb966510f4fd010d7cbf35408607  target/ltp-long-term-session5/session5-la-mm-final-combined-summary.json
```

## Stable count

After editing `LTP_STABLE_CASES`:

```text
485 485 0
```

## Not verified in this session

- 未运行完整 stable485 RV/LA × musl/glibc final gate；完整门禁保留给 Session 8。
- 未闭合 `diotest4` 的 non-existent user-buffer read/write 失败。
- 未闭合 `mprotect01` 的 `ENOMEM/EACCES` 边界或 `mprotect02` 的 SIGSEGV handler 恢复语义。
- 未重新跑 full sweep 或 LA blacklist-removal shard。

## Final local checks before commit

```bash
rustfmt examples/shell/src/cmd.rs examples/shell/src/uspace/memory_map.rs examples/shell/src/uspace/syscall_dispatch.rs
make A=examples/shell ARCH=riscv64 > target/ltp-long-term-session5/session5-build-final-stable485.log 2>&1
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY
git diff --check
rg -n 'diotest1|diotest2|diotest3|diotest4|diotest5|diotest6|mprotect05|mmap001|mmap15|mmap17|mmap19|mincore01|mprotect01|mprotect02' examples/shell/src/uspace || true
```

Results:

- `make A=examples/shell ARCH=riscv64`: exit `0`; only existing dependency/`axnet` warnings remained.
- final build log checksum: `4a28a03b97ce073b10593190b050f7df06cb986ac757173d10300e2eaf0b2525  target/ltp-long-term-session5/session5-build-final-stable485.log`.
- live stable count: `485 485 0`.
- `git diff --check`: no output / exit `0`.
- guardrail scan over changed runtime files for LTP case names/output markers: no matches.
