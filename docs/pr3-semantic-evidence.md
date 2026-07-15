# PR3：可信 semantic evidence 与双架构 CI

本文描述 OrayS PR3 的本地接口、机器可读证据模型和 CI gate。它的目标是准确说明
一次执行实际证明了什么，而不是把 syscall 注册、源码匹配或编译成功表述成 runtime
兼容。

## 证据模型

`test/suite_manifest.json` 是仓库唯一的 suite/profile 注册入口；PR3 的 collector 只是
其中四个 `evidence.*` case 的专用 adapter。`test/evidence/semantic_evidence_manifest.json`
是这些 case 内部的架构适用性、runner、capability 和 required/observational policy
事实源。manifest 与 canonical result 都使用 schema version 1；完整生成 schema 位于
`test/evidence/semantic_evidence_schema.v1.json`。PR3 测试逻辑不在 `scripts/` 中维护副本。

证据等级按强度递增：

| 等级 | 含义 |
| --- | --- |
| `declared` | 只发现声明或注册 |
| `static_checked` | 源码/合规守卫或 host unit test 已执行 |
| `built` | 固定命令成功，且本次新产物已绑定 size 和 SHA-256 |
| `booted` | guest/QEMU 已到达严格 boot marker |
| `runtime_semantic` | guest 中的 ABI-visible 行为通过严格 runtime 协议 |

结果状态是 `pass`、`fail`、`error`、`timeout`、`blocked`、`skipped`。缺工具或输入是
`blocked`/`error`，不是 pass；`TFAIL`、`TBROK`、`TCONF`、`ENOSYS`、timeout、panic、
trap、空日志、截断日志、parser exception 和 cleanup failure 都保持为明确 non-pass。
`blocked` 和 `skipped` 不得携带 observed evidence。
日志正规化只剥离不含文本 payload 的 SGR 颜色序列；OSC/DCS/SOS/PM/APC 以及其他
有状态/歧义终端控制一律产生 parser error，不能用控制 payload 隐藏 failure token。
不隶属于当前 LTP case 的 `Summary:` 或 summary field 也属于结构错误，不能被静默忽略。

每个结果保存 command、architecture、policy、目标/观察 evidence level、duration、
exit/signal/timeout/cleanup 信息、repo revision、工具身份、artifact size/SHA 和 raw log
引用。required full result 在 CI 中还要求 clean checkout，且 revision 必须等于当前
HEAD。

## 数据流与 artifact 布局

执行和渲染相互分离：

```text
versioned manifest
  -> strict validation and dependency selection
  -> bounded process supervisor
  -> raw logs + fresh artifacts
  -> fail-closed classification
  -> host / riscv64 / loongarch64 canonical JSON shards
  -> strict deterministic merge
  -> canonical semantic-evidence-v1.json
       -> semantic-evidence-v1.junit.xml
       -> semantic-evidence-v1.html
       -> semantic-matrix-v1.md
```

renderers 只消费 canonical JSON，不再次解释 raw logs。相同 JSON 的重复渲染必须
byte-identical。默认本地 bundle：

```text
build/pr3-evidence/
├── host/
│   ├── semantic-evidence-v1.json
│   └── logs/
├── rv64/
│   ├── artifacts/
│   ├── logs/
│   └── semantic-evidence-v1.json
├── la64/
│   ├── artifacts/
│   ├── logs/
│   └── semantic-evidence-v1.json
└── required/
    ├── artifacts/
    ├── logs/
    ├── semantic-evidence-v1.json
    └── reports/
        ├── semantic-evidence-v1.junit.xml
        ├── semantic-evidence-v1.html
        └── semantic-matrix-v1.md
```

HTML 不访问 CDN 或其他网络资源。报告不嵌入无界 console logs；log link 指向同一
bundle 中经过 size/SHA 验证的 raw evidence。partial result 只有显式
`--allow-partial` 才可渲染，并会显示 incomplete 警告；JUnit 同时增加 completeness
error，不能呈现为 required green。

