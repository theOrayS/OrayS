# Session 8 ABI and behavior impact

Session 8 未修改源码、syscall dispatch、ABI layout、FD/signal/futex/mmap/user pointer copy-in/out 行为。用户可见行为变化为：无新增行为变化；本 session 只确认 Session 2~6 已提交语义改动在 stable506 final gate 中仍可通过 RV/LA × musl/glibc。
