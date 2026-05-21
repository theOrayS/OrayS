# LTP score improvement brief 2026-05-22

目标：在上一轮稳定 44 cases / libc / arch 的基础上，把 stable batch 提升到 60-80 cases。

硬约束：
- 不伪造 PASS，不按 case name hardcode 成功，不把真实失败静默转 SKIP。
- timeout 单独计数且作为失败，不算 PASS。
- 不只看 run-eval exit code；必须用 scripts/ltp_summary.py 检查内部 TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
- 每次 promotion 必须有 LA/RV × musl/glibc 小 batch 证据，并记录新增 case 列表和风险。
- 先 targeted batch，再 final full gates。

优先候选：fs metadata/open/link/rename/statfs/access variants、proc/read-only metadata、time/signal basics、wait/exit/reporting、mmap/brk/msync 近邻。

本轮 Team lanes：
1. Discovery：枚举镜像/已有 batches，生成 16-36 个低风险候选，小批 LA/RV 验证，保存 raw logs。
2. Runner/Harness：检查 stable/batch/file LTP runner 是否足够可复现，必要时增强 batch 文件/inline 配置，保持 timeout fail 语义。
3. Stats/Report：增强或复用 scripts/ltp_summary.py 生成 promotion candidate matrix。
4. Syscall/ABI：根据 targeted logs 修最确定的 errno/metadata/proc/wait/time/signal 问题。
5. Hard-blocker：只调查 RV CVE/OOM、LA crash/trap，不阻塞 stable promotion。

初始证据：上一轮 LA/RV stable 44 per libc, 88 PASS each arch, 0 FAIL, internal TCONF=4, timeout=0, ENOSYS=0, panic/trap=0。
