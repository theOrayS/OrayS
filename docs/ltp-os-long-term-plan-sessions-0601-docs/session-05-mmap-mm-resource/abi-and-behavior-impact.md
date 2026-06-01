# Session 5 ABI and behavior impact

## User-visible syscall/errno behavior changes

- 新增 `mincore(2)` syscall：
  - `len == 0` 返回 0。
  - `addr` 必须按 4K 页对齐；非对齐返回 `EINVAL`。
  - 地址加法溢出、aligned end 超出用户空间、或范围内任一页未映射时返回 `ENOMEM`。
  - `vec` copy-out 目标不可写时返回 `EFAULT`。
  - 对每个已映射页写出 byte `1`，表示当前最小 VM 模型下 resident。
- `LTP_STABLE_CASES` 新增 11 个 mmap/mm/resource case；不改变 blacklist。

## ABI / struct layout / copy-in-copy-out

- 未修改公开 C struct layout；继续使用 `linux_raw_sys::general::__NR_mincore` ABI 常量。
- `mincore` 的用户缓冲区写入走现有 `validate_user_write` 和 `write_user_bytes`，不是直接内核指针写入。
- 没有硬编码 LTP case 名、路径、进程名或输出。

## FD / signal / futex / mmap impact

- 不修改 FD、signal、futex、scheduler 或 process ABI 行为。
- 不修改既有 `mmap/mprotect/msync/munmap` 语义；本 session 只新增 `mincore` 查询行为。
- `mprotect01/mprotect02` 的失败边界保持可见，未用 case-specific shim 掩盖。

## Known limitations

- `mincore` 不是完整 Linux 内存驻留/回收模型；当前只区分“页表已映射”与“未映射”，对已映射页统一返回 resident。
- 未实现 `mprotect` 权限降级后用户态 SIGSEGV handler 恢复路径；`mprotect02` 仍不能 promotion。
- 未修复 read/write 对 non-existent user buffer 的所有边界；`diotest4` 仍不能 promotion。
