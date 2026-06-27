# exp/ltp-full-sweep-blacklist：全量 LTP + 黑名单实验入口

日期：2026-05-29
分支：`exp/ltp-full-sweep-blacklist`

## 目标

本分支把 LTP runner 从单一 `stable` 白名单扩展为双轨：

- `stable`：默认路径，继续使用 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`，作为可信稳定门禁；
- `all` / `blacklist`：实验路径，枚举当前 libc 套件下 `ltp/testcases/bin` 里的可执行文件，学习 A20OS 的“全量扫 + 黑名单”冲分策略。

## 新增选择模式

`LTP_CASES` 现在支持：

```text
stable                 # 默认；不改变现有稳定门禁
core                   # 原核心集
batch:<name>           # 原静态 batch
file:<path>            # 原外部 case list
all 或 sweep:all       # 枚举 target_dir 下全部非 .sh 文件
blacklist              # all-minus-blacklist
all-minus-blacklist    # blacklist 的别名
sweep:blacklist        # blacklist 的别名
```

`blacklist` 模式会先从 `target_dir` 枚举候选，然后剔除：

1. 内建高风险默认黑名单 `LTP_SWEEP_DEFAULT_BLACKLIST_CASES`；
2. 编译期 `LTP_BLACKLIST` 里额外列出的 case；
3. 运行时 `/ltp_blacklist.txt` 或 `/tmp/ltp_blacklist.txt` 中额外列出的 case。

黑名单文件格式复用已有 case-list 解析：空白/逗号分隔，`#` 后注释。

## 使用示例

构建实验 sweep 内核：

```bash
LTP_CASES=blacklist make kernel-rv
LTP_CASES=blacklist make kernel-la
```

更激进的全量无黑名单枚举仅用于发现，不建议直接提交：

```bash
LTP_CASES=all make kernel-rv
```

增加临时黑名单：

```bash
LTP_CASES=blacklist LTP_BLACKLIST='fork13 rename14 mmapstress01' make kernel-rv
```

## 闸门语义

- 本分支不修改 `LTP_STABLE_CASES`；
- 默认无 `LTP_CASES` 时仍为 `stable`；
- `blacklist` 只是实验/冲分枚举策略，不能作为 stable promotion proof；
- 真实结果仍必须用 `scripts/ltp_summary.py` 解析 timeout、TCONF/TFAIL/TBROK、ENOSYS、panic/trap；
- 禁止把黑名单当作隐藏回归的手段：已稳定通过的 case 不应因为失败被加入默认黑名单。

## 后续建议

1. 先用 `LTP_CASES=blacklist` 跑 RV 单路，生成第一版失败矩阵；
2. 将 panic/trap、明显 fork bomb、mem/cgroup 压测放入外部 `/ltp_blacklist.txt` 或 `LTP_BLACKLIST`；
3. 对剩余失败按 syscall/FS/process/signal/futex/net 等高扇出子系统排序；
4. 只有经过 RV/LA × musl/glibc targeted gate 的 case 才能继续进入 `LTP_STABLE_CASES`。
