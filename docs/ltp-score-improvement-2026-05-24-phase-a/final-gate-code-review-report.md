# final gate code review report

Status: **reviewed for stable270 partial delivery; stable300 not approved**.

## Recommendation

- Approve the stable270 code/doc changes as a truthful partial delivery.
- Do not claim stable285 or stable300: the required promotion and aggregate gates are absent.

## Reviewed changes

| File | Review notes |
| --- | --- |
| `examples/shell/src/cmd.rs` | Adds exactly 20 evidence-backed cases; live count is 270 unique, 0 duplicates. No case-name PASS hardcoding was added. |
| `examples/shell/src/uspace/user_memory.rs` | Adds aggregate iovec length validation and rejects overflow / `SSIZE_MAX` excess with `EINVAL`; this is general ABI behavior, not LTP-specific branching. |
| `examples/shell/src/uspace/fd_table.rs` | Implements `preadv`/`pwritev` through existing file and user-memory helpers; enforces fd access mode for regular file IO; returns POSIX-shaped errors for positional IO on non-regular fds. |
| `examples/shell/src/uspace/syscall_dispatch.rs` | Wires `preadv`, `pwritev`, and `waitid` syscall dispatch. |
| `examples/shell/src/uspace/process_lifecycle.rs` | Adds `waitid` support for exited children and Linux-shaped signal wait status / `siginfo_t` fields. Unsupported stopped/continued wait modes remain explicit `EINVAL`, so `waitid07/08` are not promoted. |
| `examples/shell/src/uspace/signal_abi.rs` | Generalizes default terminating-signal handling without test-case names; improves real wait/signal semantics. |

## Guardrail findings

- No fake PASS or case-name success special casing found in the touched code.
- Real failures remain visible in post270 summaries: `waitpid01`, pipe, mmap, openat2, close_range, statx, fcntl lock, and timer cases were not promoted.
- `read02` TCONF remains disclosed in stable270 aggregate summaries.
- Raw logs and generated kernels/images are not intended for commit.

## Risks / follow-up

- Signal default-action changes are broader than the promoted kill/waitid subset. Stable270 aggregate passed on both architectures, but future signal cases should stay behind targeted evidence.
- `waitid` currently covers exited children only; `WSTOPPED`/`WCONTINUED` remain unsupported and are correctly blocked from promotion.
- `preadv`/`pwritev` are implemented for regular files; pipes/sockets return `ESPIPE`/`EBADF` through the fd layer and need separate evidence before expanding promotion.
