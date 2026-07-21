#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    sync::atomic::{AtomicI32, Ordering},
};

// This freestanding program is the userspace side of the repository-contained ABI
// smoke, so using a higher-level syscall or libc wrapper would test that wrapper
// instead of the kernel boundary. The unsafe surface is deliberately limited to the
// architecture syscall instruction and the two compiler-required C memory symbols.
// The semantic-evidence manifest builds and executes this same source on RV64 and
// LA64, and requires the ordered syscall assertions plus clean guest shutdown.

const SYS_DUP3: usize = 24;
const SYS_FLOCK: usize = 32;
const SYS_MKDIRAT: usize = 34;
const SYS_UNLINKAT: usize = 35;
const SYS_LINKAT: usize = 37;
const SYS_OPENAT: usize = 56;
const SYS_CLOSE: usize = 57;
const SYS_PIPE2: usize = 59;
const SYS_GETDENTS64: usize = 61;
const SYS_READ: usize = 63;
const SYS_WRITE: usize = 64;
const SYS_VMSPLICE: usize = 75;
const SYS_SPLICE: usize = 76;
const SYS_TEE: usize = 77;
const SYS_EXIT: usize = 93;
const SYS_FUTEX: usize = 98;
const SYS_NANOSLEEP: usize = 101;
const SYS_CLOCK_GETTIME: usize = 113;
const SYS_SCHED_SETAFFINITY: usize = 122;
const SYS_SCHED_GETAFFINITY: usize = 123;
const SYS_UNAME: usize = 160;
const SYS_GETPID: usize = 172;
const SYS_SOCKET: usize = 198;
const SYS_BIND: usize = 200;
const SYS_LISTEN: usize = 201;
const SYS_ACCEPT: usize = 202;
const SYS_CONNECT: usize = 203;
const SYS_SENDTO: usize = 206;
const SYS_RECVFROM: usize = 207;
const SYS_SETSOCKOPT: usize = 208;
const SYS_MUNMAP: usize = 215;
const SYS_CLONE: usize = 220;
const SYS_EXECVE: usize = 221;
const SYS_MMAP: usize = 222;
const SYS_MADVISE: usize = 233;
const SYS_WAIT4: usize = 260;
const SYS_RENAMEAT2: usize = 276;
const SYS_STATX: usize = 291;
const SYS_CLONE3: usize = 435;

const NEG_ENOENT: isize = -2;
const NEG_E2BIG: isize = -7;
const NEG_EFAULT: isize = -14;
const NEG_EBADF: isize = -9;
const NEG_EAGAIN: isize = -11;
const NEG_EINVAL: isize = -22;
const NEG_ESPIPE: isize = -29;
const AT_FDCWD: isize = -100;
const AT_REMOVEDIR: usize = 0x200;
const O_RDONLY: usize = 0;
const O_WRONLY: usize = 1;
const O_RDWR: usize = 2;
const O_CREAT: usize = 0o100;
const O_NONBLOCK: usize = 0o4000;
const O_DIRECTORY: usize = 0o200000;
const LOCK_EX: usize = 2;
const LOCK_UN: usize = 8;
const WNOHANG: usize = 1;
const AF_INET: usize = 2;
const SOCK_STREAM: usize = 1;
const SOL_SOCKET: usize = 1;
const SO_REUSEADDR: usize = 2;
const SIGCHLD: usize = 17;
const CLONE_VM: u64 = 0x0000_0100;
const CLONE_FS: u64 = 0x0000_0200;
const CLONE_FILES: u64 = 0x0000_0400;
const CLONE_SIGHAND: u64 = 0x0000_0800;
const CLONE_VFORK: u64 = 0x0000_4000;
const CLONE_THREAD: u64 = 0x0001_0000;
const CLONE_SYSVSEM: u64 = 0x0004_0000;
const CLONE_SETTLS: u64 = 0x0008_0000;
const CLONE_PARENT_SETTID: u64 = 0x0010_0000;
const CLONE_CHILD_CLEARTID: u64 = 0x0020_0000;
const FUTEX_WAIT_BITSET: usize = 9;
const FUTEX_CLOCK_REALTIME: usize = 256;
const FUTEX_BITSET_MATCH_ANY: usize = u32::MAX as usize;
const CLOCK_MONOTONIC: usize = 1;
const PROT_READ: usize = 1;
const PROT_WRITE: usize = 2;
const MAP_PRIVATE: usize = 2;
const MAP_ANONYMOUS: usize = 32;
const MADV_DONTNEED: usize = 4;
const AT_EMPTY_PATH: usize = 0x1000;
const STATX_BASIC_STATS: usize = 0x07ff;
const STATX_TYPE: u32 = 0x0001;
const STATX_NLINK: u32 = 0x0004;
const S_IFMT: u16 = 0o170000;
const S_IFREG: u16 = 0o100000;
const PAGE_BYTES: usize = 4096;
const MADVISE_PROBE_BYTES: usize = 8 * 1024 * 1024;
const MADVISE_PROBE_ITERATIONS: usize = 16;
const CARGO_THREAD_CLONE_FLAGS: u64 = CLONE_VM
    | CLONE_FS
    | CLONE_FILES
    | CLONE_SIGHAND
    | CLONE_THREAD
    | CLONE_SYSVSEM
    | CLONE_SETTLS
    | CLONE_PARENT_SETTID
    | CLONE_CHILD_CLEARTID;
const CPUSET_BYTES: usize = core::mem::size_of::<usize>();
const AFFINITY_BUFFER_BYTES: usize = 128;
const TCP_FORK_CLIENTS: usize = 8;
const TCP_FORK_PORT: u16 = 39_026;
const EXEC_HELPER_PATH: &[u8] = b"/tmp/pr3-semantic-exec-helper\0";
const EXEC_HELPER_PAYLOAD: &[u8] = b"orays-exec-helper\n";
const FLOCK_PROBE_PATH: &[u8] = b"/tmp/pr3-semantic-flock-lock\0";
const HARDLINK_RENAME_SOURCE: &[u8] = b"/tmp/pr3-semantic-hardlink-source\0";
const HARDLINK_RENAME_ALIAS: &[u8] = b"/tmp/pr3-semantic-hardlink-alias\0";
const HARDLINK_RENAME_TARGET: &[u8] = b"/tmp/pr3-semantic-hardlink-target\0";
const HARDLINK_RENAME_SOURCE_DATA: &[u8] = b"SRC!";
const HARDLINK_RENAME_TARGET_DATA: &[u8] = b"DST?";
const CARGO_LINK_SOURCE: &[u8] = b"/tmp/pr3-semantic-cargo-link-source\0";
const CARGO_LINK_WORKING_DIR: &[u8] = b"/tmp/pr3-semantic-cargo-link-working\0";
const CARGO_LINK_WORKING_ALIAS: &[u8] =
    b"/tmp/pr3-semantic-cargo-link-working/object.o\0";
const CARGO_LINK_PUBLISHED_DIR: &[u8] = b"/tmp/pr3-semantic-cargo-link-published\0";
const CARGO_LINK_PUBLISHED_ALIAS: &[u8] =
    b"/tmp/pr3-semantic-cargo-link-published/object.o\0";
const CARGO_LINK_ENTRY_NAME: &[u8] = b"object.o";
const CARGO_LINK_SOURCE_DATA: &[u8] = b"OBJ!";
const CLONE3_THREAD_STACK_BYTES: usize = 64 * 1024;
const CLONE3_VFORK_STACK_BYTES: usize = 64 * 1024;

// A u128 array gives the clone3 child stack the 16-byte alignment required by
// both target ABIs. Access is exclusively through raw pointers: the parent never
// creates a Rust reference while the kernel-created thread owns this stack.
static mut CLONE3_THREAD_STACK: [u128; CLONE3_THREAD_STACK_BYTES / 16] =
    [0; CLONE3_THREAD_STACK_BYTES / 16];
static CLONE3_THREAD_TID: AtomicI32 = AtomicI32::new(0);
static CLONE3_THREAD_TLS_ANCHOR: AtomicI32 = AtomicI32::new(0);

// glibc's clone3-based posix_spawn path supplies a dedicated stack even though
// CLONE_VM|CLONE_VFORK keeps the parent suspended until exec or exit. Keep that
// stack distinct from the worker-thread stack and access it only through raw
// pointers while the child owns it.
static mut CLONE3_VFORK_STACK: [u128; CLONE3_VFORK_STACK_BYTES / 16] =
    [0; CLONE3_VFORK_STACK_BYTES / 16];
static CLONE3_VFORK_STAGE: AtomicI32 = AtomicI32::new(0);

// The first vector is exactly the largest pipe capacity supported by OrayS. A
// blocking vmsplice that fills it must return its progress rather than wait on
// the following vector in the same syscall. The one-byte second vector then
// proves that the caller can drain the pipe and resume without data loss.
const VMSPLICE_FIRST_LEN: usize = 64 * 1024;
const VMSPLICE_TOTAL_LEN: usize = VMSPLICE_FIRST_LEN + 1;
static VMSPLICE_FIRST: [u8; VMSPLICE_FIRST_LEN] = [0x3c; VMSPLICE_FIRST_LEN];
static VMSPLICE_SECOND: [u8; 1] = [0xa5];

#[cfg(target_arch = "riscv64")]
const USER_START: &[u8] = b"PR3_SMOKE_V1 USER_START arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_START: &[u8] = b"PR3_SMOKE_V1 USER_START arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_WRITE: &[u8] = b"PR3_SMOKE_V1 ASSERT write PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_WRITE: &[u8] = b"PR3_SMOKE_V1 ASSERT write PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_GETPID: &[u8] = b"PR3_SMOKE_V1 ASSERT getpid PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_GETPID: &[u8] = b"PR3_SMOKE_V1 ASSERT getpid PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_SCHED_AFFINITY: &[u8] =
    b"PR3_SMOKE_V1 ASSERT sched_affinity PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_SCHED_AFFINITY: &[u8] =
    b"PR3_SMOKE_V1 ASSERT sched_affinity PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_PROC_UPTIME: &[u8] =
    b"PR3_SMOKE_V1 ASSERT proc_uptime PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_PROC_UPTIME: &[u8] =
    b"PR3_SMOKE_V1 ASSERT proc_uptime PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_PROC_STATM: &[u8] = b"PR3_SMOKE_V1 ASSERT proc_statm PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_PROC_STATM: &[u8] = b"PR3_SMOKE_V1 ASSERT proc_statm PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_MADVISE_DONTNEED: &[u8] =
    b"PR3_SMOKE_V1 ASSERT madvise_dontneed PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_MADVISE_DONTNEED: &[u8] =
    b"PR3_SMOKE_V1 ASSERT madvise_dontneed PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_SPLICE_PIPE: &[u8] = b"PR3_SMOKE_V1 ASSERT splice_pipe PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_SPLICE_PIPE: &[u8] = b"PR3_SMOKE_V1 ASSERT splice_pipe PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_PIPE_FORK_EXEC: &[u8] = b"PR3_SMOKE_V1 ASSERT pipe_fork_exec PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_PIPE_FORK_EXEC: &[u8] = b"PR3_SMOKE_V1 ASSERT pipe_fork_exec PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_FLOCK_BLOCKING: &[u8] =
    b"PR3_SMOKE_V1 ASSERT flock_blocking PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_FLOCK_BLOCKING: &[u8] =
    b"PR3_SMOKE_V1 ASSERT flock_blocking PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_HARDLINK_RENAME_REPLACE: &[u8] =
    b"PR3_SMOKE_V1 ASSERT hardlink_rename_replace PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_HARDLINK_RENAME_REPLACE: &[u8] =
    b"PR3_SMOKE_V1 ASSERT hardlink_rename_replace PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_CARGO_LINK_PUBLISH: &[u8] =
    b"PR3_SMOKE_V1 ASSERT cargo_link_publish PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_CARGO_LINK_PUBLISH: &[u8] =
    b"PR3_SMOKE_V1 ASSERT cargo_link_publish PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_CLONE3_PROCESS: &[u8] = b"PR3_SMOKE_V1 ASSERT clone3_process PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_CLONE3_PROCESS: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_process PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const CLONE3_THREAD_CHILD: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_thread_child PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const CLONE3_THREAD_CHILD: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_thread_child PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_CLONE3_THREAD: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_thread PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_CLONE3_THREAD: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_thread PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_CLONE3_FUTEX_JOIN: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_futex_join PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_CLONE3_FUTEX_JOIN: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_futex_join PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_CLONE3_VFORK_EXEC: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_vfork_exec PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_CLONE3_VFORK_EXEC: &[u8] =
    b"PR3_SMOKE_V1 ASSERT clone3_vfork_exec PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_TCP_FORK_LOOPBACK: &[u8] =
    b"PR3_SMOKE_V1 ASSERT tcp_fork_loopback PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_TCP_FORK_LOOPBACK: &[u8] =
    b"PR3_SMOKE_V1 ASSERT tcp_fork_loopback PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_UNAME_SYSNAME: &[u8] = b"PR3_SMOKE_V1 ASSERT uname_sysname PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_UNAME_SYSNAME: &[u8] = b"PR3_SMOKE_V1 ASSERT uname_sysname PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const ASSERT_UNAME_MACHINE: &[u8] = b"PR3_SMOKE_V1 ASSERT uname_machine PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const ASSERT_UNAME_MACHINE: &[u8] = b"PR3_SMOKE_V1 ASSERT uname_machine PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_PASS: &[u8] = b"PR3_SMOKE_V1 USER_PASS arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_PASS: &[u8] = b"PR3_SMOKE_V1 USER_PASS arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_WRITE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL write arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_WRITE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL write arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_GETPID: &[u8] = b"PR3_SMOKE_V1 USER_FAIL getpid arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_GETPID: &[u8] = b"PR3_SMOKE_V1 USER_FAIL getpid arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_SCHED_AFFINITY_SYSCALL: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL sched_affinity_syscall arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_SCHED_AFFINITY_SYSCALL: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL sched_affinity_syscall arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_SCHED_AFFINITY_MASK: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL sched_affinity_mask arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_SCHED_AFFINITY_MASK: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL sched_affinity_mask arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_UPTIME_OPEN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_open arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_UPTIME_OPEN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_open arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_UPTIME_READ: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_read arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_UPTIME_READ: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_read arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_UPTIME_CLOSE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_close arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_UPTIME_CLOSE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_close arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_UPTIME_FORMAT: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_format arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_UPTIME_FORMAT: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_format arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_UPTIME_SLEEP: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_sleep arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_UPTIME_SLEEP: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_sleep arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_UPTIME_ADVANCE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_advance arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_UPTIME_ADVANCE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_uptime_advance arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_STATM_OPEN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_open arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_STATM_OPEN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_open arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_STATM_STATX: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_statx arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_STATM_STATX: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_statx arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_STATM_READ: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_read arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_STATM_READ: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_read arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_STATM_CLOSE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_close arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_STATM_CLOSE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_close arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PROC_STATM_FORMAT: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_format arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PROC_STATM_FORMAT: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL proc_statm_format arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_MADVISE_DONTNEED: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL madvise_dontneed arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_MADVISE_DONTNEED: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL madvise_dontneed arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_SPLICE_PIPE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL splice_pipe arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_SPLICE_PIPE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL splice_pipe arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PIPE_FORK_EXEC: &[u8] = b"PR3_SMOKE_V1 USER_FAIL pipe_fork_exec arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PIPE_FORK_EXEC: &[u8] = b"PR3_SMOKE_V1 USER_FAIL pipe_fork_exec arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_FLOCK_BLOCKING: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL flock_blocking arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_FLOCK_BLOCKING: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL flock_blocking arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_HARDLINK_RENAME_REPLACE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL hardlink_rename_replace arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_HARDLINK_RENAME_REPLACE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL hardlink_rename_replace arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_CARGO_LINK_PUBLISH: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL cargo_link_publish arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_CARGO_LINK_PUBLISH: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL cargo_link_publish arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_CLONE3_PROCESS: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_process arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_CLONE3_PROCESS: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_process arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_CLONE3_THREAD: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_thread arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_CLONE3_THREAD: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_thread arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_CLONE3_FUTEX_JOIN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_futex_join arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_CLONE3_FUTEX_JOIN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_futex_join arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_CLONE3_THREAD_WRITE_EBADF: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_thread_write_ebadf arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_CLONE3_THREAD_WRITE_EBADF: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_thread_write_ebadf arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_CLONE3_THREAD_WRITE_EFAULT: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_thread_write_efault arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_CLONE3_THREAD_WRITE_EFAULT: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_thread_write_efault arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_CLONE3_THREAD_WRITE_OTHER: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_thread_write_other arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_CLONE3_THREAD_WRITE_OTHER: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_thread_write_other arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_CLONE3_VFORK_EXEC: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_vfork_exec arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_CLONE3_VFORK_EXEC: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL clone3_vfork_exec arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_TCP_FORK_LOOPBACK: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tcp_fork_loopback arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_TCP_FORK_LOOPBACK: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tcp_fork_loopback arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_TEE_DEVICE_OPEN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_open arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_TEE_DEVICE_OPEN: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_open arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_TEE_DEVICE_MODE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_mode arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_TEE_DEVICE_MODE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_mode arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_TEE_DEVICE_CLOSE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_close arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_TEE_DEVICE_CLOSE: &[u8] =
    b"PR3_SMOKE_V1 USER_FAIL tee_device_close arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_UNAME: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_UNAME: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_SYSNAME: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname_sysname arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_SYSNAME: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname_sysname arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_MACHINE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname_machine arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_MACHINE: &[u8] = b"PR3_SMOKE_V1 USER_FAIL uname_machine arch=loongarch64\n";
