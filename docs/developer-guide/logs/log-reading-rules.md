# 日志读取规则

本页说明开发者应该如何读此前的 raw log、summary、远程输出和质量门禁文件。

## 1. 永远先用 parser

LTP 日志必须用 parser 读：

```bash
python3 scripts/ltp_summary.py raw.log > raw-summary.txt
python3 scripts/ltp_summary.py raw.log --json > raw-summary.json
python3 scripts/ltp_summary.py --promotion-candidates rv.log la.log
```

不要只看：

- `./run-eval.sh` 退出码；
- QEMU 是否退出；
- wrapper 最后一行；
- raw log 中某个局部 `TPASS`。

## 2. wrapper marker 的当前含义

当前 `examples/shell/src/cmd.rs::run_ltp_suite()` 对 status `0` 的
completed case 输出：

```text
PASS LTP CASE <case> : 0
```

非 0 wrapper status 仍输出 failure marker；timeout 会额外输出：

```text
FAIL LTP CASE <case> : <status>
```

```text
TIMEOUT LTP CASE <case> after <n>s
```

因此当前日志里出现 `PASS LTP CASE <case> : 0` 时，含义是“该 case
的 wrapper status 为 0”；仍必须交给 parser 结合内部
`TFAIL`/`TBROK`/`TCONF` 解释。

`scripts/ltp_summary.py` 以数字 status 为准，并兼容历史日志里的旧
marker：

```text
FAIL LTP CASE <case> : 0
```

开发者不要随意改 marker wire format；远程 scorer 和历史 parser 都依赖这层兼容。

## 3. clean promotion 的最低标准

一个 case 进入 `LTP_STABLE_CASES` 前，至少需要：

- RV + LA；
- `/musl` + `/glibc`；
- wrapper status 都为 0；
- 内部 `TFAIL=0`；
- 内部 `TBROK=0`；
- 没有未解释的新 `TCONF`；
- timeout/ENOSYS/panic/trap 为 0；
- 最好再由 aggregate stable gate 覆盖。

例外必须显式披露，例如当前已知 `read02` 的 O_DIRECT-on-tmpfs `TCONF`。

## 4. 哪些日志不能当 promotion 证据

| 日志类型 | 处理方式 |
| --- | --- |
| 用户中止的 QEMU/raw log | 标记为 incomplete / not evidence |
| 并发 QEMU 共享 `/tmp/arceos-sdcard-*.run.qcow2` | 标记为 untrusted，除非串行重跑 |
| 远程输出被 `...超过1MB的部分被截断...` 截断 | 只能说已捕获片段一致，不能说完整一致 |
| targeted 单架构 clean | 只能作为线索，不能推广 |
| targeted 双架构 clean 但 aggregate 失败 | 以 aggregate 失败为准，例如 `kill02` |
| 含 TFAIL/TBROK/timeout/ENOSYS/panic/trap | 不能当 clean promotion |
| setup TBROK | 不能 fake skip；先修真实 setup/环境语义 |

## 5. 如何读 summary 顶部

`*-summary.txt` 顶部通常包含：

```text
- Wrapper PASS (code 0): <n>
- Wrapper FAIL (nonzero/timeout): <n>
- Internal TFAIL/TBROK/TCONF: ...
- timeout matches: <n>
- ENOSYS/not implemented matches: <n>
- panic/trap matches: <n>

## Suite summaries
- ltp-musl: <passed> passed, <failed> failed
- ltp-glibc: <passed> passed, <failed> failed
```

开发者先看顶部摘要，再看 case matrix。不要反过来从海量 case matrix 中手工挑选局部成功。

## 6. marker-prefix 检查

远程 scorer 对 marker 位置敏感。关键 gate 需要确认 marker 从列 0 开始，`bad_marker_prefix=0`。

常见证据文件：

- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-a/raw/*marker-prefix*.txt`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-b/raw/stable375-final-marker-prefix.txt`
- `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-25-phase-c/remote-marker-and-log-noise-regression-check.md`

## 7. 日志噪声和 errno

`AxError::NotADirectory`、`IsADirectory`、`AlreadyExists` 等可能来自预期 negative-path errno。看到这类日志时不要直接判定 testcase 失败；先看 parser 语义结果。

已经完成的噪声修复原则：把 expected error path 从会打印 warning 的 `ax_err!` 改为直接 `Err(AxError::...)`，但不改变 syscall-visible errno。任何后续噪声修复也必须遵守：**只降噪，不把失败改成成功**。
