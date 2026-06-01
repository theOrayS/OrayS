# Session 4 ABI and behavior impact

## User-visible syscall/errno behavior changes

- 新增 xattr syscall 行为：
  - `setxattr/lsetxattr/fsetxattr`：支持写入真实字节值，处理 `XATTR_CREATE`/`XATTR_REPLACE`，已有属性返回 `EEXIST`，缺失替换返回 `ENODATA`。
  - `getxattr/lgetxattr/fgetxattr`：支持查询长度和 copy-out 值；buffer 过小返回 `ERANGE`；属性缺失返回 `ENODATA`。
  - `listxattr/llistxattr/flistxattr`：返回 NUL-separated 属性名列表；buffer 过小返回 `ERANGE`。
  - `removexattr/lremovexattr/fremovexattr`：删除真实记录；属性缺失返回 `ENODATA`。
  - `f*` 变体通过 fd 解析路径，并拒绝 `O_PATH` fd（`EBADF`）。
- `statx`：空 pathname 且未设置 `AT_EMPTY_PATH` 时返回 `ENOENT`，不再错误解析为 cwd。
- `getdents64`：目录枚举增加进程记录的 symlink overlay 条目，`d_type` 为 `DT_LNK`；避免与底层目录项重复。
- `readlinkat`：
  - 过长 path 返回 `ENAMETOOLONG`。
  - `AT_FDCWD` + 空 path 返回 `ENOENT`。
  - 非 root 进程读取 symlink 前检查父目录搜索权限，权限不足返回 `EACCES`。

## ABI / struct layout / copy-in-copy-out

- 未修改公开 C struct layout；继续使用现有 `linux_raw_sys::general::*` ABI 常量和已有 internal ABI struct。
- 用户指针 copy-in/copy-out 继续走现有 `read_cstr/read_user_bytes/write_user_bytes/validate_user_write` 边界。
- xattr value/name/list 都是从用户指针真实 copy-in/copy-out；没有硬编码 LTP 输出或 case 名。

## FD / signal / futex / mmap impact

- FD 行为只影响 xattr `f*` syscall 的 fd-to-path 解析和 `getdents64` 对目录 fd 的返回内容。
- 不修改 signal、futex、mmap、scheduler 或 process ABI 行为。
- 不修改 testsuite/evaluator，不新增 blacklist。

## Known limitations

- xattr 当前是进程内路径映射，并在 fork/clone 路径中随 `UserProcess` 克隆；它不是完整持久化、全局一致的文件系统 xattr 实现。
- symlink overlay 和 xattr path key 依赖当前用户态进程模型记录，不能替代完整 VFS 元数据层。
- legacy `getdents` syscall 仍未实现，因此 `getdents01` 仍不能 promotion。
- `readlink03` 的组件级 `ELOOP` 等边界仍未完全实现；`readlinkat02` 在 LA musl 仍失败。