## 本地命令

快速验证 canonical manifest、schema、全部静态守卫和 Python unit suite：

```bash
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
  --profile quick --output-dir test/output/pr3-quick
```

兼容目标 `make pr3-manifest-check` 和 `make pr3-infrastructure-tests` 也只委托给上述
canonical runner，不维护另一套 case 列表或 PASS 判定。

只收集 host guard/parser evidence：

```bash
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
  --profile evidence-host --output-dir test/output/pr3-evidence-host
```

使用当前 `PATH` 中的 QEMU 分别执行双架构固定构建和 runtime smoke；manifest 要求两者
的 `--version` 第一行精确为 `QEMU emulator version 9.2.4`。缺工具或版本不匹配会在
case 启动前产生 visible `blocked`，不能生成 required PASS：

```bash
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
  --profile evidence-runtime --arch rv --output-dir test/output/pr3-evidence-rv
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
  --profile evidence-runtime --arch la --output-dir test/output/pr3-evidence-la
```

运行 required evidence 前，先安装构建依赖，再构建经 SHA/size 校验的两个 QEMU 9.2.4
system emulator。normal build 只接受空 prefix；CI 下载的已有安装只能通过
`--verify-only` 验证 stamp、源码参数与二进制哈希。路径本身及其父路径不能经过符号
链接。长时间源码构建使用与 CI 相同的 supervisor，并保留日志：

```bash
sudo apt-get update
sudo apt-get install --yes --no-install-recommends \
  ninja-build pkg-config libglib2.0-dev libpixman-1-dev libfdt-dev zlib1g-dev
mkdir -p build/pr3-qemu-local
python3 test/evidence/semantic_evidence.py supervise \
  --timeout 4800 \
  --log build/pr3-qemu-local/setup.log \
  -- bash test/evidence/setup_qemu.sh \
    /tmp/orays-pr3-qemu-9.2.4 "$PWD/build/qemu-source"
export PATH="/tmp/orays-pr3-qemu-9.2.4/bin:$PATH"
```

若把该 prefix 打包后传到新的 Ubuntu 24.04 runner，consumer 必须先显式安装动态运行库，
不能假定 producer 的 `-dev` 包或 runner 偶然状态会随 artifact 传递：

```bash
sudo apt-get update
sudo apt-get install --yes --no-install-recommends \
  libfdt1 libglib2.0-0t64 libpixman-1-0 zlib1g
```

源码输入固定为 QEMU 9.2.4：

- URL：`https://download.qemu.org/qemu-9.2.4.tar.xz`
- size：`134782772` bytes
- SHA-256：`f3cc1c4eabfdb288218ac3e33763dbe9e276d8bc890b867a2335d58de2ddd39a`
- targets：`riscv64-softmmu,loongarch64-softmmu`

完整本地 required suite：

```bash
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
  --profile evidence-required --output-dir test/output/pr3-evidence-required
```

它依次产生三个 shard，严格合并全部 required instances，再生成三种报告。任何
required non-pass 都使命令非零，同时保留已经写出的 raw evidence。单独重渲染：

```bash
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
  --profile evidence-aggregate --output-dir test/output/pr3-evidence-aggregate
```

`make pr3-evidence-{host,rv,la,required}` 与 `make pr3-render-required` 是同一组 profile
的兼容别名；它们不直接实现测试或证据聚合逻辑。

直接使用 CLI 过滤 case/category/policy 时，先查看帮助；过滤结果是 partial evidence，
不能替代完整 required gate：

```bash
python3 test/evidence/semantic_evidence.py run --help
python3 test/evidence/semantic_evidence.py validate-result --help
python3 test/evidence/render_semantic_evidence.py --help
```

## Repository-contained runtime smoke