#[cfg(target_arch = "riscv64")]
const USER_FAIL_PANIC: &[u8] = b"PR3_SMOKE_V1 USER_FAIL panic arch=riscv64\n";
#[cfg(target_arch = "loongarch64")]
const USER_FAIL_PANIC: &[u8] = b"PR3_SMOKE_V1 USER_FAIL panic arch=loongarch64\n";

#[cfg(target_arch = "riscv64")]
const EXPECTED_MACHINE: &[u8] = b"riscv64";
#[cfg(target_arch = "loongarch64")]
const EXPECTED_MACHINE: &[u8] = b"loongarch64";

#[repr(C)]
struct UtsName {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
    domainname: [u8; 65],
}

#[repr(C)]
struct IoVec {
    base: usize,
    len: usize,
}

#[repr(C)]
struct Timespec {
    seconds: i64,
    nanoseconds: i64,
}

#[derive(Clone, Copy)]
struct StatIdentity {
    inode: u64,
    nlink: u32,
}

enum ProcUptimeError {
    Open,
    Read,
    Close,
    Format,
}

enum ProcStatmError {
    Open,
    Statx,
    Read,
    Close,
    Format,
}

struct DecimalCentiseconds {
    value: u64,
    integer_digits: usize,
    fractional_digits: usize,
    decimal_seen: bool,
}

impl DecimalCentiseconds {
    const fn new() -> Self {
        Self {
            value: 0,
            integer_digits: 0,
            fractional_digits: 0,
            decimal_seen: false,
        }
    }

    fn push(&mut self, byte: u8) -> Option<()> {
        if byte == b'.' {
            if self.decimal_seen || self.integer_digits == 0 {
                return None;
            }
            self.decimal_seen = true;
            return Some(());
        }
        if !byte.is_ascii_digit() {
            return None;
        }
        if self.decimal_seen {
            self.fractional_digits += 1;
            if self.fractional_digits > 2 {
                return None;
            }
        } else {
            self.integer_digits += 1;
        }
        self.value = self
            .value
            .checked_mul(10)?
            .checked_add((byte - b'0') as u64)?;
        Some(())
    }

    fn finish(&self) -> Option<u64> {
        (self.integer_digits > 0 && self.decimal_seen && self.fractional_digits == 2)
            .then_some(self.value)
    }
}

impl UtsName {
    const fn zeroed() -> Self {
        Self {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        }
    }
}

#[cfg(target_arch = "riscv64")]
#[inline(always)]
/// Issues a six-argument syscall using the Linux RV64 userspace ABI.
///
/// # Safety
///
/// `number` must identify a syscall whose six raw arguments have the layouts
/// supplied in `arg0..=arg5`. A pointer may intentionally be invalid when exercising
/// kernel rejection and errno precedence, but Rust must never dereference such a value;
/// whenever the syscall is expected to access memory, the complete range must remain
/// valid with the required access and writable memory must not be aliased until `ecall`
/// returns. The caller must interpret the Linux return value, including negative errno.
/// This instruction binding uses `a0..a5` for arguments/return and `a7` for the number;
/// it intentionally does not claim `nomem` because the kernel may access caller memory.
unsafe fn syscall6(
    number: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the raw syscall argument contract documented above,
    // and this target-specific block names the Linux RV64 syscall ABI registers.
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a7") number,
            options(nostack)
        );
    }
    ret
}

#[cfg(target_arch = "loongarch64")]
#[inline(always)]
/// Issues a six-argument syscall using the Linux LA64 userspace ABI.
///
/// # Safety
///
/// `number` must identify a syscall whose six raw arguments have the layouts
/// supplied in `arg0..=arg5`. A pointer may intentionally be invalid when exercising
/// kernel rejection and errno precedence, but Rust must never dereference such a value;
/// whenever the syscall is expected to access memory, the complete range must remain
/// valid with the required access and writable memory must not be aliased until
/// `syscall 0` returns. The caller must interpret the Linux return value, including
/// negative errno. This binding uses `$a0..$a5` for arguments/return and `$a7` for the
/// number; it intentionally does not claim `nomem` because the kernel may access caller
/// memory.
unsafe fn syscall6(
    number: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the raw syscall argument contract documented above,
    // and this target-specific block names the Linux LA64 syscall ABI registers.
    unsafe {
        core::arch::asm!(
            "syscall 0",
            inlateout("$a0") arg0 => ret,
            in("$a1") arg1,
            in("$a2") arg2,
            in("$a3") arg3,
            in("$a4") arg4,
            in("$a5") arg5,
            in("$a7") number,
            options(nostack)
        );
    }
    ret
}

#[inline(always)]
fn write(bytes: &[u8]) -> isize {
    // SAFETY: `bytes` is readable for exactly `len` bytes and remains live until the
    // synchronous write returns. fd 1 and the length are scalar SYS_WRITE arguments.
    unsafe { syscall6(SYS_WRITE, 1, bytes.as_ptr() as usize, bytes.len(), 0, 0, 0) }
}

