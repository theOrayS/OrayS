use axerrno::LinuxError;
use linux_raw_sys::general;
use std::string::String;

#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
pub use orays_linux::abi::constants::AUX_PLATFORM;
pub use orays_linux::abi::constants::{
    ACCESS_MODE_MASK, ACCESS_R_OK, ACCESS_W_OK, ACCESS_X_OK, AF_UNIX_DOMAIN, AUX_CLOCK_TICKS,
    CHOWN_ID_UNCHANGED, CLOSE_RANGE_CLOEXEC, CLOSE_RANGE_UNSHARE, DEVFS_MAGIC, EXT4_SUPER_MAGIC,
    FD_SETSIZE, FILE_MODE_GROUP_EXECUTE, FILE_MODE_PERMISSION_MASK, FILE_MODE_SET_GID,
    FILE_MODE_SET_UID, FILE_MODE_STICKY, IOV_MAX, IP_MCAST_JOIN_GROUP_OPT,
    IP_MCAST_LEAVE_GROUP_OPT, IP_MULTICAST_LOOP_OPT, IP_MULTICAST_TTL_OPT, IP_RECVERR_OPT,
    IP_TOS_OPT, IP_TTL_OPT, IPPROTO_IP_LEVEL, KERNEL_SIGSET_BYTES, LINUX_EACCES,
    LINUX_EAFNOSUPPORT, LINUX_ENOPROTOOPT, LINUX_EOPNOTSUPP, LINUX_EPROTONOSUPPORT,
    LINUX_ESOCKTNOSUPPORT, LINUX_PERSONALITY_QUERY, O_NOFOLLOW_FLAG, O_PATH_FLAG, PER_LINUX,
    PERSONALITY_ADDR_COMPAT_LAYOUT, PERSONALITY_ADDR_LIMIT_3GB, PERSONALITY_ADDR_LIMIT_32BIT,
    PERSONALITY_ADDR_NO_RANDOMIZE, PERSONALITY_FDPIC_FUNCPTRS, PERSONALITY_KNOWN_FLAGS,
    PERSONALITY_MAX_KNOWN_DOMAIN, PERSONALITY_MMAP_PAGE_ZERO, PERSONALITY_PER_MASK,
    PERSONALITY_READ_IMPLIES_EXEC, PERSONALITY_SHORT_INODE, PERSONALITY_STICKY_TIMEOUTS,
    PERSONALITY_UNAME26, PERSONALITY_WHOLE_SECONDS, PIPEFS_MAGIC, PROC_SUPER_MAGIC, RAMFS_MAGIC,
    RLIMIT_FSIZE_RESOURCE, RLIMIT_NOFILE_RESOURCE, RLIMIT_STACK_RESOURCE, RTC_RD_TIME,
    SA_NODEFER_FLAG, SEEK_DATA_WHENCE, SEEK_HOLE_WHENCE, SI_TKILL_CODE, SIG_BLOCK_HOW,
    SIG_SETMASK_HOW, SIG_UNBLOCK_HOW, SIGABRT_NUM, SIGALRM_NUM, SIGCANCEL_NUM, SIGCHLD_NUM,
    SIGCONT_NUM, SIGFPE_NUM, SIGILL_NUM, SIGINT_NUM, SIGKILL_NUM, SIGPIPE_NUM, SIGPROF_NUM,
    SIGQUIT_NUM, SIGSEGV_NUM, SIGSTOP_NUM, SIGTERM_NUM, SIGVTALRM_NUM, SO_ACCEPTCONN_OPT,
    SO_BROADCAST_OPT, SO_DEBUG_OPT, SO_DOMAIN_OPT, SO_DONTROUTE_OPT, SO_ERROR_OPT,
    SO_KEEPALIVE_OPT, SO_LINGER_OPT, SO_NO_CHECK_OPT, SO_OOBINLINE_OPT, SO_PASSCRED_OPT,
    SO_PEERCRED_OPT, SO_PRIORITY_OPT, SO_PROTOCOL_OPT, SO_RCVBUF_OPT, SO_RCVBUFFORCE_OPT,
    SO_RCVTIMEO_OPT, SO_REUSEADDR_OPT, SO_REUSEPORT_OPT, SO_SNDBUF_OPT, SO_SNDBUFFORCE_OPT,
    SO_SNDTIMEO_OPT, SO_TYPE_OPT, SOL_SOCKET_LEVEL, SS_DISABLE, ST_MODE_BLK, ST_MODE_CHR,
    ST_MODE_DIR, ST_MODE_FIFO, ST_MODE_FILE, ST_MODE_LNK, ST_MODE_SOCKET, ST_MODE_TYPE_MASK,
    SYSFS_MAGIC, SYSV_IPC_CREAT, SYSV_IPC_EXCL, SYSV_IPC_INFO, SYSV_IPC_PRIVATE, SYSV_IPC_RMID,
    SYSV_IPC_SET, SYSV_IPC_STAT, SYSV_SHM_EXEC, SYSV_SHM_HUGETLB, SYSV_SHM_INFO, SYSV_SHM_LOCK,
    SYSV_SHM_LOCKED, SYSV_SHM_RDONLY, SYSV_SHM_REMAP, SYSV_SHM_RND, SYSV_SHM_STAT,
    SYSV_SHM_STAT_ANY, SYSV_SHM_UNLOCK, TCP_KEEPCNT_OPT, TCP_KEEPIDLE_OPT, TCP_KEEPINTVL_OPT,
    TCP_MAXSEG_OPT, TCP_NODELAY_OPT, TMPFS_MAGIC,
};