RV64 与 LA64 smoke 不依赖 competition SD card。`semantic-smoke` feature 构建一个
架构对应的静态 ELF；kernel harness 将它写入 RAMFS，并通过 `user/shell` 的现有 ELF
loader 和用户进程生命周期执行。ELF 真实调用 Linux ABI syscall：

| syscall | 编号 | 断言 |
| --- | ---: | --- |
| `write` | 64 | stdout 写入长度和 marker 正确 |
| `getpid` | 172 | 返回值大于 0 |
| `uname` | 160 | `sysname=Linux` 且 machine 为目标架构 |
| `exit` | 93 | 用户程序与 harness 状态均为 0 |

runner 要求唯一且有序的 harness start、user start、四个 assertion、user pass、harness
pass 和 shutdown marker。错误 marker、错架构、重复/缺失/乱序 marker、nonzero exit、
panic/trap/ENOSYS、timeout 或残留进程均非 pass。

监督器创建独立 process group；Linux 上启用 child subreaper，能够收养并清理尝试
`setsid()` 逃逸的后代。timeout/cancel/异常路径执行 TERM、限时等待、KILL、reap，并
验证没有存活后代。`make pr3-smoke-run-*-raw` 拒绝非监督调用。两条 smoke
不挂载磁盘或 user-mode network backend；为满足内核现有 `net` feature 的 NIC
探测，只连接 QEMU 内部 `hubport`，不会接入宿主网络。ABI 断言只依赖仓库内
RAMFS/ELF 和 QEMU machine/serial，因此固定 QEMU 构建不需要 `libslirp`。

该 smoke 只证明：本次构建的 kernel 在对应 QEMU 上启动，现有 loader 能执行此最小
ELF，且上述四个 ABI-visible syscall 语义满足断言。它不证明完整 LTP、完整 Linux
兼容、所有 syscall、official image compatibility 或 RV64/LA64 全面 parity。

## Required 与 observational CI

建议作为 branch protection required 的精确 check 名称如下：

- `Unit tests (required)`
- `PR3 infrastructure + host evidence (required)`
- `PR3 QEMU 9.2.4 source baseline (required)`
- `PR3 RV64 fixed build + runtime smoke (required)`
- `PR3 LA64 fixed build + runtime smoke (required)`
- `PR3 required aggregate`
- `Clippy (x86_64, fixed-required)`、`Clippy (riscv64, fixed-required)`、
  `Clippy (aarch64, fixed-required)`、`Clippy (loongarch64, fixed-required)`
- `Build (x86_64, fixed-required)`、`Build (riscv64, fixed-required)`、
  `Build (aarch64, fixed-required)`、`Build (loongarch64, fixed-required)`
- `Other platforms (fixed-required)` 与 `macOS (fixed-required)` 保留原有固定工具链
  hard-fail 覆盖；前者从仓库 `vendor/cargo/` 解析 platform crate，不再追踪远端 git
  HEAD，并以 `CARGO_HOME="$PWD/cargo-home" cargo add --offline --path` 只读取仓库已校验
  vendor，不依赖 clean runner 上不存在的 `cargo-axplat` 插件或 crates.io index；它仍是
  编译覆盖而不是 PR3 runtime semantic evidence。
- `Application tests (x86_64, fixed-required)`、
  `Application tests (riscv64, fixed-required)`、
  `Application tests (aarch64, fixed-required)`、
  `Application tests (loongarch64, fixed-required)` 保留既有 pinned-toolchain hard-fail
  覆盖；其外部 assets 不会被当成 repository-contained semantic evidence。
- `Docs (ubuntu-24.04)` 与 `Docs (macos-14)` 保留既有文档 hard-fail 覆盖。

`Deploy docs` 只在默认分支 push 上运行，不是 pull request required check，也不应加入
branch protection。

`PR3 required aggregate` 即使 producer 失败也尝试下载、合并、渲染和上传证据，最后
仍检查 producer conclusion；artifact upload 不会把失败改写为成功。artifact
`pr3-required-semantic-evidence` 包含 canonical JSON、raw logs/artifacts 和派生报告，
job summary 只给出 producer 状态及 artifact/run 链接。

