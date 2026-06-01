# Phase 0 report: LA severe blockers and full-sweep quality lane

Worker: `worker-5`
Task: `7` / report-only source diagnosis
Date: 2026-06-01
Scope: no QEMU run, no stable promotion, no Ultragoal checkpoint, no source or blacklist edit.

## Executive summary

- Live stable baseline in this worktree is `506 total / 506 unique / 0 duplicate`, re-read from `examples/shell/src/cmd.rs::LTP_STABLE_CASES`.
- Current supplemental blacklist files are `common=5`, `rv_only=1`, `la_only=374`; with the source built-in default sweep blacklist (`38` unique cases), effective blacklist-consuming runs have `rv_active=44` and `la_active=417` unique excluded case names before target-dir filtering.
- The archived pure blacklist/full-sweep evidence proves RV and LA can close when severe blockers are excluded, but it is **not** stable promotion evidence: final closed sweeps still contain thousands of ordinary `TFAIL/TBROK/TCONF/ENOSYS` signals and wrapper failures.
- Stable506 remains the only trusted promotion baseline here: archived Session 8 RV and LA stable gates both report `PASS 1012 / FAIL 0`, `ltp-musl 506/0`, `ltp-glibc 506/0`, and only the inherited `read02 TCONF` caveat.
- LA quality risk is dominated by an arch-scoped network/resource blacklist family plus allocator panic and no-log-growth blockers. Session 7 removed only `creat07` and `tcp4-uni-basic01`; both are now ordinary closed failures, not PASS.

## Evidence sources cited

| Purpose | Path |
| --- | --- |
| Live stable list and runner selection code | `examples/shell/src/cmd.rs` (`LTP_STABLE_CASES`, `LTP_SWEEP_DEFAULT_BLACKLIST_CASES`, `selected_ltp_cases()`) |
| Remote default selection contract | `Makefile` (`REMOTE_LTP_CASES ?= stable-plus-blacklist`) |
| Current supplemental blacklist files | `docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt`, `blacklist-rv.txt`, `blacklist-la.txt` |
| Archived full-sweep closure | `docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-report.md`, `summaries/rv-arch002-summary.json`, `summaries/la-arch012-summary.json` |
| Older RV/LA delta and high-yield raw-log triage | `docs/ltp-full-sweep-blacklist-2026-05-29/final-report.md`, `high-yield-candidates.json` |
| Session 7 blacklist reduction | `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-07-la-severe-blockers/blacklist-change-report.md` |
| Stable506 final gate | `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/validation.md` |
| Candidate matrix | `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-01-baseline-candidate-matrix/candidate-matrix-stable460-to-500plus.md` |
| Network/proc/synthetic source surfaces | `examples/shell/src/uspace/fd_socket.rs`, `synthetic_fs.rs`, `system_info.rs`, `fd_table.rs` |

## Current baseline and runner semantics

Fresh local count:

```text
examples/shell/src/cmd.rs
LTP_STABLE_CASES total=506 unique=506 duplicates=0
```

Runner/remote semantics:

- `selected_ltp_cases()` supports pure `stable`, pure `all`, pure `all-minus-blacklist`, and score-safe `stable-plus-blacklist` / `score-blacklist` modes.
- `Makefile` defaults remote `make` / `make all` to `REMOTE_LTP_CASES=stable-plus-blacklist`.
- In default score mode, case order is:

```text
LTP_STABLE_CASES + (all guest LTP binaries - LTP_STABLE_CASES - active blacklist)
```

This means blacklist suppresses only the extra suffix in the default remote path; the stable whitelist still runs first. Pure `stable` and pure `blacklist` remain opt-in modes.

## Active blacklist reconstruction

### Input lists

| Source | Current unique count | Scope | Notes |
| --- | ---: | --- | --- |
| `LTP_SWEEP_DEFAULT_BLACKLIST_CASES` in `examples/shell/src/cmd.rs` | 38 | common built-in | experimental stress/cgroup/crash/forkbomb guardrail list |
| `blacklist-common.txt` | 5 | common supplemental | `pthserv`, `oom01`, `shmat1`, `accept02`, `mincore03` |
| `blacklist-rv.txt` | 1 | RV-only supplemental | `kill10` |
| `blacklist-la.txt` | 374 | LA-only supplemental | current after Session 7 removed `creat07` and `tcp4-uni-basic01` |

Effective unique names in blacklist-consuming runs:

| Arch | Built-in + common + arch-specific | Unique count | Duplicate count |
| --- | ---: | ---: | ---: |
| RV | `38 + 5 + 1` | 44 | 0 |
| LA | `38 + 5 + 374` | 417 | 0 |

