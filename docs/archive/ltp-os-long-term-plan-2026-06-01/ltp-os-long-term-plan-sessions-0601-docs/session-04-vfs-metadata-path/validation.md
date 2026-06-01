# Session 4 validation

## Commands

```bash
# live stable count before promotion
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY

# source-informed diagnosis helpers under target/ only; raw LTP source is not committed
curl -L .../statx03.c
curl -L .../getxattr01.c
curl -L .../listxattr01.c
curl -L .../getdents01.c
curl -L .../readlink03.c

# build after VFS/xattr/getdents/readlink code edits
make A=examples/shell ARCH=riscv64 > target/ltp-long-term-session4/session4-build-vfs-xattr-getdents.log 2>&1

# RV scout/postfix/final and LA final gates; all summaries parsed with scripts/ltp_summary.py
python3 -B scripts/ltp_summary.py target/ltp-long-term-session4/session4-rv-vfs-scout.log
python3 -B scripts/ltp_summary.py target/ltp-long-term-session4/session4-rv-vfs-postfix1.log
python3 -B scripts/ltp_summary.py target/ltp-long-term-session4/session4-rv-vfs-final-combined.log
python3 -B scripts/ltp_summary.py target/ltp-long-term-session4/session4-la-vfs-final-combined.log
python3 -B scripts/ltp_summary.py target/ltp-long-term-session4/session4-la-vfs-final-promotion-clean.log

# live stable count after promotion
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

长跑/QEMU 前后均按仓库规则检查 `df -h / /root`；raw log 留在 `target/ltp-long-term-session4/`，不提交。

## Parser-backed results

| Gate | Log | PASS | FAIL | Internal | timeout | ENOSYS | panic/trap | Conclusion |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| RV initial scout | `target/ltp-long-term-session4/session4-rv-vfs-scout.log` | 14 | 16 | `{'TBROK': 10, 'TFAIL': 15, 'TCONF': 6}` | 0 | 6 | 0 | 只作候选分类，不作 promotion |
| RV postfix | `target/ltp-long-term-session4/session4-rv-vfs-postfix1.log` | 18 | 4 | `{'TCONF': 3, 'TFAIL': 9}` | 0 | 1 | 0 | `getxattr01/listxattr01/statx03` 修复；`getdents01/readlink03` 不推广 |
| RV final combined | `target/ltp-long-term-session4/session4-rv-vfs-final-combined.log` | 70 | 0 | `{}` | 0 | 0 | 0 | RV 35-case promotion+相邻回归 clean |
| LA final combined | `target/ltp-long-term-session4/session4-la-vfs-final-combined.log` | 69 | 1 | `{'TFAIL': 1}` | 0 | 0 | 0 | 唯一失败 `ltp-musl:readlinkat02`；不推广 `readlinkat02` |
| LA promotion-clean | `target/ltp-long-term-session4/session4-la-vfs-final-promotion-clean.log` | 68 | 0 | `{}` | 0 | 0 | 0 | LA 34-case promotion+相邻回归 clean |

## Checksums for retained local evidence

```text
3dc31962f2c6964d969cebe2f0549c9f112288b04c7fb5024f9f20f9406a12c7  target/ltp-long-term-session4/session4-rv-vfs-final-combined.log
6d575e9c241803949b6f13d3e55457e1b8cc1a0a6b5e9134901f1c77aa87aae8  target/ltp-long-term-session4/session4-rv-vfs-final-combined-summary.txt
11238786982b5aaf6f9b2db47cd7080f627bd3f489081f4e043b9546cb673605  target/ltp-long-term-session4/session4-rv-vfs-final-combined-summary.json
a8e5b8405370dc78b159a945b67f3a8c72163c9731b02ccb62e28542b4a01083  target/ltp-long-term-session4/session4-la-vfs-final-promotion-clean.log
8eb9ac8352935342108bc9a3f14569c14c4006edbf8b3a3a4f6532c3eb0e2e72  target/ltp-long-term-session4/session4-la-vfs-final-promotion-clean-summary.txt
b659c18850788acc0f6f5797ce6421c6b622edf55abf457837e68a40a90daa5a  target/ltp-long-term-session4/session4-la-vfs-final-promotion-clean-summary.json
c34ac002378ce99be861676b6e4bdc64ab6b887bc335dd9d036f4dabe8686145  target/ltp-long-term-session4/session4-la-vfs-final-combined.log
5d755664c5e5ef525e93a8f3766dd4a35ebd594bac85dc2fea16820fe9700479  target/ltp-long-term-session4/session4-la-vfs-final-combined-summary.txt
65b292b18c240453e7b6c75def5f279941707e6af3b69fa08a365bcbbb761f19  target/ltp-long-term-session4/session4-la-vfs-final-combined-summary.json
```

## Stable count

After editing `LTP_STABLE_CASES`:

```text
474 474 0
```

## Not verified in this session

- 未运行完整 stable474 RV/LA × musl/glibc final gate；完整门禁保留给 Session 8。
- 未实现/验证持久化、跨全局文件系统的完整 xattr 语义。
- 未闭合 `readlink03` 的组件级 ELOOP 和 LA musl `readlinkat02` 失败。
- 未实现 legacy `getdents` syscall；`getdents01/getdents02` 不作为 promotion 证据。

## Final local checks before commit

```bash
rustfmt examples/shell/src/cmd.rs examples/shell/src/uspace/metadata.rs examples/shell/src/uspace/fd_table.rs examples/shell/src/uspace/syscall_dispatch.rs examples/shell/src/uspace/mod.rs examples/shell/src/uspace/process_lifecycle.rs
make A=examples/shell ARCH=riscv64 > target/ltp-long-term-session4/session4-build-final-stable474.log 2>&1
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
rg -n 'getxattr01|listxattr01|statx03|fpathconf01|pathconf01|rename14|mknod08|mknodat01|readlinkat02|getdents01|readlink03' examples/shell/src/uspace || true
```

Results:

- `make A=examples/shell ARCH=riscv64`: exit `0`; only existing `axnet` dead-code warning remained.
- final build log checksum: `3359ce84229a8d24f91f4f197c3d3ee2dda510549f39464246bfc164c2778222  target/ltp-long-term-session4/session4-build-final-stable474.log`.
- live stable count: `474 474 0`.
- `git diff --check`: no output / exit `0`.
- guardrail scan over changed runtime files for LTP case names/output markers: no matches.