RV64/LA64 runtime matrix 对 QEMU producer 使用 job-level `always()`，避免上游失败时
required check 被 GitHub 作为 skipped-success；job 末尾再显式断言 producer conclusion。
因此 QEMU 未构建、artifact 缺失或 producer 失败都不会让架构 required check 变绿。

以下精确 check 名称明确 observational，不应加入 branch protection：

- `Clippy (x86_64, moving-nightly-observational)`、
  `Clippy (riscv64, moving-nightly-observational)`、
  `Clippy (aarch64, moving-nightly-observational)`、
  `Clippy (loongarch64, moving-nightly-observational)`；
- `Build (x86_64, moving-nightly-observational)`、
  `Build (riscv64, moving-nightly-observational)`、
  `Build (aarch64, moving-nightly-observational)`、
  `Build (loongarch64, moving-nightly-observational)`；
- `Other platforms (moving-nightly-observational)`；
- `macOS (moving-nightly-observational)`。

需要 official SD card/judge 的 `run-eval.sh` full evaluation 当前不是 PR check；若以后
加入 CI，只能使用明确的 observational 名称，不能替代 repository-contained required
runtime smoke。

external app tests 仍是 hard-fail legacy coverage，但其 setup actions 内部下载的
QEMU/musl assets 尚未建立 PR3 semantic evidence 所需的完整 checksum provenance；
因此它们不进入 canonical aggregate，也不会替代 repository-contained runtime smoke。

## Official/full evaluator 边界

旧命令保持兼容：

```bash
./run-eval.sh rv
./run-eval.sh la
```

镜像解析优先级是架构专用环境变量、`TESTSUITE_DIR`，再到默认 sibling testsuite。
每次执行使用唯一 qcow2 overlay，并由同一个 supervisor 限时和清理。raw console、
strict protocol JSON/Markdown、LTP summary 和 failure report 保存在独立运行目录。

仓库不提交 official `sdcard-rv.img` 或 `sdcard-la.img`。缺镜像、QEMU 或 parser input
会返回非零并显示 blocked/error；不会生成空绿色报告。official judge directory 是可选
外部 provenance；显式请求却缺失时显示 N/A/error，而不是“0 failures”。

共享 `test/evidence/evaluator_protocol.py` 直接解析 raw bytes。它支持 official wrapper 的
`FAIL LTP CASE <case> : 0` 成功编码，但同时验证 group/case framing、唯一结果、case
count、suite summary、internal LTP Summary 和全日志 fatal token。`TCONF` 为 skipped，
不能进入 promotion pass。

`ltp_summary.py --promotion-candidates` 为兼容旧日志仍从文件名推断 `rv`/`la` 标签；
该标签是未认证的输入分类，不是 guest 架构或 runtime provenance。相同 raw SHA-256
若被绑定为不同架构会作为 integrity error fail-closed，但仅改写内容仍不能把这种旧式
报告提升为 required runtime evidence。PR3 的架构 required 结论只来自 manifest 绑定、
精确 QEMU 可执行文件和仓库内 guest smoke protocol 的 canonical shard。

guard 与 runtime marker 文本协议只接受 ASCII。分类器会保留原始字节作为证据，但会把
任何非 ASCII 协议日志判为 `malformed_log`，防止软连字符、BOM 等不可见字符拆分
`FAIL`、`panic` 或其他 fatal token。仅 SGR 颜色序列可在分类前安全移除；NUL/BEL
不会用于重构 PASS/marker，但 fatal 扫描会删除它们以防隐藏失败。

## 安全添加 case

1. 在 manifest 中按稳定 ID 排序添加 case；不要在 renderer 或 workflow 复制 case
   事实。
2. 选择不超过 runner `max_evidence` 的 target level。源码检查不能选择
   `runtime_semantic`。