Important count caveat: archived `final-quality-gate.json` and `final-report.md` from the 2026-05-30 closure still mention LA `376`; current live files and Session 7 docs supersede that number. The live count is `374`, because `creat07` and `tcp4-uni-basic01` were removed after targeted LA LTP-only runs closed normally as ordinary failures.

### LA supplemental composition

Current `blacklist-la.txt` groups as:

| Group | Count | Interpretation |
| --- | ---: | --- |
| `tcp4-*` | 153 | remaining LA TCPv4 network stress family after `tcp4-uni-basic01` removal |
| `tcp6-*` | 154 | LA TCPv6 network stress family |
| `udp4-*` | 28 | LA UDPv4 network stress family |
| `udp6-*` | 28 | LA UDPv6 network stress family |
| Non-network severe blockers | 11 | `fsync02`, `pth_str01`, `fcntl16`, `kill10`, `lftest`, `mmstress`, `dirtyc0w`, `write01`, `futex_wait01`, `futex_wait05`, `nice05` |

The network subset is still LA-only evidence. It must not be copied into RV/common unless RV evidence appears.

## Archived full-sweep evidence

### 2026-05-29 first full-sweep experiment

Source: `docs/ltp-full-sweep-blacklist-2026-05-29/final-report.md`.

| Run | Closure | Key parser/marker facts |
| --- | --- | --- |
| `rv-iter006` | closed | `RUN=4660`, normalized `PASS=1186`, `FAIL=3473`, `UNKNOWN=1`, `TIMEOUT=68`, incomplete `0`, panic/trap `0`; `cpuset_memory_pressure` marker glue required inline-aware audit. |
| `la-iter001` | not closed | stopped at LA-only `creat07` incomplete RUN; `RUN=160`, normalized `PASS=47`, `FAIL=112`, incomplete `1`; monitor terminated QEMU after no log growth. |

Interpretation: this run discovered the initial LA comparison blocker and high-yield scouting cases. It did not prove promotion; ordinary failures, `TCONF`, `TBROK`, `TFAIL`, and `ENOSYS` remained visible.

### 2026-05-30 arch-specific closure run

Sources: `docs/ltp-full-sweep-blacklist-2026-05-30-arch/final-report.md`, `summaries/rv-arch002-summary.json`, `summaries/la-arch012-summary.json`.

| Metric | RV `rv-arch002` | LA `la-arch012` | Quality meaning |
| --- | ---: | ---: | --- |
| Closed | true | true | both final pure blacklist sweeps reached terminal markers |
| `run_eval_status` | 0 | 0 | wrapper status alone is not enough; parser/marker audit still required |
| Selection | skipped `41` musl / `44` glibc | skipped `416` musl / `419` glibc | LA skips far more due arch-only severe blockers |
| RUN markers | 4658 | 3908 | LA executes fewer cases because many network/resource cases are excluded |
| Parser PASS | 1204 | 1207 | scouting PASS only; not promotion proof |
| Parser FAIL | 3453 | 2698 | ordinary failures remain real failures |
| TIMEOUT markers | 55 | 53 | closed timeouts are not blacklist reasons by themselves |
| Internal `TBROK/TCONF/TFAIL` | `1043 / 2663 / 4058` | `1031 / 1936 / 4041` | not clean; cannot promote from this evidence |
| ENOSYS/not implemented | 1280 | 1279 | large ABI gap remains |
| Incomplete / panic / trap / resource | `0 / 0 / 0 / 0` | `0 / 0 / 0 / 0` | quality closure criterion satisfied for the blacklisted sweep only |

Suite summary details:

- RV: `ltp-musl passed=598 failed=1729 timed_out=27`; `ltp-glibc passed=606 failed=1725 timed_out=28`.
- LA: `ltp-musl passed=602 failed=1350 timed_out=25`; `ltp-glibc passed=605 failed=1351 timed_out=28`.

Interpretation: LA's lower fail count is mostly skip surface, not better semantics. These logs are useful for backlog mining and severe-blocker tracking, not for stable-list promotion.

## Stable506 evidence boundary

Source: `docs/archive/ltp-os-long-term-plan-2026-06-01/ltp-os-long-term-plan-sessions-0601-docs/session-08-integration-final-gate/validation.md`.

| Arch | Command | PASS | FAIL | Suite summaries | Internal caveat | timeout / ENOSYS / panic-trap |
| --- | --- | ---: | ---: | --- | --- | --- |
| RV | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh rv` | 1012 | 0 | `ltp-musl 506/0`, `ltp-glibc 506/0` | inherited `read02 TCONF` only (`TCONF=4`) | `0 / 0 / 0` |
| LA | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh la` | 1012 | 0 | `ltp-musl 506/0`, `ltp-glibc 506/0` | inherited `read02 TCONF` only (`TCONF=4`) | `0 / 0 / 0` |

