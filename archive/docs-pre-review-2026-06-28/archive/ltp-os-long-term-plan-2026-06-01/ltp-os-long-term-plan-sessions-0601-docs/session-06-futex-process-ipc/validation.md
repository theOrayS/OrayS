# Session 6 validation

## Commands

```bash
# source/context lookup before promotion-only edit
# CodeGraph: sys_futex, wait/process, SysV shm/IPCs; no runtime code edited.

# RV clean21 gate
cases='futex_wait02,futex_wait04,futex_wake01,kill02,sched_tc2,sched_tc3,sched_tc4,sched_tc5,shmdt02,shmem_2nstest,shmnstest,shmt02,shmt03,shmt06,shmt07,shmt08,shmt10,tkill01,tkill02,vfork01,vfork02'
LTP_CASES="$cases" ./run-eval.sh rv > target/ltp-long-term-session6/session6-rv-clean21.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session6/session6-rv-clean21.log
python3 -B scripts/ltp_summary.py --json target/ltp-long-term-session6/session6-rv-clean21.log

# LA clean21 gate
LTP_CASES="$cases" ./run-eval.sh la > target/ltp-long-term-session6/session6-la-clean21.log 2>&1
python3 -B scripts/ltp_summary.py target/ltp-long-term-session6/session6-la-clean21.log
python3 -B scripts/ltp_summary.py --json target/ltp-long-term-session6/session6-la-clean21.log

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

# final build after stable list edit
make A=examples/shell ARCH=riscv64 > target/ltp-long-term-session6/session6-build-final-stable506.log 2>&1
```

长跑/QEMU/build 前后均按仓库规则检查 `df -h / /root`；记录值保持 `/dev/vda2 59G used 23G avail 34G use 41%`。raw log 留在 `target/ltp-long-term-session6/`，不提交。

## Parser-backed results

| Gate | Log | PASS | FAIL | Internal | timeout | ENOSYS | panic/trap | Conclusion |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | --- |
| RV clean21 | `target/ltp-long-term-session6/session6-rv-clean21.log` | 42 | 0 | `{}` | 0 | 0 | 0 | RV 21-case promotion gate clean |
| LA clean21 | `target/ltp-long-term-session6/session6-la-clean21.log` | 42 | 0 | `{}` | 0 | 0 | 0 | LA 21-case promotion gate clean |

## Checksums for retained local evidence

```text
f5b61fb5f643ba94546207b93b13ad733ebeef5fcdd0673bdd110f2f7b648e84  target/ltp-long-term-session6/session6-rv-clean21.log
9c1b9dccc118062eb49b13f4684cfdccf4099bebc75649b447633a4718a3c989  target/ltp-long-term-session6/session6-rv-clean21-summary.txt
0331f55d700d7447aba565b7b88fa3052e98504c7b4e46c075d7680c5175a7fb  target/ltp-long-term-session6/session6-rv-clean21-summary.json
f7cd9cb7ed422edebf9fdbe6dbac45a934b728a671b4fe7a06081564c79d9ad7  target/ltp-long-term-session6/session6-rv-clean21.status
c3b1d80b232a0aef6579687d6a72b7dcb351bd1e6196e8f86b4d105f40aa685f  target/ltp-long-term-session6/session6-la-clean21.log
c2a73c6ae0782ca5d5adb509b580289f28eb4389acfe6bbee8690929bfce01d0  target/ltp-long-term-session6/session6-la-clean21-summary.txt
90eaf51e973e4407ea03dbdcc6faf40059299ec252d4d2e4e2ed7fd7f965a90e  target/ltp-long-term-session6/session6-la-clean21-summary.json
1c885a4ab256e43311dd44a0e970d030cf854a864f52250ec6944c842a162057  target/ltp-long-term-session6/session6-la-clean21.status
6474163906f5295b08fd742eaaaa695ce52cc2d87ff3edebdc52e88ea3901883  target/ltp-long-term-session6/session6-build-final-stable506.log
```

## Stable count

After editing `LTP_STABLE_CASES`:

```text
506 506 0
```

## Not verified in this session

- 未运行完整 stable506 RV/LA × musl/glibc final gate；完整门禁保留给 Session 8。
- 未重新跑 `futex_wait03/futex_wait05/waitid07/clone02/execve01` 等 blocker。
- 未修改或验证 SysV sem/msg；本 session 的 IPC promotion 仅覆盖 SysV shm/shm namespace cases。

## Final local checks before commit

```bash
rustfmt examples/shell/src/cmd.rs
make A=examples/shell ARCH=riscv64 > target/ltp-long-term-session6/session6-build-final-stable506.log 2>&1
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
```

Results:

- `make A=examples/shell ARCH=riscv64`: exit `0`; only existing dependency/`axnet` warnings remained.
- live stable count: `506 506 0`.
- `git diff --check`: no output / exit `0`.
