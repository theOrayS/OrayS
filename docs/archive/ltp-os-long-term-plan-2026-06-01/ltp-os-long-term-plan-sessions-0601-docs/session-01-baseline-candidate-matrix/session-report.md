# Session 1 Report — Baseline Candidate Matrix

Commit: pending; final SHA to be recorded by leader after commit.

## 目标

冻结 live stable460 基线，把 rv-arch002 / la-arch012 full-sweep summary/raw log 转成可执行候选矩阵，并选出第一批 20~40 个 targeted cases。

## 改动

- 新增 `candidate-matrix-stable460-to-500plus.md`：按 lane 汇总 clean-not-stable、第一批 targeted cases 和后续 blocked family highlights。
- 新增 `targeted-cases.txt` / `targeted-cases.md`：40 个 case，单位均为 LTP case。
- 新增 `parser-derived-summary.json`：记录 live stable count、raw log sha256、summary counts、promotion-candidate counts 和 Team fallback note。
- 新增 `clean-candidates-not-in-stable460.txt`：full-sweep 四路 clean 但尚未在 stable460 中的 106 个 case。
- 新增 `validation.md` 与 `no-promotion-reason.md`。

## 证据

- live stable count：`460 total / 460 unique / 0 duplicate`。
- RV summary：`docs/ltp-full-sweep-blacklist-2026-05-30-arch/summaries/rv-arch002-summary.json`，closed=`true`，wrapper PASS `1204` / FAIL `3453` / TIMEOUT `55`。
- LA summary：`docs/ltp-full-sweep-blacklist-2026-05-30-arch/summaries/la-arch012-summary.json`，closed=`true`，wrapper PASS `1207` / FAIL `2698` / TIMEOUT `53`。
- Cross RV/LA × musl/glibc parser：`563` full-sweep clean candidates，`106` 个不在 stable460。
- marker audit：RV/LA `incomplete=0`、`resource_failure=0`、`panic=0`、`trap-like=0`。

## 结论

Session 1 已完成。下一步 Session 2 优先处理：`getitimer01`、`ppoll01`、`select02`，并把 `clock_gettime04`、`clock_nanosleep02`、`nanosleep01`、`poll02`、`pselect01`、`pselect01_64`、`settimeofday01`、`time-schedule` 作为 fresh targeted gate 候选。所有 promotion 必须等 targeted RV/LA × musl/glibc parser-clean 后再做。

## 风险

- full-sweep clean 不是 promotion proof；该 session 没有运行新的 targeted gate。
- `select02` 当前四路 timeout/status=137；不能加入 stable，需 Session 2 分类 timeout/剩余时间/EINTR 行为。
- `getitimer01` 四路 `TFAIL` 且伴随 ENOSYS 文本，优先检查 getitimer/setitimer 最小真实模型。
- Team runtime 因无关 dirty worktree 被拒绝；未清理无关文件，避免污染用户/他人改动。

## 下一步入口

Session 2 文档目录：`docs/ltp-os-long-term-plan-sessions-0601-docs/session-02-time-select-signal/`。从 `targeted-cases.txt` 取 time/select/signal 小批，先 RV targeted，再 LA 复核；若任何内部 `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap` 存在，只能报告分类，不能 promotion。