Marker-prefix audit in the archived Session 8 report found `0` non-prefix `LTP CASE` lines in both stable506 logs. Raw logs and JSON summaries are retained under `target/ltp-long-term-session8/`, with checksums recorded in the validation doc; they are not committed.

## LA severe blocker map

### Still active LA-only severe blockers

| Blocker class | Current cases | Evidence path | Diagnosis / removal gate |
| --- | --- | --- | --- |
| Network stress resource pollution | 363 current `tcp4-*`, `tcp6-*`, `udp4-*`, `udp6-*` entries | `blacklist-la.txt` comments and `summaries/la-arch001-summary.json` | Original failure was fork/resource exhaustion and polluted follow-on cases (`tcp4-multi-sameport09`, later `fork(): EAGAIN/EWOULDBLOCK`). Remove only after targeted family/shard closes with normal markers and no resource pollution. |
| Allocator panic / platform shutdown | `fsync02`, `lftest`, `mmstress`, `write01` | `blacklist-la.txt` comments for `la-arch002`, `la-arch005`, `la-arch006`, `la-arch008` | These can make `run-eval` exit 0 after QEMU shutdown; marker/panic audit is mandatory. Fix allocator/file-IO pressure before removing. |
| Guest hang / no log growth | `pth_str01`, `dirtyc0w`, `futex_wait01`, `futex_wait05`, `nice05` | `blacklist-la.txt` comments for `la-arch003`, `la-arch007`, `la-arch009`, `la-arch010`, `la-arch011` | Need normal PASS/FAIL/TIMEOUT wrapper marker and no stalled QEMU/guest hang. Partial TPASS lines are not enough. |
| Cumulative resource pollution | `fcntl16`, `kill10` | `blacklist-la.txt` comments for `la-arch004` | Markers may exist, but free-frame/resource drops poisoned later glibc cases. Removal requires stable resources through the case and follow-on shard. |

### No longer active as LA severe blockers

| Case | Current status | Evidence |
| --- | --- | --- |
| `creat07` | removed from LA blacklist; ordinary closed FAIL/TBROK, not PASS | Session 7 `blacklist-change-report.md` and `validation.md` |
| `tcp4-uni-basic01` | removed from LA blacklist; ordinary closed FAIL/TCONF, not PASS | Session 7 `blacklist-change-report.md` and `validation.md` |

### Cross-arch severe notes

- Common supplemental blockers remain `pthserv`, `oom01`, `shmat1`, `accept02`, `mincore03`.
- RV-only `kill10` remains due allocator panic evidence from `rv-arch001`; LA has its own separate `kill10` resource-pollution evidence.

## Network / proc / syntheticfs candidate diagnosis

### Score-candidate surface from archived matrix

From Session 1's clean-not-stable matrix, the current live stable506 list still does **not** include these four sweep-clean network/proc/synthetic candidates:

```text
accept01
listen01
socket02
socketpair02
```

They were `4/4` clean in archived full-sweep evidence, but that is still scouting-only. They need fresh targeted RV + LA × musl + glibc parser-backed gates before any promotion claim.

Additional source/runner-adjacent clean candidates still outside stable506 include `newuname01`, `utsname01`, `utsname04`, `tst_ncpus`, `tst_ncpus_conf`, `tst_ncpus_max`, and `tst_supported_fs`. Treat helper-style `tst_*` rows carefully: they may validate environment/synthetic reporting rather than Linux syscall semantics directly.

### Diagnosis candidates, not promotion candidates

- `socket01`: `docs/ltp-full-sweep-blacklist-2026-05-29/high-yield-candidates.json` records partial success (`TPASS=7`, `TFAIL=2` in both RV libc variants in that run). This points at socket errno/option semantics, not clean eligibility.
- LA network stress family: current blacklist still holds 363 network cases. This is primarily a full-sweep quality and resource-cleanup project before it becomes score work.
- Proc/syntheticfs gaps: `proc01` is already in stable, but broader `/proc` semantics remain narrow and should be audited as a model, not case-name shims.

### Source-level boundaries to respect

