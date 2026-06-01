# Session 8 blacklist change report

Session 8 未修改 blacklist。当前 active blacklist 状态来自 Session 7 commit `8b00b494`：

| File | Active entries | Session 8 change |
| --- | ---: | --- |
| `docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-common.txt` | 5 | none |
| `docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-rv.txt` | 1 | none |
| `docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt` | 374 | none |

Session 7 已把 LA-only blacklist 从 `376` 降到 `374`，移除：

- `creat07`：LA LTP-only targeted run closed normally as ordinary FAIL/TBROK (`PASS 0`, `FAIL 2`, internal `TBROK 2`), no timeout/ENOSYS/panic/trap。
- `tcp4-uni-basic01`：LA LTP-only targeted run closed normally as ordinary FAIL/TCONF (`PASS 0`, `FAIL 2`, internal `TCONF 2`), no timeout/ENOSYS/panic/trap。

边界：这两个 case 只是从 severe blocker 降级为普通失败；它们不是 PASS，不计 stable promotion。剩余 LA network family、allocator/resource/hang blockers 需要后续单项或 family shard 证明后才能继续移除。