pub(super) const USER_ASPACE_BASE: usize = 0x1_0000;
pub(super) const USER_ASPACE_SIZE: usize = 0x3f_0000_0000;
pub(super) const USER_STACK_SIZE: usize = 8 * 1024 * 1024;
pub(super) const USER_STACK_GUARD: usize = 0x1_0000;
pub(super) const USER_STACK_TOP: usize = USER_ASPACE_BASE + USER_ASPACE_SIZE - USER_STACK_GUARD;
pub(super) const USER_MMAP_BASE: usize = 0x10_0000_0000;
pub(super) const USER_BRK_GROW_SIZE: usize = 64 * 1024 * 1024;
pub(super) const MAX_IN_MEMORY_FILE_SIZE: u64 = 128 * 1024 * 1024;
// The synthetic virtio block devices are sparse-backed, but they must report a
// realistic capacity: Linux block-device helper code rejects very small devices.
pub(super) const SYNTHETIC_BLOCK_DEVICE_SIZE: u64 = 512 * 1024 * 1024;
pub(super) const USER_PIE_LOAD_BASE: usize = USER_ASPACE_BASE;
pub(super) const MAX_SCRIPT_INTERPRETER_DEPTH: usize = 4;
// Match cmd.rs: a compact staging root under writable /tmp keeps guest-visible
// evaluator paths within small POSIX probe buffers without changing directory
// layout semantics.
pub(super) const TESTSUITE_STAGE_ROOT: &str = "/tmp";
pub(super) const LEGACY_TESTSUITE_STAGE_ROOT: &str = "/tmp/testsuite";
pub(super) const DEFAULT_NOFILE_LIMIT: u64 = 1024;
pub(super) const NR_OPEN_LIMIT: u64 = 1024 * 1024;

pub(super) const BITS_PER_USIZE: usize = usize::BITS as usize;
pub(super) const FD_SET_WORDS: usize = FD_SETSIZE.div_ceil(BITS_PER_USIZE);

#[cfg(target_arch = "riscv64")]
pub(super) const RISCV_SIGNAL_SIGSET_RESERVED_BYTES: usize = 120;
#[cfg(target_arch = "riscv64")]
pub(super) const RISCV_SIGNAL_FPSTATE_BYTES: usize = 528;
#[cfg(target_arch = "riscv64")]
pub(super) const RISCV_SIGTRAMP_CODE: [u32; 3] = [0x08b0_0893, 0x0000_0073, 0x0010_0073];
#[cfg(target_arch = "loongarch64")]
pub(super) const LOONGARCH_SIGTRAMP_CODE: [u32; 3] = [0x0282_2c0b, 0x002b_0000, 0x0000_0000];

pub(super) const STATFS_BLOCK_SIZE: i64 = 4096;
pub(super) const STATFS_NAME_MAX: i64 = 63;
pub(super) const SYSV_SHM_MAX_SIZE: usize = 1024 * 1024;
pub(super) const SYSV_SHM_MAX_SEGMENTS: usize = 128;

pub(super) const PROC_SELF_MAPS_PATH: &str = "/proc/self/maps";
pub(super) const ETC_PASSWD_PATH: &str = "/etc/passwd";
pub(super) const ETC_GROUP_PATH: &str = "/etc/group";

pub(super) const LOCAL_SOCKET_INO_BASE: u64 = 0x5f00_0000;
pub(super) const DEFAULT_PASSWD_CONTENT: &[u8] =
    b"root:x:0:0:root:/root:/bin/sh\nnobody:x:65534:65534:nobody:/nonexistent:/sbin/nologin\n";
pub(super) const DEFAULT_GROUP_CONTENT: &[u8] =
    b"root:x:0:\ndaemon:x:1:\nusers:x:100:\nnogroup:x:65534:\n";
pub(super) const TCP_INFO_COMPAT_SIZE: usize = 256;
pub(super) const INTERRUPTIBLE_SOCKET_RECV_QUANTUM: core::time::Duration =
    core::time::Duration::from_millis(20);
pub(super) fn posix_errno_from_ret(ret: isize) -> LinuxError {
    LinuxError::try_from((-ret) as i32).unwrap_or(LinuxError::EIO)
}

pub(super) fn posix_ret_usize(ret: isize) -> Result<usize, LinuxError> {
    if ret < 0 {
        Err(posix_errno_from_ret(ret))
    } else {
        Ok(ret as usize)
    }
}

pub(super) fn posix_ret_i32(ret: i32) -> Result<i32, LinuxError> {
    if ret < 0 {
        Err(posix_errno_from_ret(ret as isize))
    } else {
        Ok(ret)
    }
}

pub(super) fn neg_errno(err: LinuxError) -> isize {
    -(err.code() as isize)
}

pub(super) fn neg_errno_code(code: u32) -> isize {
    -(code as isize)
}

pub(super) fn fd_cloexec_flag(enabled: bool) -> u32 {
    if enabled { general::FD_CLOEXEC } else { 0 }
}

pub(super) fn str_err(err: &'static str) -> String {
    err.into()
}