3. required case 只能依赖 repository-contained capability；外部/私有输入必须
   observational。
4. 为 build case 声明新鲜 artifact；为 runtime case定义明确 success/fatal protocol。
5. 为 pass、失败、空/截断、重复/歧义和 evidence overclaim 增加小型 fixture tests。
6. 更新 generated schema（若结构变化），然后执行：

```bash
python3 test/evidence/semantic_evidence.py schema \
  --write test/evidence/semantic_evidence_schema.v1.json
python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py \
  --profile quick --output-dir test/output/pr3-quick
```

不要直接编辑 generated schema 或 generated reports。schema 的破坏性变更必须使用新
版本，不能静默重新解释旧 canonical PASS。

## 故障排查与已知限制

- `missing prerequisite`：安装日志指出的工具；required blocked 不会降级。
- `artifact_missing`：build 必须实际生成 declared path；`MAKEFLAGS=-n` 等危险环境会被
  清理，不能用 dry-run 制造 pass。
- `ambiguous_guard_protocol`：guard 必须只输出一个锚定 PASS，或在非零退出时输出明确
  FAIL；混合/重复 PASS 是 error。
- `fatal_runtime_signal`：检查 raw QEMU log；即使所有 success markers 已出现，marker
  后 panic/trap 仍为 error。
- `residual_process`/`cleanup_incomplete`：先保存 PID/cleanup diagnostics；不要只提高
  timeout 或隐藏失败。
- merge 拒绝 shard：确认三个 shard 来自同一 manifest、同一 commit，且没有重复 case
  或被修改的 log/artifact。
- CI full result 拒绝 dirty tree：QEMU prefix 必须放 `runner.temp`，generated evidence
  放 ignored `build/`；不要在运行中改 tracked source。collector 同时记录
  `run.repository_before` 与结束时 `repository`；两者不同会把所有已启动 case 标记为
  `repository_changed_during_run` error，merge 也拒绝该 shard。
- 工具身份的 `path` 是稳定的逻辑名，不包含开发机 home 绝对路径；`version` 与 `sha256`
  仍来自实际执行二进制。对 rustup proxy，版本通过原始 proxy 分派，SHA 来自
  `rustup which <tool>` 的实际工具；对仓库内 linker/objcopy shim，collector 通过受限的
  `--pr3-print-effective-tool` 协议解析并哈希实际工具。解析失败不会伪装成 wrapper/rustup
  身份，也不能支撑 required PASS。wrapper 自身仍由 repository content SHA 绑定。
- required QEMU capability 精确绑定 `QEMU emulator version 9.2.4`；其他版本会在执行前
  `blocked`，manifest-aware result validation 也会拒绝伪造的 passing shard。其他版本的
  本地兼容性探索不得写成 required canonical PASS。
- schema v1 将每个架构的最小 ABI smoke 记录为一个 scenario-level runtime case；
  `write/getpid/uname/exit` 的有序 assertion marker 与失败细节保存在 raw log/manifest 中，
  尚未拆成四个可单独聚合的 canonical row。matrix 因此只声称该完整场景通过，不外推为
  单个 syscall 的完整 Linux 兼容性。
- `make unittest_no_fail_fast` 的现有 axfs `test_devfs_ramfs()` `NotFound` 是 PR3 审计时
  已存在的失败。PR3 不改变其结论，也不通过 `continue-on-error` 隐藏；修复属于独立
  kernel/fs 工作。
- 固定基线上的 `cargo fmt --all -- --check` 仍会报告四个未被 PR3 修改的既有文件；
  `make clippy` 也存在 origin/main 可复现的既有 uspace 报错。两者在 workflow 中保持
  硬失败，不用 `continue-on-error` 隐藏；PR3 只要求其自身修改的 Rust 文件通过定向
  rustfmt，并把完整基线失败明确记录为 pre-existing，而不越界做全仓清理。