| Area | Current source behavior | Risk for future fixes |
| --- | --- | --- |
| Socket domains | `fd_socket.rs::sys_socket_bridge()` supports AF_UNIX local stream/datagram and AF_INET TCP/UDP; non-AF_INET domains return `EAFNOSUPPORT`. | IPv6/multicast/network-stress tests will not become real by adding case-specific exceptions; unsupported capabilities need honest errno/model decisions. |
| AF_UNIX pathname sockets | `sys_connect_local_socket()` has no pathname listener registry; existing filesystem nodes yield `ECONNREFUSED`, missing nodes `ENOENT`. | `socketpair02` is lower risk than pathname socket server/client semantics; do not overgeneralize from socketpair to full AF_UNIX bind/listen/connect. |
| Socket options | `socket_option_supported()` accepts a small set of SOL_SOCKET/IP/TCP options; many unsupported options return `EINVAL`. | LTP `getsockopt/setsockopt` candidates should start with an errno matrix, not blanket zero-success. |
| Address ABI | `read_socket_addr_from_user()` currently requires `addrlen == sizeof(sockaddr)`. | Linux often accepts variable sockaddr lengths per family; strict length may explain errno mismatches. |
| `/proc` synthetic files | `synthetic_fs.rs` synthesizes `/proc/self/maps`, `/proc/{pid}/stat`, `/proc/{pid}/status`, `/proc/*/comm`, and `/proc/*/exe` target links. | Values are simplified and not a general procfs. Fields like CPU time, scheduler, fd/task enumeration, and process_vm style visibility need real model work. |
| User/system synthetic reporting | `system_info.rs` synthesizes uname, sysinfo, syslog, rusage, prctl name/pdeathsig. | `newuname01`/`utsname*` may be low risk, but should be validated against field layout and architecture-specific strings, not hardcoded test output. |
| Synthetic path plumbing | `fd_table.rs` routes synthetic files via `synthetic_fs.rs` and maps `/dev/shm/*` to `/tmp/shm/*`. | Syntheticfs consistency risks include stat/open/readlink/getdents behavior mismatch across path and fd views. |

## Sweep-quality risks and guardrails

1. **Blacklist/SKIP/status0 is not PASS.** A blacklisted case is absent evidence, not success. Full-sweep partial `TPASS` lines are not promotion evidence when wrapper status or internal markers still fail.
2. **Parser/marker audit beats wrapper exit.** Several archived blocker iterations had `run-eval` status `0` after panic/platform shutdown; final claims need `scripts/ltp_summary.py` plus marker audit (`incomplete=0`, panic/trap/resource=0).
3. **Archived counts can drift.** The 2026-05-30 closure docs record LA `376`; live files and Session 7 now show LA `374`. Always recount from files before using skip math.
4. **Selection skip counts are target-dir dependent.** Effective blacklist input count is not identical to archived `skipped=N` because the runner only skips blacklist names present in each libc target directory and after stable/extras filtering.
5. **LA lower failure count is skip-surface bias.** LA final full sweep skipped hundreds more cases than RV; do not infer semantic superiority from fewer LA failures.
6. **Known `read02 TCONF` remains visible.** Stable506 accepts only the inherited `read02` O_DIRECT/tmpfs caveat; new `TCONF` in candidate lanes must block promotion.
7. **Raw logs are not committed.** Durable docs record paths and hashes; before deep case-by-case replay, verify retained `target/...` raw logs exist in the current environment.
8. **Marker glue exists.** `cpuset_memory_pressure` in older RV full-sweep logs required inline-aware marker audit; do not rely on naive line-prefix scans alone for all archived full-sweep logs.

## Recommended next slices

1. **LA network shard audit:** choose one small family shard, e.g. a few `tcp4-uni-*` / `udp4-uni-*` cases, and require normal closure plus no fork/resource pollution before removing any more LA blacklist entries.
2. **Allocator/panic lane:** isolate `fsync02`, `lftest`, `mmstress`, and `write01` as late-order allocator/file-IO pressure blockers; success means no panic/shutdown and normal markers, not necessarily PASS.
3. **Socket low-risk promotion scout:** if leader assigns QEMU artifacts, run fresh targeted RV/LA × musl/glibc for `accept01,listen01,socket02,socketpair02`; promote only if parser-clean and no new internal signals.
4. **Proc/syntheticfs model audit:** document current supported `/proc` and synthetic paths, then target one real semantics gap at a time. Avoid case-name/path shims.
5. **Quality gate for any blacklist removal:** update blacklist counts, record severe-blocker reason/removal condition, and run a shard that proves `incomplete=0`, panic/trap=0, resource failure=0.

## Subagent note

Subagent skip reason: this was a bounded report-only/source-diagnosis task with no parallel QEMU allowance and enough committed summaries/source files for direct verification. I used parallel local read-only shell scans instead of spawning native subagents.