#[inline(always)]
fn sched_getaffinity(mask: &mut [u8; AFFINITY_BUFFER_BYTES]) -> isize {
    // SAFETY: `mask` is uniquely borrowed and writable for the complete supplied
    // cpusetsize until this synchronous syscall returns. pid zero selects the caller.
    unsafe {
        syscall6(
            SYS_SCHED_GETAFFINITY,
            0,
            mask.len(),
            mask.as_mut_ptr() as usize,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn sched_setaffinity(mask: &[u8; CPUSET_BYTES]) -> isize {
    // SAFETY: `mask` remains readable for exactly `CPUSET_BYTES` until this
    // synchronous syscall returns. pid zero selects the caller.
    unsafe {
        syscall6(
            SYS_SCHED_SETAFFINITY,
            0,
            mask.len(),
            mask.as_ptr() as usize,
            0,
            0,
            0,
        )
    }
}

fn affinity_snapshot_matches(
    buffer: &[u8; AFFINITY_BUFFER_BYTES],
    expected_first_byte: u8,
) -> bool {
    buffer[0] == expected_first_byte
        && buffer[1..CPUSET_BYTES].iter().all(|byte| *byte == 0)
        && buffer[CPUSET_BYTES..].iter().all(|byte| *byte == 0xa5)
}

#[inline(always)]
fn pipe2(pipe: &mut [i32; 2], flags: usize) -> isize {
    // SAFETY: `pipe` is uniquely borrowed and writable for two i32 values until the
    // synchronous syscall returns; flags and unused argument slots are scalar values.
    unsafe {
        syscall6(
            SYS_PIPE2,
            pipe.as_mut_ptr() as usize,
            flags,
            0,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn dup3(oldfd: i32, newfd: i32, flags: usize) -> isize {
    // SAFETY: dup3 consumes only descriptor and flag scalars. The kernel validates
    // both descriptors and atomically replaces the destination when required.
    unsafe { syscall6(SYS_DUP3, oldfd as usize, newfd as usize, flags, 0, 0, 0) }
}

#[inline(always)]
fn pipe_write(fd: i32, bytes: &[u8]) -> isize {
    // SAFETY: `bytes` remains readable for its complete length until the synchronous
    // syscall returns; the descriptor and unused argument slots are scalar values.
    unsafe {
        syscall6(
            SYS_WRITE,
            fd as usize,
            bytes.as_ptr() as usize,
            bytes.len(),
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn fd_read(fd: i32, bytes: &mut [u8]) -> isize {
    // SAFETY: `bytes` is uniquely borrowed and writable for its complete length until
    // the synchronous syscall returns; no Rust reference observes it during the call.
    unsafe {
        syscall6(
            SYS_READ,
            fd as usize,
            bytes.as_mut_ptr() as usize,
            bytes.len(),
            0,
            0,
            0,
        )
    }
}

fn parse_proc_uptime(bytes: &[u8], length: usize) -> Option<(u64, u64)> {
    if length > bytes.len() {
        return None;
    }
    let mut current = DecimalCentiseconds::new();
    let mut in_field = false;
    let mut fields = 0;
    let mut uptime = None;
    let mut idle = None;
    for byte in bytes.iter().take(length) {
        if byte.is_ascii_whitespace() {
            if in_field {
                let value = current.finish()?;
                if fields == 0 {
                    uptime = Some(value);
                } else if fields == 1 {
                    idle = Some(value);
                } else {
                    return None;
                }
                fields += 1;
                current = DecimalCentiseconds::new();
                in_field = false;
            }
        } else {
            if fields >= 2 {
                return None;
            }
            current.push(*byte)?;
            in_field = true;
        }
    }
    if in_field {
        let value = current.finish()?;
        if fields == 0 {
            uptime = Some(value);
        } else if fields == 1 {
            idle = Some(value);
        } else {
            return None;
        }
        fields += 1;
    }
    (fields == 2).then_some((uptime?, idle?))
}

fn read_proc_uptime() -> Result<(u64, u64), ProcUptimeError> {
    let fd = openat(b"/proc/uptime\0", O_RDONLY);
    if fd < 0 {
        return Err(ProcUptimeError::Open);
    }
    let fd = fd as i32;
    let mut buffer = [0_u8; 64];
    let read = fd_read(fd, &mut buffer);
    if read <= 0 || read as usize > buffer.len() {
        let _ = close(fd);
        return Err(ProcUptimeError::Read);
    }
    if close(fd) != 0 {
        return Err(ProcUptimeError::Close);
    }
    parse_proc_uptime(&buffer, read as usize).ok_or(ProcUptimeError::Format)
}

fn fail_proc_uptime(error: ProcUptimeError, code: usize) -> ! {
    let marker = match error {
        ProcUptimeError::Open => USER_FAIL_PROC_UPTIME_OPEN,
        ProcUptimeError::Read => USER_FAIL_PROC_UPTIME_READ,
        ProcUptimeError::Close => USER_FAIL_PROC_UPTIME_CLOSE,
        ProcUptimeError::Format => USER_FAIL_PROC_UPTIME_FORMAT,
    };
    fail(marker, code)
}

fn parse_proc_statm(bytes: &[u8], length: usize) -> Option<[u64; 7]> {
    if length > bytes.len() {
        return None;
    }
    let mut values = [0_u64; 7];
    let mut fields = 0_usize;
    let mut value = 0_u64;
    let mut in_field = false;
    for byte in bytes.iter().take(length) {
        if byte.is_ascii_whitespace() {
            if in_field {
                if fields >= values.len() {
                    return None;
                }
                values[fields] = value;
                fields += 1;
                value = 0;
                in_field = false;
            }
        } else {
            if fields >= values.len() || !byte.is_ascii_digit() {
                return None;
            }
            value = value.checked_mul(10)?.checked_add((byte - b'0') as u64)?;
            in_field = true;
        }
    }
    if in_field {
        if fields >= values.len() {
            return None;
        }
        values[fields] = value;
        fields += 1;
    }
    (fields == values.len()).then_some(values)
}

fn read_proc_statm() -> Result<[u64; 7], ProcStatmError> {
    let fd = openat(b"/proc/self/statm\0", O_RDONLY);
    if fd < 0 {
        return Err(ProcStatmError::Open);
    }
    let fd = fd as i32;

    // Linux's statx structure is 256 bytes and naturally 8-byte aligned. Only the
    // stable leading mask and mode fields are inspected here; the remaining words
    // stay available for the kernel's complete ABI write.
    let mut statx = [0_u64; 32];
    // SAFETY: the empty path is a live NUL-terminated byte string, `statx` is an
    // aligned writable 256-byte output object, and AT_EMPTY_PATH selects `fd`.
    let statx_result = unsafe {
        syscall6(
            SYS_STATX,
            fd as usize,
            b"\0".as_ptr() as usize,
            AT_EMPTY_PATH,
            STATX_BASIC_STATS,
            statx.as_mut_ptr() as usize,
            0,
        )
    };
    let statx_mask = statx[0] as u32;
    let statx_mode = ((statx[3] >> 32) & u16::MAX as u64) as u16;
    if statx_result != 0
        || statx_mask & STATX_TYPE == 0
        || statx_mode & S_IFMT != S_IFREG
    {
        let _ = close(fd);
        return Err(ProcStatmError::Statx);
    }

    let mut buffer = [0_u8; 160];
    let read = fd_read(fd, &mut buffer);
    if read <= 0 || read as usize > buffer.len() {
        let _ = close(fd);
        return Err(ProcStatmError::Read);
    }
    if close(fd) != 0 {
        return Err(ProcStatmError::Close);
    }
    let values = parse_proc_statm(&buffer, read as usize).ok_or(ProcStatmError::Format)?;
    let [size, resident, shared, text, library, data, dirty] = values;
    if size == 0
        || resident == 0
        || resident > size
        || shared > resident
        || text == 0
        || text > size
        || library != 0
        || data == 0
        || data > size
        || dirty != 0
    {
        return Err(ProcStatmError::Format);
    }
    Ok(values)
}

fn fail_proc_statm(error: ProcStatmError, code: usize) -> ! {
    let marker = match error {
        ProcStatmError::Open => USER_FAIL_PROC_STATM_OPEN,
        ProcStatmError::Statx => USER_FAIL_PROC_STATM_STATX,
        ProcStatmError::Read => USER_FAIL_PROC_STATM_READ,
        ProcStatmError::Close => USER_FAIL_PROC_STATM_CLOSE,
        ProcStatmError::Format => USER_FAIL_PROC_STATM_FORMAT,
    };
    fail(marker, code)
}

#[inline(always)]
fn tee(fd_in: i32, fd_out: i32, len: usize, flags: usize) -> isize {
    // SAFETY: tee consumes only scalar descriptors, length, and flags; it has no
    // userspace pointer arguments.
    unsafe {
        syscall6(
            SYS_TEE,
            fd_in as usize,
            fd_out as usize,
            len,
            flags,
            0,
            0,
        )
    }
}

#[inline(always)]
fn vmsplice(fd: i32, iovecs: &[IoVec], flags: usize) -> isize {
    // SAFETY: the iovec array and every described byte range remain readable until
    // this synchronous vmsplice returns. The descriptors, count, and flags are
    // scalar arguments, and the immutable backing arrays outlive the call.
    unsafe {
        syscall6(
            SYS_VMSPLICE,
            fd as usize,
            iovecs.as_ptr() as usize,
            iovecs.len(),
            flags,
            0,
            0,
        )
    }
}

#[inline(always)]
fn splice(fd_in: i32, fd_out: i32, len: usize, flags: usize) -> isize {
    // SAFETY: both offset pointers are null by contract, descriptors/length/flags are
    // scalars, and the kernel validates descriptor direction and available data.
    unsafe {
        syscall6(
            SYS_SPLICE,
            fd_in as usize,
            0,
            fd_out as usize,
            0,
            len,
            flags,
        )
    }
}

#[inline(always)]
fn close(fd: i32) -> isize {
    // SAFETY: SYS_CLOSE consumes only the scalar descriptor and ignores the remaining
    // argument slots. Callers ensure each successfully installed descriptor is closed
    // at most once in the success path.
    unsafe { syscall6(SYS_CLOSE, fd as usize, 0, 0, 0, 0, 0) }
}

#[inline(always)]
fn openat(path: &[u8], flags: usize) -> isize {
    openat_mode(path, flags, 0)
}

#[inline(always)]
fn openat_mode(path: &[u8], flags: usize, mode: usize) -> isize {
    // SAFETY: callers provide a readable NUL-terminated pathname that remains live
    // until this synchronous syscall returns. AT_FDCWD, flags, and mode are scalars;
    // callers requesting O_CREAT supply the intended permission bits explicitly.
    unsafe {
        syscall6(
            SYS_OPENAT,
            AT_FDCWD as usize,
            path.as_ptr() as usize,
            flags,
            mode,
            0,
            0,
        )
    }
}

#[inline(always)]
fn flock(fd: i32, operation: usize) -> isize {
    // SAFETY: flock consumes only a scalar descriptor and operation bitmask; all
    // remaining raw argument slots are ignored by the Linux syscall contract.
    unsafe { syscall6(SYS_FLOCK, fd as usize, operation, 0, 0, 0, 0) }
}

#[inline(always)]
fn unlinkat(path: &[u8]) -> isize {
    // SAFETY: `path` is a live readable NUL-terminated pathname for the complete
    // synchronous call. AT_FDCWD and zero flags request ordinary file unlink.
    unsafe {
        syscall6(
            SYS_UNLINKAT,
            AT_FDCWD as usize,
            path.as_ptr() as usize,
            0,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn mkdirat(path: &[u8], mode: usize) -> isize {
    // SAFETY: `path` is a live readable NUL-terminated pathname for the complete
    // synchronous call. AT_FDCWD and mode are scalar Linux mkdirat arguments.
    unsafe {
        syscall6(
            SYS_MKDIRAT,
            AT_FDCWD as usize,
            path.as_ptr() as usize,
            mode,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn unlinkat_dir(path: &[u8]) -> isize {
    // SAFETY: `path` is a live readable NUL-terminated pathname for the complete
    // synchronous call. AT_REMOVEDIR requests removal of an empty directory.
    unsafe {
        syscall6(
            SYS_UNLINKAT,
            AT_FDCWD as usize,
            path.as_ptr() as usize,
            AT_REMOVEDIR,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn getdents64(fd: i32, buffer: &mut [u8]) -> isize {
    // SAFETY: `buffer` is uniquely borrowed and writable for its complete length
    // until the synchronous syscall returns; fd and length are scalar arguments.
    unsafe {
        syscall6(
            SYS_GETDENTS64,
            fd as usize,
            buffer.as_mut_ptr() as usize,
            buffer.len(),
            0,
            0,
            0,
        )
    }
}

fn directory_contains(fd: i32, expected_name: &[u8]) -> bool {
    let mut buffer = [0_u8; 512];
    loop {
        let count = getdents64(fd, &mut buffer);
        if count < 0 || count as usize > buffer.len() {
            return false;
        }
        if count == 0 {
            return false;
        }
        let count = count as usize;
        let mut offset = 0_usize;
        while offset < count {
            if count - offset < 20 {
                return false;
            }
            let Some(record_len_offset) = offset.checked_add(16) else {
                return false;
            };
            let Some(record_len_low) = buffer.get(record_len_offset).copied() else {
                return false;
            };
            let Some(record_len_high) = buffer.get(record_len_offset + 1).copied() else {
                return false;
            };
            let record_len = u16::from_ne_bytes([record_len_low, record_len_high]) as usize;
            if record_len < 20 || record_len > count - offset {
                return false;
            }
            let Some(name_start) = offset.checked_add(19) else {
                return false;
            };
            let Some(record_end) = offset.checked_add(record_len) else {
                return false;
            };
            let mut cursor = name_start;
            let mut name_len = 0_usize;
            let mut name_matches = true;
            let mut terminated = false;
            while cursor < record_end {
                let Some(byte) = buffer.get(cursor).copied() else {
                    return false;
                };
                if byte == 0 {
                    terminated = true;
                    break;
                }
                if expected_name.get(name_len).copied() != Some(byte) {
                    name_matches = false;
                }
                name_len = name_len.saturating_add(1);
                cursor = cursor.saturating_add(1);
            }
            if !terminated {
                return false;
            }
            if name_matches && name_len == expected_name.len() {
                return true;
            }
            offset = record_end;
        }
    }
}

fn statx_identity(path: &[u8]) -> Option<StatIdentity> {
    // Linux's statx structure is 256 bytes and naturally 8-byte aligned. The nlink
    // and inode fields occupy stable offsets in the UAPI layout.
    let mut statx = [0_u64; 32];
    // SAFETY: `path` is a live NUL-terminated pathname and `statx` is an aligned,
    // uniquely borrowed writable 256-byte output object for this synchronous call.
    let result = unsafe {
        syscall6(
            SYS_STATX,
            AT_FDCWD as usize,
            path.as_ptr() as usize,
            0,
            STATX_BASIC_STATS,
            statx.as_mut_ptr() as usize,
            0,
        )
    };
    let mask = statx[0] as u32;
    if result != 0 || mask & STATX_NLINK == 0 {
        return None;
    }
    Some(StatIdentity {
        nlink: (statx[2] & u32::MAX as u64) as u32,
        inode: statx[4],
    })
}

#[inline(always)]
fn linkat(old_path: &[u8], new_path: &[u8]) -> isize {
    // SAFETY: both slices are readable NUL-terminated pathnames that remain live for
    // the synchronous syscall. AT_FDCWD selects the current directory namespace for
    // each absolute pathname, and zero flags request an ordinary hard link.
    unsafe {
        syscall6(
            SYS_LINKAT,
            AT_FDCWD as usize,
            old_path.as_ptr() as usize,
            AT_FDCWD as usize,
            new_path.as_ptr() as usize,
            0,
            0,
        )
    }
}

#[inline(always)]
fn renameat2(old_path: &[u8], new_path: &[u8]) -> isize {
    // SAFETY: both slices are readable NUL-terminated pathnames for the complete
    // synchronous call. AT_FDCWD and zero flags request ordinary replacement rename.
    unsafe {
        syscall6(
            SYS_RENAMEAT2,
            AT_FDCWD as usize,
            old_path.as_ptr() as usize,
            AT_FDCWD as usize,
            new_path.as_ptr() as usize,
            0,
            0,
        )
    }
}

fn path_has_exact_data(path: &[u8], expected: &[u8]) -> bool {
    let fd = openat(path, O_RDONLY);
    if fd < 0 {
        return false;
    }
    let mut data = [0_u8; 8];
    let read_result = fd_read(fd as i32, &mut data);
    let close_result = close(fd as i32);
    read_result == expected.len() as isize
        && data[..expected.len()] == *expected
        && close_result == 0
}

fn loopback_sockaddr() -> [u8; 16] {
    let mut address = [0_u8; 16];
    let family = (AF_INET as u16).to_ne_bytes();
    let port = TCP_FORK_PORT.to_be_bytes();
    address[..2].copy_from_slice(&family);
    address[2..4].copy_from_slice(&port);
    address[4..8].copy_from_slice(&[127, 0, 0, 1]);
    address
}

#[inline(always)]
fn socket_stream() -> isize {
    // SAFETY: socket consumes only scalar domain, type, and protocol values. AF_INET
    // plus SOCK_STREAM and protocol zero requests an ordinary IPv4 TCP socket.
    unsafe { syscall6(SYS_SOCKET, AF_INET, SOCK_STREAM, 0, 0, 0, 0) }
}

#[inline(always)]
fn socket_set_reuseaddr(fd: i32) -> isize {
    let enabled = 1_i32;
    // SAFETY: `enabled` is an aligned readable i32 for the complete synchronous
    // setsockopt call; all remaining arguments are bounded scalar values.
    unsafe {
        syscall6(
            SYS_SETSOCKOPT,
            fd as usize,
            SOL_SOCKET,
            SO_REUSEADDR,
            &enabled as *const i32 as usize,
            core::mem::size_of::<i32>(),
            0,
        )
    }
}

#[inline(always)]
fn socket_bind(fd: i32, address: &[u8; 16]) -> isize {
    // SAFETY: `address` contains a complete Linux sockaddr_in byte layout and remains
    // readable until the synchronous bind call returns.
    unsafe {
        syscall6(
            SYS_BIND,
            fd as usize,
            address.as_ptr() as usize,
            address.len(),
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn socket_listen(fd: i32, backlog: usize) -> isize {
    // SAFETY: listen consumes only the scalar descriptor and backlog.
    unsafe { syscall6(SYS_LISTEN, fd as usize, backlog, 0, 0, 0, 0) }
}

#[inline(always)]
fn socket_accept(fd: i32) -> isize {
    // SAFETY: null address and length pointers explicitly decline peer-address output;
    // accept consumes only the live listener descriptor.
    unsafe { syscall6(SYS_ACCEPT, fd as usize, 0, 0, 0, 0, 0) }
}

#[inline(always)]
fn socket_connect(fd: i32, address: &[u8; 16]) -> isize {
    // SAFETY: `address` contains a complete Linux sockaddr_in byte layout and remains
    // readable until the synchronous connect call returns.
    unsafe {
        syscall6(
            SYS_CONNECT,
            fd as usize,
            address.as_ptr() as usize,
            address.len(),
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn socket_send(fd: i32, bytes: &[u8]) -> isize {
    // SAFETY: `bytes` remains readable for its complete length until sendto returns;
    // null destination arguments select the already-connected stream peer.
    unsafe {
        syscall6(
            SYS_SENDTO,
            fd as usize,
            bytes.as_ptr() as usize,
            bytes.len(),
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn socket_recv(fd: i32, bytes: &mut [u8]) -> isize {
    // SAFETY: `bytes` is uniquely borrowed and writable for its complete length until
    // recvfrom returns; null source arguments decline peer-address output.
    unsafe {
        syscall6(
            SYS_RECVFROM,
            fd as usize,
            bytes.as_mut_ptr() as usize,
            bytes.len(),
            0,
            0,
            0,
        )
    }
}

fn socket_send_all(fd: i32, bytes: &[u8]) -> bool {
    let mut sent = 0usize;
    while sent < bytes.len() {
        let result = socket_send(fd, &bytes[sent..]);
        if result <= 0 || result as usize > bytes.len() - sent {
            return false;
        }
        sent += result as usize;
    }
    true
}

fn socket_recv_exact(fd: i32, bytes: &mut [u8]) -> bool {
    let mut received = 0usize;
    while received < bytes.len() {
        let result = socket_recv(fd, &mut bytes[received..]);
        if result <= 0 || result as usize > bytes.len() - received {
            return false;
        }
        received += result as usize;
    }
    true
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CloneArgs {
    flags: u64,
    pidfd: u64,
    child_tid: u64,
    parent_tid: u64,
    exit_signal: u64,
    stack: u64,
    stack_size: u64,
    tls: u64,
    set_tid: u64,
    set_tid_size: u64,
    cgroup: u64,
}

impl CloneArgs {
    const fn fork() -> Self {
        Self {
            flags: 0,
            pidfd: 0,
            child_tid: 0,
            parent_tid: 0,
            exit_signal: SIGCHLD as u64,
            stack: 0,
            stack_size: 0,
            tls: 0,
            set_tid: 0,
            set_tid_size: 0,
            cgroup: 0,
        }
    }

    const fn cargo_thread(stack: usize, parent_and_child_tid: usize, tls: usize) -> Self {
        Self {
            flags: CARGO_THREAD_CLONE_FLAGS,
            pidfd: 0,
            child_tid: parent_and_child_tid as u64,
            parent_tid: parent_and_child_tid as u64,
            exit_signal: 0,
            stack: stack as u64,
            stack_size: CLONE3_THREAD_STACK_BYTES as u64,
            tls: tls as u64,
            set_tid: 0,
            set_tid_size: 0,
            cgroup: 0,
        }
    }

    const fn vfork(stack: usize) -> Self {
        Self {
            flags: CLONE_VM | CLONE_VFORK,
            pidfd: 0,
            child_tid: 0,
            parent_tid: 0,
            exit_signal: SIGCHLD as u64,
            stack: stack as u64,
            stack_size: CLONE3_VFORK_STACK_BYTES as u64,
            tls: 0,
            set_tid: 0,
            set_tid_size: 0,
            cgroup: 0,
        }
    }
}

#[repr(C)]
struct ExtendedCloneArgs {
    args: CloneArgs,
    future_field: u64,
}

#[inline(always)]
fn clone3_process(args: *const CloneArgs, size: usize) -> isize {
    // SAFETY: callers keep the complete `size` byte range readable until the
    // synchronous clone3 entry has copied it. Null is used only for the EFAULT probe.
    unsafe { syscall6(SYS_CLONE3, args as usize, size, 0, 0, 0, 0) }
}

#[repr(C)]
struct VforkExecContext {
    stdout_read_fd: usize,
    stdout_write_fd: usize,
    path: usize,
    argv: usize,
    envp: usize,
    stage: usize,
}

#[cfg(target_arch = "riscv64")]
#[inline(never)]
/// Enters clone3 with glibc posix_spawn's vfork and explicit-stack shape.
///
/// # Safety
///
/// `args` and `context` must remain readable until the parent syscall returns.
/// The stack described by `args` must be writable and exclusively owned by the
/// child. Every context pointer and descriptor must remain valid in the inherited
/// child image. The child path uses only its new stack and raw syscalls, then
/// either successfully execs or exits without returning to Rust.
unsafe fn clone3_vfork_exec(args: *const CloneArgs, context: *const VforkExecContext) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the pointer, fd, and stack contract above. The
    // parent follows the ordinary ABI return path; the child never returns through
    // a Rust frame after the kernel installs its explicit stack.
    unsafe {
        core::arch::asm!(
            "ecall",
            "bnez a0, 3f",
            "addi sp, sp, -16",
            "sd a2, 0(sp)",
            "ld t0, 0(sp)",
            "ld a0, 0(t0)",
            "li a7, 57",
            "ecall",
            "bnez a0, 7f",
            "ld t0, 0(sp)",
            "ld a0, 8(t0)",
            "li a1, 1",
            "beq a0, a1, 5f",
            "li a2, 0",
            "li a7, 24",
            "ecall",
            "bltz a0, 7f",
            "ld t0, 0(sp)",
            "ld a0, 8(t0)",
            "li a7, 57",
            "ecall",
            "bnez a0, 7f",
            "5:",
            "ld t0, 0(sp)",
            "ld t0, 40(t0)",
            "li a0, 1",
            "fence rw, w",
            "sw a0, 0(t0)",
            "ld t0, 0(sp)",
            "ld a0, 16(t0)",
            "ld a1, 24(t0)",
            "ld a2, 32(t0)",
            "li a7, 221",
            "ecall",
            "7:",
            "li a0, 44",
            "li a7, 93",
            "ecall",
            "8:",
            "j 8b",
            "3:",
            inlateout("a0") args as usize => ret,
            inlateout("a1") core::mem::size_of::<CloneArgs>() => _,
            inlateout("a2") context as usize => _,
            inlateout("a7") SYS_CLONE3 => _,
            lateout("a3") _,
            lateout("a4") _,
            lateout("a5") _,
            lateout("a6") _,
            lateout("t0") _,
        );
    }
    ret
}

#[cfg(target_arch = "loongarch64")]
#[inline(never)]
/// Enters clone3 with glibc posix_spawn's vfork and explicit-stack shape.
///
/// # Safety
///
/// This has the same pointer, descriptor, stack-ownership, and no-return child
/// contract as the RISC-V64 implementation above. The child uses only raw LA64
/// Linux syscalls after clone3 changes `$sp`.
unsafe fn clone3_vfork_exec(args: *const CloneArgs, context: *const VforkExecContext) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the documented raw ABI contract. The child
    // performs no Rust call or return after switching to its explicit stack.
    unsafe {
        core::arch::asm!(
            "syscall 0",
            "bnez $a0, 3f",
            "addi.d $sp, $sp, -16",
            "st.d $a2, $sp, 0",
            "ld.d $t0, $sp, 0",
            "ld.d $a0, $t0, 0",
            "addi.d $a7, $zero, 57",
            "syscall 0",
            "bnez $a0, 7f",
            "ld.d $t0, $sp, 0",
            "ld.d $a0, $t0, 8",
            "addi.d $a1, $zero, 1",
            "beq $a0, $a1, 5f",
            "or $a2, $zero, $zero",
            "addi.d $a7, $zero, 24",
            "syscall 0",
            "blt $a0, $zero, 7f",
            "ld.d $t0, $sp, 0",
            "ld.d $a0, $t0, 8",
            "addi.d $a7, $zero, 57",
            "syscall 0",
            "bnez $a0, 7f",
            "5:",
            "ld.d $t0, $sp, 0",
            "ld.d $t0, $t0, 40",
            "addi.d $a0, $zero, 1",
            "dbar 0",
            "st.w $a0, $t0, 0",
            "ld.d $t0, $sp, 0",
            "ld.d $a0, $t0, 16",
            "ld.d $a1, $t0, 24",
            "ld.d $a2, $t0, 32",
            "addi.d $a7, $zero, 221",
            "syscall 0",
            "7:",
            "addi.d $a0, $zero, 44",
            "addi.d $a7, $zero, 93",
            "syscall 0",
            "8:",
            "b 8b",
            "3:",
            inlateout("$a0") args as usize => ret,
            inlateout("$a1") core::mem::size_of::<CloneArgs>() => _,
            inlateout("$a2") context as usize => _,
            inlateout("$a7") SYS_CLONE3 => _,
            lateout("$a3") _,
            lateout("$a4") _,
            lateout("$a5") _,
            lateout("$a6") _,
            lateout("$t0") _,
        );
    }
    ret
}

#[cfg(target_arch = "riscv64")]
#[inline(never)]
/// Enters clone3 with Cargo/glibc's worker-thread register shape.
///
/// # Safety
///
/// `args` must remain readable through syscall entry and describe a writable,
/// exclusively owned child stack. `ready_write_fd` and `release_read_fd` must be
/// opposite ends of live pipes shared with the new thread. `child_marker` must
/// remain readable until the child exits. The child path never returns to Rust:
/// after the kernel switches `sp`, the assembly exchanges one-byte pipe messages,
/// reports its TLS comparison, writes the marker, and invokes `exit(2)` directly.
unsafe fn clone3_cargo_thread(
    args: *const CloneArgs,
    ready_write_fd: i32,
    release_read_fd: i32,
    child_marker: &[u8],
    expected_tls: usize,
) -> isize {
    let ret: isize;
    // SAFETY: the caller provides the live pointers, descriptors, and exclusive
    // stack described above. The parent follows the ordinary ABI return path. The
    // child uses only its new stack and raw syscalls and therefore never lets Rust
    // observe a changed stack pointer or return through the parent's call frame.
    unsafe {
        core::arch::asm!(
            "ecall",
            "bnez a0, 3f",
            "addi sp, sp, -48",
            "sd a2, 8(sp)",
            "sd a3, 16(sp)",
            "sd a4, 24(sp)",
            "sd a5, 32(sp)",
            "li t0, 70",
            "bne tp, a6, 5f",
            "li t0, 82",
            "5:",
            "sb t0, 40(sp)",
            "sb t0, 0(sp)",
            "ld a0, 8(sp)",
            "mv a1, sp",
            "li a2, 1",
            "li a7, 64",
            "ecall",
            "ld a0, 16(sp)",
            "mv a1, sp",
            "li a2, 1",
            "li a7, 63",
            "ecall",
            "lbu t0, 40(sp)",
            "li a0, 82",
            "bne t0, a0, 7f",
            "li a0, 1",
            "ld a1, 24(sp)",
            "ld a2, 32(sp)",
            "li a7, 64",
            "ecall",
            "j 8f",
            "7:",
            "li a0, -1",
            "8:",
            "sd a0, 0(sp)",
            "ld a0, 8(sp)",
            "mv a1, sp",
            "li a2, 8",
            "li a7, 64",
            "ecall",
            "li a0, 0",
            "li a7, 93",
            "ecall",
            "6:",
            "j 6b",
            "3:",
            inlateout("a0") args as usize => ret,
            inlateout("a1") core::mem::size_of::<CloneArgs>() => _,
            inlateout("a2") ready_write_fd as usize => _,
            inlateout("a3") release_read_fd as usize => _,
            inlateout("a4") child_marker.as_ptr() as usize => _,
            inlateout("a5") child_marker.len() => _,
            inlateout("a6") expected_tls => _,
            inlateout("a7") SYS_CLONE3 => _,
            lateout("t0") _,
        );
    }
    ret
}

#[cfg(target_arch = "loongarch64")]
#[inline(never)]
/// Enters clone3 with Cargo/glibc's worker-thread register shape.
///
/// # Safety
///
/// This has the same pointer, descriptor, stack-ownership, and no-return child
/// contract as the RISC-V64 implementation above. The architecture-specific block
/// uses only the Linux LA64 syscall ABI and never resumes Rust on the child stack.
unsafe fn clone3_cargo_thread(
    args: *const CloneArgs,
    ready_write_fd: i32,
    release_read_fd: i32,
    child_marker: &[u8],
    expected_tls: usize,
) -> isize {
    let ret: isize;
    // SAFETY: the caller upholds the documented raw ABI and ownership contract.
    // The child performs no Rust call or return after clone3 changes `$sp`.
    unsafe {
        core::arch::asm!(
            "syscall 0",
            "bnez $a0, 3f",
            "addi.d $sp, $sp, -48",
            "st.d $a2, $sp, 8",
            "st.d $a3, $sp, 16",
            "st.d $a4, $sp, 24",
            "st.d $a5, $sp, 32",
            "addi.d $t0, $zero, 70",
            "bne $tp, $a6, 5f",
            "addi.d $t0, $zero, 82",
            "5:",
            "st.b $t0, $sp, 40",
            "st.b $t0, $sp, 0",
            "ld.d $a0, $sp, 8",
            "or $a1, $sp, $zero",
            "addi.d $a2, $zero, 1",
            "addi.d $a7, $zero, 64",
            "syscall 0",
            "ld.d $a0, $sp, 16",
            "or $a1, $sp, $zero",
            "addi.d $a2, $zero, 1",
            "addi.d $a7, $zero, 63",
            "syscall 0",
            "ld.bu $t0, $sp, 40",
            "addi.d $a0, $zero, 82",
            "bne $t0, $a0, 7f",
            "addi.d $a0, $zero, 1",
            "ld.d $a1, $sp, 24",
            "ld.d $a2, $sp, 32",
            "addi.d $a7, $zero, 64",
            "syscall 0",
            "b 8f",
            "7:",
            "addi.d $a0, $zero, -1",
            "8:",
            "st.d $a0, $sp, 0",
            "ld.d $a0, $sp, 8",
            "or $a1, $sp, $zero",
            "addi.d $a2, $zero, 8",
            "addi.d $a7, $zero, 64",
            "syscall 0",
            "or $a0, $zero, $zero",
            "addi.d $a7, $zero, 93",
            "syscall 0",
            "6:",
            "b 6b",
            "3:",
            inlateout("$a0") args as usize => ret,
            inlateout("$a1") core::mem::size_of::<CloneArgs>() => _,
            inlateout("$a2") ready_write_fd as usize => _,
            inlateout("$a3") release_read_fd as usize => _,
            inlateout("$a4") child_marker.as_ptr() as usize => _,
            inlateout("$a5") child_marker.len() => _,
            inlateout("$a6") expected_tls => _,
            inlateout("$a7") SYS_CLONE3 => _,
            lateout("$t0") _,
        );
    }
    ret
}

#[inline(always)]
fn futex_wait_clear_tid(tid: &AtomicI32, expected_tid: i32) -> isize {
    // SAFETY: `tid` is an aligned, live i32-sized atomic shared with the clone3
    // child through CLONE_VM. FUTEX_WAIT_BITSET reads it only for the duration of
    // this synchronous call. A null timeout plus MATCH_ANY is the exact glibc
    // clear-child-tid join shape observed for Cargo worker threads.
    unsafe {
        syscall6(
            SYS_FUTEX,
            tid.as_ptr() as usize,
            FUTEX_WAIT_BITSET | FUTEX_CLOCK_REALTIME,
            expected_tid as u32 as usize,
            0,
            0,
            FUTEX_BITSET_MATCH_ANY,
        )
    }
}

#[inline(always)]
fn mmap_private_anonymous(len: usize) -> isize {
    // SAFETY: this requests a new kernel-selected private anonymous mapping. There
    // is no file backing or userspace input pointer; the returned raw address is
    // checked before the test constructs pointers within the mapped byte range.
    unsafe {
        syscall6(
            SYS_MMAP,
            0,
            len,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            usize::MAX,
            0,
        )
    }
}

#[inline(always)]
fn madvise_dontneed(addr: usize, len: usize) -> isize {
    // SAFETY: callers keep the complete page-aligned mapping live throughout this
    // synchronous syscall. The remaining arguments are scalar Linux ABI values.
    unsafe { syscall6(SYS_MADVISE, addr, len, MADV_DONTNEED, 0, 0, 0) }
}

#[inline(always)]
fn munmap(addr: usize, len: usize) -> isize {
    // SAFETY: callers pass the exact base and length of a live standalone mapping
    // after their last pointer access; no Rust reference survives this call.
    unsafe { syscall6(SYS_MUNMAP, addr, len, 0, 0, 0, 0) }
}

fn monotonic_nanoseconds() -> Option<u64> {
    let mut now = Timespec {
        seconds: 0,
        nanoseconds: 0,
    };
    // SAFETY: `now` is uniquely borrowed, aligned, and writable for one complete
    // Linux timespec until the synchronous clock_gettime call returns.
    if unsafe {
        syscall6(
            SYS_CLOCK_GETTIME,
            CLOCK_MONOTONIC,
            &mut now as *mut Timespec as usize,
            0,
            0,
            0,
            0,
        )
    } != 0
        || now.seconds < 0
        || !(0..1_000_000_000).contains(&now.nanoseconds)
    {
        return None;
    }
    (now.seconds as u64)
        .checked_mul(1_000_000_000)?
        .checked_add(now.nanoseconds as u64)
}

fn report_madvise_elapsed_nanoseconds(value: u64) {
    const PREFIX: &[u8] = b"PR3_SMOKE_V1 DIAG madvise_dontneed_elapsed_ns=0x";
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = [b'0'; core::mem::size_of::<u64>() * 2 + 1];
    for (index, digit) in encoded[..core::mem::size_of::<u64>() * 2]
        .iter_mut()
        .enumerate()
    {
        let shift = (core::mem::size_of::<u64>() * 2 - index - 1) * 4;
        *digit = HEX[((value >> shift) & 0xf) as usize];
    }
    encoded[encoded.len() - 1] = b'\n';
    let _ = write(PREFIX);
    let _ = write(&encoded);
}

fn report_clone3_thread_write_result(value: isize) {
    const PREFIX: &[u8] = b"PR3_SMOKE_V1 DIAG clone3_thread_write_result=0x";
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = [b'0'; core::mem::size_of::<usize>() * 2 + 1];
    let value = value as usize;
    for (index, digit) in encoded[..core::mem::size_of::<usize>() * 2]
        .iter_mut()
        .enumerate()
    {
        let shift = (core::mem::size_of::<usize>() * 2 - index - 1) * 4;
        *digit = HEX[(value >> shift) & 0xf];
    }
    encoded[encoded.len() - 1] = b'\n';
    let _ = write(PREFIX);
    let _ = write(&encoded);
}

#[inline(always)]
fn fork_process() -> isize {
    // SAFETY: SIGCHLD with a null child stack and no clone flags requests ordinary
    // fork-like process creation. All optional user pointers are null.
    unsafe { syscall6(SYS_CLONE, SIGCHLD, 0, 0, 0, 0, 0) }
}

#[inline(always)]
fn wait_child(pid: isize, status: &mut i32) -> isize {
    // SAFETY: `status` is uniquely borrowed and writable for one i32 until wait4
    // returns. The exact positive pid selects one child; options and rusage are zero.
    unsafe {
        syscall6(
            SYS_WAIT4,
            pid as usize,
            status as *mut i32 as usize,
            0,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn wait_child_nohang(pid: isize, status: &mut i32) -> isize {
    // SAFETY: `status` remains uniquely writable for the synchronous wait4 call;
    // WNOHANG requests an immediate observation of this exact positive child pid.
    unsafe {
        syscall6(
            SYS_WAIT4,
            pid as usize,
            status as *mut i32 as usize,
            WNOHANG,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn nanosleep(request: &Timespec) -> isize {
    // SAFETY: `request` is aligned and readable for the complete syscall. A null
    // remainder pointer explicitly declines the optional interrupted duration.
    unsafe {
        syscall6(
            SYS_NANOSLEEP,
            request as *const Timespec as usize,
            0,
            0,
            0,
            0,
            0,
        )
    }
}

#[inline(always)]
fn exec_helper() -> isize {
    let argv = [EXEC_HELPER_PATH.as_ptr() as usize, 0];
    let envp = [0_usize];
    // SAFETY: the path is NUL-terminated and readable; argv/envp are live arrays of
    // readable pointers terminated by null. Successful execve never returns.
    unsafe {
        syscall6(
            SYS_EXECVE,
            EXEC_HELPER_PATH.as_ptr() as usize,
            argv.as_ptr() as usize,
            envp.as_ptr() as usize,
            0,
            0,
            0,
        )
    }
}

fn tcp_fork_child(listener: i32, client_index: usize, address: &[u8; 16]) -> usize {
    if close(listener) != 0 {
        return 1;
    }
    let client = socket_stream();
    if client < 0 {
        return 2;
    }
    let client = client as i32;
    if socket_connect(client, address) != 0 {
        let _ = close(client);
        return 3;
    }
    let request = [b'C', client_index as u8, 0x5a, 0xa5];
    if !socket_send_all(client, &request) {
        let _ = close(client);
        return 4;
    }
    let mut response = [0_u8; 4];
    if !socket_recv_exact(client, &mut response) || response != request {
        let _ = close(client);
        return 5;
    }
    if close(client) != 0 {
        return 6;
    }
    0
}

#[inline(always)]
fn exit(code: usize) -> ! {
    // SAFETY: SYS_EXIT consumes only the scalar exit code; its other raw arguments are
    // ignored. If a defective kernel returns, the fallback loop preserves `!`.
    unsafe {
        let _ = syscall6(SYS_EXIT, code, 0, 0, 0, 0, 0);
    }
    loop {
        core::hint::spin_loop();
    }
}

#[inline(never)]
fn fail(marker: &[u8], code: usize) -> ! {
    let _ = write(marker);
    exit(code)
}

fn c_field_equals(field: &[u8; 65], expected: &[u8]) -> bool {
    expected.len() < field.len()
        && &field[..expected.len()] == expected
        && field[expected.len()] == 0
}

fn expected_vmsplice_byte(offset: usize) -> u8 {
    if offset < VMSPLICE_FIRST_LEN {
        0x3c
    } else {
        0xa5
    }
}

// SAFETY: this freestanding ELF provides the sole `memset` definition selected by its
// private linker script, with the C ABI/signature rustc expects for compiler lowering.
#[unsafe(no_mangle)]
/// Fills a caller-provided byte range for compiler-generated freestanding code.
///
/// # Safety
///
/// For `count > 0`, `destination` must be non-null and valid for writes of `count`
/// bytes, and no concurrent or aliased access may violate Rust's memory rules for that
/// range. A zero count performs no pointer arithmetic or dereference.
unsafe extern "C" fn memset(destination: *mut u8, value: i32, count: usize) -> *mut u8 {
    for offset in 0..count {
        // SAFETY: the function contract makes the complete range writable, and
        // `offset < count` keeps this byte within that range (`u8` has alignment 1).
        unsafe {
            destination.add(offset).write_volatile(value as u8);
        }
    }
    destination
}

// SAFETY: this freestanding ELF provides the sole `memcmp` definition selected by its
// private linker script, with the C ABI/signature rustc expects for compiler lowering.
#[unsafe(no_mangle)]
/// Compares two caller-provided byte ranges for compiler-generated freestanding code.
///
/// # Safety
///
/// For `count > 0`, both pointers must be non-null and valid for reads of `count`
/// bytes, and neither range may be mutated concurrently. The ranges may overlap because
/// this function only reads them. A zero count performs no pointer arithmetic or
/// dereference.
unsafe extern "C" fn memcmp(left: *const u8, right: *const u8, count: usize) -> i32 {
    for offset in 0..count {
        // SAFETY: the function contract makes both complete ranges readable, and
        // `offset < count` keeps each byte within its range (`u8` has alignment 1).
        let left_byte = unsafe { left.add(offset).read_volatile() };
        let right_byte = unsafe { right.add(offset).read_volatile() };
        if left_byte != right_byte {
            return left_byte as i32 - right_byte as i32;
        }
    }
    0
}

// SAFETY: the private linker script selects this sole `_start` symbol as the entry of
// the freestanding smoke ELF; its C ABI and non-returning signature match that role.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if write(USER_START) != USER_START.len() as isize {
        fail(USER_FAIL_WRITE, 101);
    }
    if write(ASSERT_WRITE) != ASSERT_WRITE.len() as isize {
        fail(USER_FAIL_WRITE, 102);
    }

    // SAFETY: SYS_GETPID has no pointer arguments; all three raw argument slots are
    // ignored, and the returned scalar is checked before use.
    let pid = unsafe { syscall6(SYS_GETPID, 0, 0, 0, 0, 0, 0) };
    if pid <= 0 {
        fail(USER_FAIL_GETPID, 103);
    }
    if write(ASSERT_GETPID) != ASSERT_GETPID.len() as isize {
        fail(USER_FAIL_WRITE, 104);
    }

    // The final-2026 workloads run with eight online CPUs. Linux returns the
    // kernel cpumask width (one unsigned long here), copies only that many bytes,
    // and reports the live task mask. Pinning the caller to CPU 7 and restoring
    // the full mask proves setaffinity changes scheduler state rather than merely
    // acknowledging the request. Sentinel bytes beyond the returned width must
    // remain untouched.
    let mut affinity = [0xa5_u8; AFFINITY_BUFFER_BYTES];
    if sched_getaffinity(&mut affinity) != CPUSET_BYTES as isize {
        fail(USER_FAIL_SCHED_AFFINITY_SYSCALL, 235);
    }
    if !affinity_snapshot_matches(&affinity, 0xff) {
        fail(USER_FAIL_SCHED_AFFINITY_MASK, 236);
    }
    let mut cpu_seven = [0_u8; CPUSET_BYTES];
    cpu_seven[0] = 0x80;
    if sched_setaffinity(&cpu_seven) != 0 {
        fail(USER_FAIL_SCHED_AFFINITY_SYSCALL, 237);
    }
    affinity = [0xa5; AFFINITY_BUFFER_BYTES];
    if sched_getaffinity(&mut affinity) != CPUSET_BYTES as isize
        || !affinity_snapshot_matches(&affinity, 0x80)
    {
        fail(USER_FAIL_SCHED_AFFINITY_MASK, 238);
    }
    let mut all_cpus = [0_u8; CPUSET_BYTES];
    all_cpus[0] = 0xff;
    if sched_setaffinity(&all_cpus) != 0 {
        fail(USER_FAIL_SCHED_AFFINITY_SYSCALL, 239);
    }
    affinity = [0xa5; AFFINITY_BUFFER_BYTES];
    if sched_getaffinity(&mut affinity) != CPUSET_BYTES as isize
        || !affinity_snapshot_matches(&affinity, 0xff)
    {
        fail(USER_FAIL_SCHED_AFFINITY_MASK, 240);
    }
    if write(ASSERT_SCHED_AFFINITY) != ASSERT_SCHED_AFFINITY.len() as isize {
        fail(USER_FAIL_WRITE, 241);
    }

    let (uptime_before, _) = match read_proc_uptime() {
        Ok(uptime) => uptime,
        Err(error) => fail_proc_uptime(error, 230),
    };
    let sleep_request = Timespec {
        seconds: 0,
        nanoseconds: 150_000_000,
    };
    // SAFETY: `sleep_request` is a live, aligned, readable Linux timespec for the
    // complete synchronous SYS_NANOSLEEP call. A null remainder pointer explicitly
    // declines the optional remaining-duration result; all other slots are ignored.
    if unsafe {
        syscall6(
            SYS_NANOSLEEP,
            &sleep_request as *const Timespec as usize,
            0,
            0,
            0,
            0,
            0,
        )
    } != 0
    {
        fail(USER_FAIL_PROC_UPTIME_SLEEP, 231);
    }
    let (uptime_after, _) = match read_proc_uptime() {
        Ok(uptime) => uptime,
        Err(error) => fail_proc_uptime(error, 232),
    };
    if uptime_after <= uptime_before {
        fail(USER_FAIL_PROC_UPTIME_ADVANCE, 233);
    }
    if write(ASSERT_PROC_UPTIME) != ASSERT_PROC_UPTIME.len() as isize {
        fail(USER_FAIL_WRITE, 234);
    }

    if let Err(error) = read_proc_statm() {
        fail_proc_statm(error, 242);
    }
    if write(ASSERT_PROC_STATM) != ASSERT_PROC_STATM.len() as isize {
        fail(USER_FAIL_WRITE, 243);
    }

    // Cargo/rustc allocators repeatedly discard private anonymous MAP_NORESERVE
    // ranges with MADV_DONTNEED. Verify the Linux-visible contract without relying
    // on allocator internals: each populated page becomes zero, can be faulted and
    // written again, and the mapping remains valid for all iterations. Only time
    // spent inside madvise is reported, so later page-discard optimizations can be
    // compared without turning a host-dependent duration into a PASS condition.
    let madvise_mapping = mmap_private_anonymous(MADVISE_PROBE_BYTES);
    if madvise_mapping < 0 || madvise_mapping as usize & (PAGE_BYTES - 1) != 0 {
        fail(USER_FAIL_MADVISE_DONTNEED, 272);
    }
    let madvise_mapping = madvise_mapping as usize;
    let mut madvise_elapsed_ns = 0_u64;
    for iteration in 0..MADVISE_PROBE_ITERATIONS {
        for offset in (0..MADVISE_PROBE_BYTES).step_by(PAGE_BYTES) {
            // SAFETY: mmap returned this complete writable range, `offset` stays on
            // a page start within it, and no reference is kept across madvise.
            unsafe {
                ((madvise_mapping + offset) as *mut u8)
                    .write_volatile((iteration as u8).wrapping_add(1));
            }
        }
        let before = monotonic_nanoseconds().unwrap_or_else(|| {
            let _ = munmap(madvise_mapping, MADVISE_PROBE_BYTES);
            fail(USER_FAIL_MADVISE_DONTNEED, 273)
        });
        let advice_result = madvise_dontneed(madvise_mapping, MADVISE_PROBE_BYTES);
        let after = monotonic_nanoseconds().unwrap_or_else(|| {
            let _ = munmap(madvise_mapping, MADVISE_PROBE_BYTES);
            fail(USER_FAIL_MADVISE_DONTNEED, 274)
        });
        let Some(elapsed) = after.checked_sub(before) else {
            let _ = munmap(madvise_mapping, MADVISE_PROBE_BYTES);
            fail(USER_FAIL_MADVISE_DONTNEED, 275);
        };
        let Some(total) = madvise_elapsed_ns.checked_add(elapsed) else {
            let _ = munmap(madvise_mapping, MADVISE_PROBE_BYTES);
            fail(USER_FAIL_MADVISE_DONTNEED, 276);
        };
        madvise_elapsed_ns = total;
        if advice_result != 0 {
            let _ = munmap(madvise_mapping, MADVISE_PROBE_BYTES);
            fail(USER_FAIL_MADVISE_DONTNEED, 277);
        }
        for offset in (0..MADVISE_PROBE_BYTES).step_by(PAGE_BYTES) {
            // SAFETY: the successful MADV_DONTNEED preserves the mapping and read
            // permission. Reading one byte per page checks zero-fill and faults in
            // discarded pages without creating a reference that outlives the read.
            if unsafe {
                ((madvise_mapping + offset) as *const u8).read_volatile()
            } != 0
            {
                let _ = munmap(madvise_mapping, MADVISE_PROBE_BYTES);
                fail(USER_FAIL_MADVISE_DONTNEED, 278);
            }
        }
    }
    // After fork, the child owns a COW view of this private mapping. Discarding
    // the child's resident page must produce a private zero-filled page without
    // changing the parent's retained byte; writing the replacement page must
    // remain isolated as well. This covers the allocation-backed and cloned
    // shared-metadata discard paths used by ordinary processes.
    // SAFETY: the first byte is inside the still-live writable mapping, and the
    // volatile access does not create a reference that crosses fork or madvise.
    unsafe { (madvise_mapping as *mut u8).write_volatile(0x5a) };
    let madvise_child = fork_process();
    if madvise_child == 0 {
        if madvise_dontneed(madvise_mapping, PAGE_BYTES) != 0
            // SAFETY: the child retains its private readable mapping after fork
            // and successful madvise; this single volatile read stays in-page.
            || unsafe { (madvise_mapping as *const u8).read_volatile() } != 0
        {
            exit(41);
        }
        // SAFETY: the discarded private page has been faulted back into the
        // child's writable mapping and remains live until the immediate exit.
        unsafe { (madvise_mapping as *mut u8).write_volatile(0xa5) };
        exit(0);
    }
    if madvise_child < 0 {
        let _ = munmap(madvise_mapping, MADVISE_PROBE_BYTES);
        fail(USER_FAIL_MADVISE_DONTNEED, 281);
    }
    let mut madvise_child_status = -1_i32;
    if wait_child(madvise_child, &mut madvise_child_status) != madvise_child
        || madvise_child_status != 0
        // SAFETY: the parent's original private page remains mapped and readable;
        // neither the child's discard nor its later write may change this byte.
        || unsafe { (madvise_mapping as *const u8).read_volatile() } != 0x5a
    {
        let _ = munmap(madvise_mapping, MADVISE_PROBE_BYTES);
        fail(USER_FAIL_MADVISE_DONTNEED, 282);
    }
    if munmap(madvise_mapping, MADVISE_PROBE_BYTES) != 0 {
        fail(USER_FAIL_MADVISE_DONTNEED, 279);
    }
    report_madvise_elapsed_nanoseconds(madvise_elapsed_ns);
    if write(ASSERT_MADVISE_DONTNEED) != ASSERT_MADVISE_DONTNEED.len() as isize {
        fail(USER_FAIL_WRITE, 280);
    }

    // Linux resolves a zero-length splice before flags, descriptors, or user offsets.
    // The deliberately invalid scalar pointer values must therefore never be touched.
    // SAFETY: no Rust pointer is constructed or dereferenced; this call intentionally
    // supplies invalid raw values to verify that the zero-length ABI returns first.
    if unsafe {
        syscall6(
            SYS_SPLICE,
            usize::MAX,
            1,
            usize::MAX,
            1,
            0,
            usize::MAX,
        )
    } != 0
    {
        fail(USER_FAIL_SPLICE_PIPE, 105);
    }

    let mut source_pipe = [-1_i32; 2];
    let mut destination_pipe = [-1_i32; 2];
    if pipe2(&mut source_pipe, 0) != 0 || pipe2(&mut destination_pipe, 0) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 106);
    }
    const SPLICE_PAYLOAD: &[u8; 12] = b"splice-smoke";
    if pipe_write(source_pipe[1], SPLICE_PAYLOAD) != SPLICE_PAYLOAD.len() as isize {
        fail(USER_FAIL_SPLICE_PIPE, 107);
    }

    // fd acquisition precedes offset copying. An invalid input fd plus an invalid raw
    // offset must report EBADF without dereferencing the offset.
    // SAFETY: the raw value 1 is intentionally not a Rust pointer and must be rejected
    // before access because fd_in is invalid; all other arguments are bounded scalars.
    if unsafe {
        syscall6(
            SYS_SPLICE,
            usize::MAX,
            1,
            destination_pipe[1] as usize,
            0,
            1,
            0,
        )
    } != NEG_EBADF
    {
        fail(USER_FAIL_SPLICE_PIPE, 108);
    }

    // Pipe offsets are rejected as ESPIPE before the kernel reads the pointed-to value.
    // SAFETY: the deliberately invalid raw offset is never made into a Rust reference;
    // the live pipe endpoint requires the kernel to reject it before user-memory access.
    if unsafe {
        syscall6(
            SYS_SPLICE,
            source_pipe[0] as usize,
            1,
            destination_pipe[1] as usize,
            0,
            1,
            0,
        )
    } != NEG_ESPIPE
    {
        fail(USER_FAIL_SPLICE_PIPE, 109);
    }

    if splice(source_pipe[1], destination_pipe[1], 1, 0) != NEG_EBADF {
        fail(USER_FAIL_SPLICE_PIPE, 110);
    }

    // Different descriptors can still identify the same pipe backing object. The
    // rejected transfer must leave the source bytes available for the valid splice.
    if splice(
        source_pipe[0],
        source_pipe[1],
        SPLICE_PAYLOAD.len(),
        0,
    ) != NEG_EINVAL
    {
        fail(USER_FAIL_SPLICE_PIPE, 111);
    }
    if splice(
        source_pipe[0],
        destination_pipe[1],
        SPLICE_PAYLOAD.len(),
        0,
    ) != SPLICE_PAYLOAD.len() as isize
    {
        fail(USER_FAIL_SPLICE_PIPE, 112);
    }
    let mut spliced = [0_u8; SPLICE_PAYLOAD.len()];
    if fd_read(destination_pipe[0], &mut spliced) != spliced.len() as isize
        || spliced != *SPLICE_PAYLOAD
    {
        fail(USER_FAIL_SPLICE_PIPE, 113);
    }

    // Close both destination descriptors, install a replacement pipe (which may reuse
    // either fd number), and prove the next transfer uses the newly installed objects.
    if close(destination_pipe[0]) != 0 || close(destination_pipe[1]) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 114);
    }
    destination_pipe = [-1; 2];
    if pipe2(&mut destination_pipe, 0) != 0
        || pipe_write(source_pipe[1], SPLICE_PAYLOAD) != SPLICE_PAYLOAD.len() as isize
        || splice(
            source_pipe[0],
            destination_pipe[1],
            SPLICE_PAYLOAD.len(),
            0,
        ) != SPLICE_PAYLOAD.len() as isize
    {
        fail(USER_FAIL_SPLICE_PIPE, 115);
    }
    spliced = [0; SPLICE_PAYLOAD.len()];
    if fd_read(destination_pipe[0], &mut spliced) != spliced.len() as isize
        || spliced != *SPLICE_PAYLOAD
    {
        fail(USER_FAIL_SPLICE_PIPE, 116);
    }
    for fd in source_pipe.into_iter().chain(destination_pipe) {
        if close(fd) != 0 {
            fail(USER_FAIL_SPLICE_PIPE, 117);
        }
    }

    // A splice into a full O_NONBLOCK destination must not consume its source even
    // without SPLICE_F_NONBLOCK. Filling the destination through its normal writer
    // exercises capacity contention without a timing-dependent race; the subsequent
    // source read proves byte preservation.
    let mut preserved_source = [-1_i32; 2];
    let mut full_destination = [-1_i32; 2];
    if pipe2(&mut preserved_source, 0) != 0
        || pipe2(&mut full_destination, O_NONBLOCK) != 0
        || pipe_write(preserved_source[1], SPLICE_PAYLOAD) != SPLICE_PAYLOAD.len() as isize
    {
        fail(USER_FAIL_SPLICE_PIPE, 118);
    }
    let fill = [0x5a_u8; 512];
    let mut filled = 0usize;
    loop {
        let result = pipe_write(full_destination[1], &fill);
        if result > 0 {
            filled = filled.saturating_add(result as usize);
            if filled > 1024 * 1024 {
                fail(USER_FAIL_SPLICE_PIPE, 119);
            }
        } else if result == NEG_EAGAIN {
            break;
        } else {
            fail(USER_FAIL_SPLICE_PIPE, 120);
        }
    }
    if filled == 0
        || splice(
            preserved_source[0],
            full_destination[1],
            SPLICE_PAYLOAD.len(),
            0,
        ) != NEG_EAGAIN
    {
        fail(USER_FAIL_SPLICE_PIPE, 121);
    }
    let mut preserved = [0_u8; SPLICE_PAYLOAD.len()];
    if fd_read(preserved_source[0], &mut preserved) != preserved.len() as isize
        || preserved != *SPLICE_PAYLOAD
    {
        fail(USER_FAIL_SPLICE_PIPE, 122);
    }
    for fd in preserved_source.into_iter().chain(full_destination) {
        if close(fd) != 0 {
            fail(USER_FAIL_SPLICE_PIPE, 123);
        }
    }

    // Linux validates tee flags first, returns zero for a zero-length request before
    // resolving either descriptor, then resolves both descriptors before checking
    // access modes and finally pipe type/backing identity. Stdio supplies stable
    // live non-pipe descriptions with known access modes: fd 0 is read-only and fd 1
    // is write-only. Every rejection below must precede any wait on the empty pipe.
    let mut tee_pipe = [-1_i32; 2];
    if pipe2(&mut tee_pipe, 0) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 124);
    }
    if tee(-1, -1, 0, 0) != 0 || tee(-1, -1, 0, usize::MAX) != NEG_EINVAL {
        fail(USER_FAIL_SPLICE_PIPE, 224);
    }
    if tee(1, tee_pipe[1], 1, 0) != NEG_EBADF
        || tee(tee_pipe[0], 0, 1, 0) != NEG_EBADF
        || tee(0, -1, 1, 0) != NEG_EBADF
        || tee(0, tee_pipe[0], 1, 0) != NEG_EBADF
        || tee(tee_pipe[1], 1, 1, 0) != NEG_EBADF
    {
        fail(USER_FAIL_SPLICE_PIPE, 225);
    }
    if tee(0, tee_pipe[1], 1, 0) != NEG_EINVAL
        || tee(tee_pipe[0], 1, 1, 0) != NEG_EINVAL
        || tee(tee_pipe[0], tee_pipe[1], 1, 0) != NEG_EINVAL
    {
        fail(USER_FAIL_SPLICE_PIPE, 125);
    }

    // Device-backed descriptions must retain the access mode supplied to openat.
    // Wrong-direction endpoints fail with EBADF before the correctly directed live
    // non-pipe combinations reach the EINVAL type check.
    let dev_null_read = openat(b"/dev/null\0", O_RDONLY);
    let dev_null_write = openat(b"/dev/null\0", O_WRONLY);
    if dev_null_read < 0 || dev_null_write < 0 {
        fail(USER_FAIL_TEE_DEVICE_OPEN, 226);
    }
    let dev_null_read = dev_null_read as i32;
    let dev_null_write = dev_null_write as i32;
    if tee(tee_pipe[0], dev_null_read, 1, 0) != NEG_EBADF
        || tee(dev_null_write, tee_pipe[1], 1, 0) != NEG_EBADF
    {
        fail(USER_FAIL_TEE_DEVICE_MODE, 227);
    }
    if tee(dev_null_read, tee_pipe[1], 1, 0) != NEG_EINVAL
        || tee(tee_pipe[0], dev_null_write, 1, 0) != NEG_EINVAL
    {
        fail(USER_FAIL_TEE_DEVICE_MODE, 228);
    }
    if close(dev_null_read) != 0 || close(dev_null_write) != 0 {
        fail(USER_FAIL_TEE_DEVICE_CLOSE, 229);
    }
    if close(tee_pipe[0]) != 0 || close(tee_pipe[1]) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 126);
    }

    // Exercise blocking vmsplice across an iovec boundary larger than a pipe.
    // Each successful partial result is drained before retrying, and every byte is
    // checked in order. A syscall that has already copied bytes must not wait for
    // capacity while advancing to the next vector.
    let mut vmsplice_pipe = [-1_i32; 2];
    if pipe2(&mut vmsplice_pipe, 0) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 127);
    }
    let mut total = 0usize;
    let mut calls = 0usize;
    while total < VMSPLICE_TOTAL_LEN {
        let (iovecs, count) = if total < VMSPLICE_FIRST_LEN {
            let first = &VMSPLICE_FIRST[total..];
            (
                [
                    IoVec {
                        base: first.as_ptr() as usize,
                        len: first.len(),
                    },
                    IoVec {
                        base: VMSPLICE_SECOND.as_ptr() as usize,
                        len: VMSPLICE_SECOND.len(),
                    },
                ],
                2,
            )
        } else {
            (
                [
                    IoVec {
                        base: VMSPLICE_SECOND.as_ptr() as usize,
                        len: VMSPLICE_SECOND.len(),
                    },
                    IoVec { base: 0, len: 0 },
                ],
                1,
            )
        };
        let result = vmsplice(vmsplice_pipe[1], &iovecs[..count], 0);
        let remaining = VMSPLICE_TOTAL_LEN - total;
        if result <= 0 || result as usize > remaining {
            fail(USER_FAIL_SPLICE_PIPE, 128);
        }
        let moved = result as usize;
        let mut drained = 0usize;
        let mut read_buffer = [0_u8; 512];
        while drained < moved {
            let requested = (moved - drained).min(read_buffer.len());
            let read = fd_read(vmsplice_pipe[0], &mut read_buffer[..requested]);
            if read <= 0 || read as usize > requested {
                fail(USER_FAIL_SPLICE_PIPE, 129);
            }
            let read = read as usize;
            for (index, byte) in read_buffer[..read].iter().enumerate() {
                if *byte != expected_vmsplice_byte(total + drained + index) {
                    fail(USER_FAIL_SPLICE_PIPE, 130);
                }
            }
            drained += read;
        }

        total += moved;
        calls += 1;
        if calls > 1024 {
            fail(USER_FAIL_SPLICE_PIPE, 131);
        }
    }
    if close(vmsplice_pipe[0]) != 0 || close(vmsplice_pipe[1]) != 0 {
        fail(USER_FAIL_SPLICE_PIPE, 132);
    }
    if write(ASSERT_SPLICE_PIPE) != ASSERT_SPLICE_PIPE.len() as isize {
        fail(USER_FAIL_WRITE, 133);
    }

    // Cargo and rustc use the versioned clone3 ABI for both worker threads and
    // process spawning on current glibc. Exercise its common process form without
    // libc fallback, including Linux's extensible struct-size contract: undersized
    // input is invalid, a zero future tail is accepted, and a nonzero unknown tail
    // is rejected with E2BIG. Every successful child is reaped by exact pid.
    if clone3_process(core::ptr::null(), core::mem::size_of::<CloneArgs>()) != NEG_EFAULT
        || clone3_process(
            &CloneArgs::fork(),
            core::mem::size_of::<CloneArgs>() - 4 * core::mem::size_of::<u64>(),
        ) != NEG_EINVAL
    {
        fail(USER_FAIL_CLONE3_PROCESS, 257);
    }
    let clone3_child = clone3_process(&CloneArgs::fork(), core::mem::size_of::<CloneArgs>());
    if clone3_child == 0 {
        exit(0);
    }
    if clone3_child < 0 {
        fail(USER_FAIL_CLONE3_PROCESS, 258);
    }
    let mut clone3_status = -1_i32;
    if wait_child(clone3_child, &mut clone3_status) != clone3_child || clone3_status != 0 {
        fail(USER_FAIL_CLONE3_PROCESS, 259);
    }

    let extended = ExtendedCloneArgs {
        args: CloneArgs::fork(),
        future_field: 0,
    };
    let extended_child = clone3_process(
        &extended.args,
        core::mem::size_of::<ExtendedCloneArgs>(),
    );
    if extended_child == 0 {
        exit(0);
    }
    if extended_child < 0 {
        fail(USER_FAIL_CLONE3_PROCESS, 260);
    }
    let mut extended_status = -1_i32;
    if wait_child(extended_child, &mut extended_status) != extended_child || extended_status != 0 {
        fail(USER_FAIL_CLONE3_PROCESS, 261);
    }
    let nonzero_tail = ExtendedCloneArgs {
        args: CloneArgs::fork(),
        future_field: 1,
    };
    if clone3_process(
        &nonzero_tail.args,
        core::mem::size_of::<ExtendedCloneArgs>(),
    ) != NEG_E2BIG
    {
        fail(USER_FAIL_CLONE3_PROCESS, 262);
    }
    if write(ASSERT_CLONE3_PROCESS) != ASSERT_CLONE3_PROCESS.len() as isize {
        fail(USER_FAIL_WRITE, 263);
    }

    // Current glibc uses this exact clone3 flag and argument shape for the worker
    // threads that drive Cargo and rustc. The child begins on the supplied stack,
    // proves that CLONE_SETTLS installed the requested architecture TLS register,
    // and blocks on a shared pipe while the parent observes CLONE_PARENT_SETTID.
    // Releasing the child then requires CLONE_CHILD_CLEARTID to publish zero and wake
    // the exact FUTEX_WAIT_BITSET|FUTEX_CLOCK_REALTIME join used by glibc.
    let mut thread_ready_pipe = [-1_i32; 2];
    let mut thread_release_pipe = [-1_i32; 2];
    if pipe2(&mut thread_ready_pipe, 0) != 0 || pipe2(&mut thread_release_pipe, 0) != 0 {
        fail(USER_FAIL_CLONE3_THREAD, 264);
    }
    CLONE3_THREAD_TID.store(0, Ordering::Release);
    let thread_stack = core::ptr::addr_of_mut!(CLONE3_THREAD_STACK).cast::<u8>() as usize;
    let thread_tid = CLONE3_THREAD_TID.as_ptr() as usize;
    let thread_tls = core::ptr::addr_of!(CLONE3_THREAD_TLS_ANCHOR) as usize;
    let thread_args = CloneArgs::cargo_thread(thread_stack, thread_tid, thread_tls);
    // SAFETY: both pipes are live, the static stack is exclusively reserved for this
    // one child until its clear-child-tid publication, and every pointer remains valid.
    let clone3_thread = unsafe {
        clone3_cargo_thread(
            &thread_args,
            thread_ready_pipe[1],
            thread_release_pipe[0],
            CLONE3_THREAD_CHILD,
            thread_tls,
        )
    };
    if clone3_thread <= 0 {
        let _ = close(thread_ready_pipe[0]);
        let _ = close(thread_ready_pipe[1]);
        let _ = close(thread_release_pipe[0]);
        let _ = close(thread_release_pipe[1]);
        fail(USER_FAIL_CLONE3_THREAD, 265);
    }

    let mut thread_ready = [0_u8; 1];
    let ready_result = fd_read(thread_ready_pipe[0], &mut thread_ready);
    let parent_tid_seen = CLONE3_THREAD_TID.load(Ordering::Acquire) == clone3_thread as i32;
    let release_result = pipe_write(thread_release_pipe[1], b"G");
    let futex_join_result = futex_wait_clear_tid(&CLONE3_THREAD_TID, clone3_thread as i32);
    let clear_tid_seen = CLONE3_THREAD_TID.load(Ordering::Acquire) == 0;
    // Linux permits the child to clear the word just before the waiter enters the
    // kernel. In that legitimate race the exact futex call returns EAGAIN; accept it
    // only when the acquire load proves that clear_child_tid already published zero.
    let futex_join_ok = clear_tid_seen && matches!(futex_join_result, 0 | NEG_EAGAIN);
    let mut thread_write_status = [0_u8; core::mem::size_of::<isize>()];
    let write_status_result = fd_read(thread_ready_pipe[0], &mut thread_write_status);
    let thread_marker_write = isize::from_ne_bytes(thread_write_status);
    // CLONE_FILES shares the descriptor table itself, so descriptors may only be
    // closed after the child has consumed its release byte and exited.
    let close_ok = (close(thread_ready_pipe[0]) == 0)
        & (close(thread_ready_pipe[1]) == 0)
        & (close(thread_release_pipe[0]) == 0)
        & (close(thread_release_pipe[1]) == 0);
    if ready_result != 1
        || thread_ready != [b'R']
        || !parent_tid_seen
        || release_result != 1
        || write_status_result != core::mem::size_of::<isize>() as isize
        || !close_ok
    {
        fail(USER_FAIL_CLONE3_THREAD, 266);
    }
    if !futex_join_ok {
        fail(USER_FAIL_CLONE3_FUTEX_JOIN, 270);
    }
    if thread_marker_write != CLONE3_THREAD_CHILD.len() as isize {
        report_clone3_thread_write_result(thread_marker_write);
        let marker = match thread_marker_write {
            NEG_EBADF => USER_FAIL_CLONE3_THREAD_WRITE_EBADF,
            NEG_EFAULT => USER_FAIL_CLONE3_THREAD_WRITE_EFAULT,
            _ => USER_FAIL_CLONE3_THREAD_WRITE_OTHER,
        };
        fail(marker, 268);
    }
    if write(ASSERT_CLONE3_THREAD) != ASSERT_CLONE3_THREAD.len() as isize {
        fail(USER_FAIL_WRITE, 267);
    }
    if write(ASSERT_CLONE3_FUTEX_JOIN) != ASSERT_CLONE3_FUTEX_JOIN.len() as isize {
        fail(USER_FAIL_WRITE, 271);
    }

    // glibc's posix_spawn path uses clone3(CLONE_VM|CLONE_VFORK) with an explicit
    // stack and SIGCHLD. The child may touch only async-signal-safe state before
    // exec, so the raw assembly closes/redirects inherited descriptors, publishes
    // one shared pre-exec stage, and replaces the image with the independent helper.
    // The parent must remain suspended until exec commits: clone3 returning before
    // the stage publication is a vfork contract failure.
    let mut vfork_pipe = [-1_i32; 2];
    if pipe2(&mut vfork_pipe, 0) != 0 {
        fail(USER_FAIL_CLONE3_VFORK_EXEC, 269);
    }
    let vfork_stack = core::ptr::addr_of_mut!(CLONE3_VFORK_STACK).cast::<u8>() as usize;
    let vfork_args = CloneArgs::vfork(vfork_stack);
    let vfork_argv = [EXEC_HELPER_PATH.as_ptr() as usize, 0];
    let vfork_envp = [0_usize];
    CLONE3_VFORK_STAGE.store(0, Ordering::Release);
    let vfork_context = VforkExecContext {
        stdout_read_fd: vfork_pipe[0] as usize,
        stdout_write_fd: vfork_pipe[1] as usize,
        path: EXEC_HELPER_PATH.as_ptr() as usize,
        argv: vfork_argv.as_ptr() as usize,
        envp: vfork_envp.as_ptr() as usize,
        stage: CLONE3_VFORK_STAGE.as_ptr() as usize,
    };
    // SAFETY: the context, argv/envp, and static explicit stack remain live until
    // clone3 returns after child exec/exit; the child owns the stack exclusively.
    let vfork_child = unsafe { clone3_vfork_exec(&vfork_args, &vfork_context) };
    let vfork_stage = CLONE3_VFORK_STAGE.load(Ordering::Acquire);
    let vfork_parent_write_close = close(vfork_pipe[1]);
    if vfork_child <= 0 {
        let _ = close(vfork_pipe[0]);
        fail(USER_FAIL_CLONE3_VFORK_EXEC, 270);
    }
    if vfork_parent_write_close != 0 {
        let _ = close(vfork_pipe[0]);
        let mut status = -1_i32;
        let _ = wait_child(vfork_child, &mut status);
        fail(USER_FAIL_CLONE3_VFORK_EXEC, 270);
    }
    let mut vfork_output = [0_u8; EXEC_HELPER_PAYLOAD.len()];
    let mut vfork_received = 0usize;
    while vfork_received < vfork_output.len() {
        let result = fd_read(vfork_pipe[0], &mut vfork_output[vfork_received..]);
        if result <= 0 || result as usize > vfork_output.len() - vfork_received {
            break;
        }
        vfork_received += result as usize;
    }
    let mut vfork_trailing = [0_u8; 1];
    let vfork_eof = fd_read(vfork_pipe[0], &mut vfork_trailing);
    let vfork_close = close(vfork_pipe[0]);
    let mut vfork_status = -1_i32;
    let vfork_wait = wait_child(vfork_child, &mut vfork_status);
    if vfork_stage != 1
        || vfork_received != EXEC_HELPER_PAYLOAD.len()
        || vfork_output != *EXEC_HELPER_PAYLOAD
        || vfork_eof != 0
        || vfork_close != 0
        || vfork_wait != vfork_child
        || vfork_status != 0
    {
        fail(USER_FAIL_CLONE3_VFORK_EXEC, 271);
    }
    if write(ASSERT_CLONE3_VFORK_EXEC) != ASSERT_CLONE3_VFORK_EXEC.len() as isize {
        fail(USER_FAIL_WRITE, 272);
    }

    // A libc popen-style operation is built from these same generic primitives:
    // create a pipe, fork, redirect the child's stdout, replace the child image,
    // consume output to EOF, and reap the exact pid. The helper is a separately
    // linked static ELF, so a PASS requires a real exec image transition rather
    // than continued execution in the forked address space.
    let mut exec_pipe = [-1_i32; 2];
    if pipe2(&mut exec_pipe, 0) != 0 {
        fail(USER_FAIL_PIPE_FORK_EXEC, 252);
    }
    let exec_child = fork_process();
    if exec_child == 0 {
        if close(exec_pipe[0]) != 0 {
            exit(31);
        }
        if exec_pipe[1] != 1 {
            if dup3(exec_pipe[1], 1, 0) != 1 || close(exec_pipe[1]) != 0 {
                exit(32);
            }
        }
        let _ = exec_helper();
        exit(33);
    }
    if exec_child < 0 {
        let _ = close(exec_pipe[0]);
        let _ = close(exec_pipe[1]);
        fail(USER_FAIL_PIPE_FORK_EXEC, 253);
    }
    if close(exec_pipe[1]) != 0 {
        let mut status = 0_i32;
        let _ = wait_child(exec_child, &mut status);
        fail(USER_FAIL_PIPE_FORK_EXEC, 254);
    }
    let mut helper_output = [0_u8; EXEC_HELPER_PAYLOAD.len()];
    let mut received = 0usize;
    while received < helper_output.len() {
        let result = fd_read(exec_pipe[0], &mut helper_output[received..]);
        if result <= 0 || result as usize > helper_output.len() - received {
            break;
        }
        received += result as usize;
    }
    let mut trailing = [0_u8; 1];
    let eof = fd_read(exec_pipe[0], &mut trailing);
    let close_result = close(exec_pipe[0]);
    let mut exec_status = -1_i32;
    let wait_result = wait_child(exec_child, &mut exec_status);
    if received != EXEC_HELPER_PAYLOAD.len()
        || helper_output != *EXEC_HELPER_PAYLOAD
        || eof != 0
        || close_result != 0
        || wait_result != exec_child
        || exec_status != 0
    {
        fail(USER_FAIL_PIPE_FORK_EXEC, 255);
    }
    if write(ASSERT_PIPE_FORK_EXEC) != ASSERT_PIPE_FORK_EXEC.len() as isize {
        fail(USER_FAIL_WRITE, 256);
    }

    // Cargo serializes shared cache/package state with blocking flock. Hold an
    // exclusive lock in the parent, then make a child use an independently opened
    // description for the same file. WNOHANG must observe the child still blocked;
    // only the parent's unlock may let it acquire, unlock, and exit successfully.
    let flock_parent_fd = openat_mode(FLOCK_PROBE_PATH, O_CREAT | O_RDWR, 0o600);
    if flock_parent_fd < 0 || flock(flock_parent_fd as i32, LOCK_EX) != 0 {
        fail(USER_FAIL_FLOCK_BLOCKING, 273);
    }
    let mut flock_ready_pipe = [-1_i32; 2];
    if pipe2(&mut flock_ready_pipe, 0) != 0 {
        let _ = flock(flock_parent_fd as i32, LOCK_UN);
        let _ = close(flock_parent_fd as i32);
        fail(USER_FAIL_FLOCK_BLOCKING, 274);
    }
    let flock_child = fork_process();
    if flock_child == 0 {
        let mut code = 41;
        if close(flock_ready_pipe[0]) == 0 && close(flock_parent_fd as i32) == 0 {
            let child_fd = openat_mode(FLOCK_PROBE_PATH, O_RDWR, 0);
            if child_fd >= 0 && pipe_write(flock_ready_pipe[1], b"R") == 1 {
                if flock(child_fd as i32, LOCK_EX) == 0
                    && flock(child_fd as i32, LOCK_UN) == 0
                    && close(child_fd as i32) == 0
                {
                    code = 0;
                } else {
                    code = 42;
                }
            }
        }
        let _ = close(flock_ready_pipe[1]);
        exit(code);
    }
    if flock_child < 0 || close(flock_ready_pipe[1]) != 0 {
        let _ = flock(flock_parent_fd as i32, LOCK_UN);
        let _ = close(flock_parent_fd as i32);
        let _ = close(flock_ready_pipe[0]);
        fail(USER_FAIL_FLOCK_BLOCKING, 275);
    }
    let mut flock_ready = [0_u8; 1];
    let ready = fd_read(flock_ready_pipe[0], &mut flock_ready);
    let settle = Timespec {
        seconds: 0,
        nanoseconds: 50_000_000,
    };
    let sleep_result = nanosleep(&settle);
    let mut early_status = -1_i32;
    let early_wait = wait_child_nohang(flock_child, &mut early_status);
    let unlock_result = flock(flock_parent_fd as i32, LOCK_UN);
    if ready != 1
        || flock_ready != [b'R']
        || sleep_result != 0
        || early_wait != 0
        || unlock_result != 0
    {
        let _ = close(flock_parent_fd as i32);
        let _ = close(flock_ready_pipe[0]);
        let _ = unlinkat(FLOCK_PROBE_PATH);
        fail(USER_FAIL_FLOCK_BLOCKING, 276);
    }
    let mut flock_status = -1_i32;
    let flock_wait = wait_child(flock_child, &mut flock_status);
    let flock_cleanup = (close(flock_parent_fd as i32) == 0)
        & (close(flock_ready_pipe[0]) == 0)
        & (unlinkat(FLOCK_PROBE_PATH) == 0);
    if flock_wait != flock_child || flock_status != 0 || !flock_cleanup {
        fail(USER_FAIL_FLOCK_BLOCKING, 277);
    }
    if write(ASSERT_FLOCK_BLOCKING) != ASSERT_FLOCK_BLOCKING.len() as isize {
        fail(USER_FAIL_WRITE, 278);
    }

    // Rust build tools publish temporary artifacts with ordinary replacement rename.
    // Exercise the namespace shape where the source name is a hardlink alias and the
    // destination is a different existing regular file. The destination must become
    // another name for the source inode; RENAME_NOREPLACE is deliberately not used.
    let _ = unlinkat(HARDLINK_RENAME_ALIAS);
    let _ = unlinkat(HARDLINK_RENAME_TARGET);
    let _ = unlinkat(HARDLINK_RENAME_SOURCE);
    let source_fd = openat_mode(HARDLINK_RENAME_SOURCE, O_CREAT | O_RDWR, 0o600);
    if source_fd < 0
        || pipe_write(source_fd as i32, HARDLINK_RENAME_SOURCE_DATA)
            != HARDLINK_RENAME_SOURCE_DATA.len() as isize
        || close(source_fd as i32) != 0
        || linkat(HARDLINK_RENAME_SOURCE, HARDLINK_RENAME_ALIAS) != 0
    {
        let _ = unlinkat(HARDLINK_RENAME_ALIAS);
        let _ = unlinkat(HARDLINK_RENAME_SOURCE);
        fail(USER_FAIL_HARDLINK_RENAME_REPLACE, 279);
    }
    let target_fd = openat_mode(HARDLINK_RENAME_TARGET, O_CREAT | O_RDWR, 0o600);
    if target_fd < 0
        || pipe_write(target_fd as i32, HARDLINK_RENAME_TARGET_DATA)
            != HARDLINK_RENAME_TARGET_DATA.len() as isize
        || close(target_fd as i32) != 0
    {
        let _ = unlinkat(HARDLINK_RENAME_ALIAS);
        let _ = unlinkat(HARDLINK_RENAME_TARGET);
        let _ = unlinkat(HARDLINK_RENAME_SOURCE);
        fail(USER_FAIL_HARDLINK_RENAME_REPLACE, 280);
    }
    let replace_result = renameat2(HARDLINK_RENAME_ALIAS, HARDLINK_RENAME_TARGET);
    let replaced_data_ok = path_has_exact_data(
        HARDLINK_RENAME_TARGET,
        HARDLINK_RENAME_SOURCE_DATA,
    );
    let source_data_ok = path_has_exact_data(
        HARDLINK_RENAME_SOURCE,
        HARDLINK_RENAME_SOURCE_DATA,
    );
    let old_alias_result = openat(HARDLINK_RENAME_ALIAS, O_RDONLY);
    if old_alias_result >= 0 {
        let _ = close(old_alias_result as i32);
    }
    let cleanup_ok = (unlinkat(HARDLINK_RENAME_TARGET) == 0)
        & (unlinkat(HARDLINK_RENAME_SOURCE) == 0);
    if replace_result != 0
        || !replaced_data_ok
        || !source_data_ok
        || old_alias_result != NEG_ENOENT
        || !cleanup_ok
    {
        let _ = unlinkat(HARDLINK_RENAME_ALIAS);
        let _ = unlinkat(HARDLINK_RENAME_TARGET);
        let _ = unlinkat(HARDLINK_RENAME_SOURCE);
        fail(USER_FAIL_HARDLINK_RENAME_REPLACE, 281);
    }
    if write(ASSERT_HARDLINK_RENAME_REPLACE) != ASSERT_HARDLINK_RENAME_REPLACE.len() as isize {
        fail(USER_FAIL_WRITE, 282);
    }

    // rustc publishes incremental object files by hard-linking them into a working
    // directory and then renaming that directory as a unit. The linked directory
    // entry must be enumerable before publication, and every descendant name must
    // follow the parent rename while preserving inode identity and link counts.
    let _ = unlinkat(CARGO_LINK_PUBLISHED_ALIAS);
    let _ = unlinkat(CARGO_LINK_WORKING_ALIAS);
    let _ = unlinkat_dir(CARGO_LINK_PUBLISHED_DIR);
    let _ = unlinkat_dir(CARGO_LINK_WORKING_DIR);
    let _ = unlinkat(CARGO_LINK_SOURCE);
    let cargo_source_fd = openat_mode(CARGO_LINK_SOURCE, O_CREAT | O_RDWR, 0o600);
    if cargo_source_fd < 0
        || pipe_write(cargo_source_fd as i32, CARGO_LINK_SOURCE_DATA)
            != CARGO_LINK_SOURCE_DATA.len() as isize
        || close(cargo_source_fd as i32) != 0
        || mkdirat(CARGO_LINK_WORKING_DIR, 0o700) != 0
        || linkat(CARGO_LINK_SOURCE, CARGO_LINK_WORKING_ALIAS) != 0
    {
        let _ = unlinkat(CARGO_LINK_WORKING_ALIAS);
        let _ = unlinkat_dir(CARGO_LINK_WORKING_DIR);
        let _ = unlinkat(CARGO_LINK_SOURCE);
        fail(USER_FAIL_CARGO_LINK_PUBLISH, 283);
    }
    let cargo_working_fd = openat(CARGO_LINK_WORKING_DIR, O_RDONLY | O_DIRECTORY);
    let cargo_entry_visible = cargo_working_fd >= 0
        && directory_contains(cargo_working_fd as i32, CARGO_LINK_ENTRY_NAME);
    let cargo_working_close = if cargo_working_fd >= 0 {
        close(cargo_working_fd as i32)
    } else {
        NEG_EBADF
    };
    let cargo_pre_identity_ok = match (
        statx_identity(CARGO_LINK_SOURCE),
        statx_identity(CARGO_LINK_WORKING_ALIAS),
    ) {
        (Some(source), Some(alias)) => {
            source.inode == alias.inode && source.nlink == 2 && alias.nlink == 2
        }
        _ => false,
    };
    if !cargo_entry_visible || cargo_working_close != 0 || !cargo_pre_identity_ok {
        let _ = unlinkat(CARGO_LINK_WORKING_ALIAS);
        let _ = unlinkat_dir(CARGO_LINK_WORKING_DIR);
        let _ = unlinkat(CARGO_LINK_SOURCE);
        fail(USER_FAIL_CARGO_LINK_PUBLISH, 284);
    }

    let cargo_publish_result =
        renameat2(CARGO_LINK_WORKING_DIR, CARGO_LINK_PUBLISHED_DIR);
    let cargo_published_data_ok =
        path_has_exact_data(CARGO_LINK_PUBLISHED_ALIAS, CARGO_LINK_SOURCE_DATA);
    let cargo_old_alias = openat(CARGO_LINK_WORKING_ALIAS, O_RDONLY);
    if cargo_old_alias >= 0 {
        let _ = close(cargo_old_alias as i32);
    }
    let cargo_post_identity_ok = match (
        statx_identity(CARGO_LINK_SOURCE),
        statx_identity(CARGO_LINK_PUBLISHED_ALIAS),
    ) {
        (Some(source), Some(alias)) => {
            source.inode == alias.inode && source.nlink == 2 && alias.nlink == 2
        }
        _ => false,
    };
    if cargo_publish_result != 0
        || !cargo_published_data_ok
        || cargo_old_alias != NEG_ENOENT
        || !cargo_post_identity_ok
    {
        let _ = unlinkat(CARGO_LINK_PUBLISHED_ALIAS);
        let _ = unlinkat(CARGO_LINK_WORKING_ALIAS);
        let _ = unlinkat_dir(CARGO_LINK_PUBLISHED_DIR);
        let _ = unlinkat_dir(CARGO_LINK_WORKING_DIR);
        let _ = unlinkat(CARGO_LINK_SOURCE);
        fail(USER_FAIL_CARGO_LINK_PUBLISH, 285);
    }

    let cargo_alias_cleanup = unlinkat(CARGO_LINK_PUBLISHED_ALIAS);
    let cargo_final_identity_ok = statx_identity(CARGO_LINK_SOURCE)
        .is_some_and(|source| source.nlink == 1);
    let cargo_source_cleanup = unlinkat(CARGO_LINK_SOURCE);
    let cargo_dir_cleanup = unlinkat_dir(CARGO_LINK_PUBLISHED_DIR);
    if cargo_alias_cleanup != 0
        || !cargo_final_identity_ok
        || cargo_source_cleanup != 0
        || cargo_dir_cleanup != 0
    {
        fail(USER_FAIL_CARGO_LINK_PUBLISH, 286);
    }
    if write(ASSERT_CARGO_LINK_PUBLISH) != ASSERT_CARGO_LINK_PUBLISH.len() as isize {
        fail(USER_FAIL_WRITE, 287);
    }

    // CAgent's server and concurrent clients depend on ordinary process creation and
    // blocking IPv4 stream semantics. Bind a reusable loopback listener, fork eight
    // independent clients before accepting any of them, and echo a distinct payload
    // over every connection. The parent then reaps every exact pid and requires a
    // normal zero exit status. This is a generic TCP/fork/wait contract: no evaluator
    // path, process name, or protocol response is visible to the kernel.
    let address = loopback_sockaddr();
    let listener = socket_stream();
    if listener < 0 {
        fail(USER_FAIL_TCP_FORK_LOOPBACK, 242);
    }
    let listener = listener as i32;
    if socket_set_reuseaddr(listener) != 0
        || socket_bind(listener, &address) != 0
        || socket_listen(listener, TCP_FORK_CLIENTS) != 0
    {
        let _ = close(listener);
        fail(USER_FAIL_TCP_FORK_LOOPBACK, 243);
    }

    let mut child_pids = [0_isize; TCP_FORK_CLIENTS];
    for client_index in 0..TCP_FORK_CLIENTS {
        let child_pid = fork_process();
        if child_pid == 0 {
            exit(tcp_fork_child(listener, client_index, &address));
        }
        if child_pid < 0 {
            let _ = close(listener);
            fail(USER_FAIL_TCP_FORK_LOOPBACK, 244);
        }
        child_pids[client_index] = child_pid;
    }

    let mut seen_clients = 0_u16;
    for _ in 0..TCP_FORK_CLIENTS {
        let accepted = socket_accept(listener);
        if accepted < 0 {
            let _ = close(listener);
            fail(USER_FAIL_TCP_FORK_LOOPBACK, 245);
        }
        let accepted = accepted as i32;
        let mut request = [0_u8; 4];
        if !socket_recv_exact(accepted, &mut request) {
            let _ = close(accepted);
            let _ = close(listener);
            fail(USER_FAIL_TCP_FORK_LOOPBACK, 246);
        }
        let client_index = request[1] as usize;
        if request[0] != b'C'
            || client_index >= TCP_FORK_CLIENTS
            || request[2..] != [0x5a, 0xa5]
            || seen_clients & (1_u16 << client_index) != 0
        {
            let _ = close(accepted);
            let _ = close(listener);
            fail(USER_FAIL_TCP_FORK_LOOPBACK, 247);
        }
        seen_clients |= 1_u16 << client_index;
        if !socket_send_all(accepted, &request) || close(accepted) != 0 {
            let _ = close(listener);
            fail(USER_FAIL_TCP_FORK_LOOPBACK, 248);
        }
    }
    if seen_clients != (1_u16 << TCP_FORK_CLIENTS) - 1 || close(listener) != 0 {
        fail(USER_FAIL_TCP_FORK_LOOPBACK, 249);
    }
    for child_pid in child_pids {
        let mut status = -1_i32;
        if wait_child(child_pid, &mut status) != child_pid || status != 0 {
            fail(USER_FAIL_TCP_FORK_LOOPBACK, 250);
        }
    }
    if write(ASSERT_TCP_FORK_LOOPBACK) != ASSERT_TCP_FORK_LOOPBACK.len() as isize {
        fail(USER_FAIL_WRITE, 251);
    }

    let mut uts = UtsName::zeroed();
    // SAFETY: `uts` is a live, uniquely borrowed, writable `repr(C)` Linux utsname
    // buffer for the duration of SYS_UNAME; no Rust reference observes it until the
    // synchronous syscall returns. The remaining argument slots are ignored.
    let uname_result =
        unsafe { syscall6(SYS_UNAME, &mut uts as *mut UtsName as usize, 0, 0, 0, 0, 0) };
    if uname_result != 0 {
        fail(USER_FAIL_UNAME, 134);
    }
    if !c_field_equals(&uts.sysname, b"Linux") {
        fail(USER_FAIL_SYSNAME, 135);
    }
    if write(ASSERT_UNAME_SYSNAME) != ASSERT_UNAME_SYSNAME.len() as isize {
        fail(USER_FAIL_WRITE, 136);
    }
    if !c_field_equals(&uts.machine, EXPECTED_MACHINE) {
        fail(USER_FAIL_MACHINE, 137);
    }
    if write(ASSERT_UNAME_MACHINE) != ASSERT_UNAME_MACHINE.len() as isize {
        fail(USER_FAIL_WRITE, 138);
    }
    if write(USER_PASS) != USER_PASS.len() as isize {
        fail(USER_FAIL_WRITE, 139);
    }
    exit(0)
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    fail(USER_FAIL_PANIC, 120)
}
