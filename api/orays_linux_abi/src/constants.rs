//! Linux UAPI constants that do not depend on an OrayS backend.

pub const AUX_CLOCK_TICKS: usize = 100;
pub const SEEK_DATA_WHENCE: u32 = 3;
pub const SEEK_HOLE_WHENCE: u32 = 4;

pub const SIGCHLD_NUM: isize = 17;
pub const SIGCONT_NUM: i32 = 18;
pub const SIGINT_NUM: i32 = 2;
pub const SIGQUIT_NUM: i32 = 3;
pub const SIGILL_NUM: i32 = 4;
pub const SIGABRT_NUM: i32 = 6;
pub const SIGFPE_NUM: i32 = 8;
pub const SIGKILL_NUM: i32 = 9;
pub const SIGSEGV_NUM: i32 = 11;
pub const SIGPIPE_NUM: i32 = 13;
pub const SIGALRM_NUM: i32 = 14;
pub const SIGTERM_NUM: i32 = 15;
pub const SIGSTOP_NUM: i32 = 19;
pub const SIGVTALRM_NUM: i32 = 26;
pub const SIGPROF_NUM: i32 = 27;
pub const SIGCANCEL_NUM: i32 = 33;
pub const SI_TKILL_CODE: i32 = -6;
pub const SA_NODEFER_FLAG: u64 = 0x4000_0000;
pub const KERNEL_SIGSET_BYTES: usize = core::mem::size_of::<u64>();
pub const SIG_BLOCK_HOW: usize = 0;
pub const SIG_UNBLOCK_HOW: usize = 1;
pub const SIG_SETMASK_HOW: usize = 2;
pub const SS_DISABLE: i32 = 2;

pub const RLIMIT_STACK_RESOURCE: u32 = 3;
pub const RLIMIT_FSIZE_RESOURCE: u32 = 1;
pub const RLIMIT_NOFILE_RESOURCE: u32 = 7;
pub const FD_SETSIZE: usize = 1024;
pub const IOV_MAX: usize = 1024;

pub const ST_MODE_DIR: u32 = 0o040000;
pub const ST_MODE_FILE: u32 = 0o100000;
pub const ST_MODE_LNK: u32 = 0o120000;
pub const ST_MODE_CHR: u32 = 0o020000;
pub const ST_MODE_BLK: u32 = 0o060000;
pub const ST_MODE_FIFO: u32 = 0o010000;
pub const ST_MODE_SOCKET: u32 = 0o140000;
pub const ST_MODE_TYPE_MASK: u32 = 0o170000;
pub const FILE_MODE_PERMISSION_MASK: u32 = 0o7777;
pub const FILE_MODE_SET_UID: u32 = 0o4000;
pub const FILE_MODE_SET_GID: u32 = 0o2000;
pub const FILE_MODE_STICKY: u32 = 0o1000;
pub const FILE_MODE_GROUP_EXECUTE: u32 = 0o0010;
pub const CHOWN_ID_UNCHANGED: u32 = u32::MAX;

pub const EXT4_SUPER_MAGIC: i64 = 0xef53;
pub const RAMFS_MAGIC: i64 = 0x8584_58f6;
pub const TMPFS_MAGIC: i64 = 0x0102_1994;
pub const PROC_SUPER_MAGIC: i64 = 0x9fa0;
pub const SYSFS_MAGIC: i64 = 0x6265_6572;
pub const DEVFS_MAGIC: i64 = 0x1373;
pub const PIPEFS_MAGIC: i64 = 0x5049_5045;

pub const SYSV_IPC_PRIVATE: i32 = 0;
pub const SYSV_IPC_CREAT: i32 = 0o1000;
pub const SYSV_IPC_EXCL: i32 = 0o2000;
pub const SYSV_IPC_RMID: i32 = 0;
pub const SYSV_IPC_SET: i32 = 1;
pub const SYSV_IPC_STAT: i32 = 2;
pub const SYSV_IPC_INFO: i32 = 3;
pub const SYSV_SHM_HUGETLB: i32 = 0o4000;
pub const SYSV_SHM_RDONLY: i32 = 0o10000;
pub const SYSV_SHM_RND: i32 = 0o20000;
pub const SYSV_SHM_REMAP: i32 = 0o40000;
pub const SYSV_SHM_EXEC: i32 = 0o100000;
pub const SYSV_SHM_LOCK: i32 = 11;
pub const SYSV_SHM_UNLOCK: i32 = 12;
pub const SYSV_SHM_STAT: i32 = 13;
pub const SYSV_SHM_INFO: i32 = 14;
pub const SYSV_SHM_STAT_ANY: i32 = 15;
pub const SYSV_SHM_LOCKED: u32 = 0o2000;

pub const O_PATH_FLAG: u32 = 0o10000000;
pub const O_NOFOLLOW_FLAG: u32 = 0o0400000;
pub const CLOSE_RANGE_UNSHARE: u32 = 1 << 1;
pub const CLOSE_RANGE_CLOEXEC: u32 = 1 << 2;

pub const AF_UNIX_DOMAIN: i32 = 1;
pub const LINUX_EACCES: u32 = 13;
pub const ACCESS_X_OK: usize = 1;
pub const ACCESS_W_OK: usize = 2;
pub const ACCESS_R_OK: usize = 4;
pub const ACCESS_MODE_MASK: usize = ACCESS_X_OK | ACCESS_W_OK | ACCESS_R_OK;
pub const RTC_RD_TIME: u32 = 0x8024_7009;

pub const SOL_SOCKET_LEVEL: i32 = 1;
pub const SO_DEBUG_OPT: i32 = 1;
pub const SO_REUSEADDR_OPT: i32 = 2;
pub const SO_TYPE_OPT: i32 = 3;
pub const SO_ERROR_OPT: i32 = 4;
pub const SO_DONTROUTE_OPT: i32 = 5;
pub const SO_BROADCAST_OPT: i32 = 6;
pub const SO_SNDBUF_OPT: i32 = 7;
pub const SO_RCVBUF_OPT: i32 = 8;
pub const SO_KEEPALIVE_OPT: i32 = 9;
pub const SO_OOBINLINE_OPT: i32 = 10;
pub const SO_NO_CHECK_OPT: i32 = 11;
pub const SO_PRIORITY_OPT: i32 = 12;
pub const SO_LINGER_OPT: i32 = 13;
pub const SO_REUSEPORT_OPT: i32 = 15;
pub const SO_PASSCRED_OPT: i32 = 16;
pub const SO_PEERCRED_OPT: i32 = 17;
pub const SO_RCVTIMEO_OPT: i32 = 20;
pub const SO_SNDTIMEO_OPT: i32 = 21;
pub const SO_ACCEPTCONN_OPT: i32 = 30;
pub const SO_SNDBUFFORCE_OPT: i32 = 32;
pub const SO_RCVBUFFORCE_OPT: i32 = 33;
pub const SO_PROTOCOL_OPT: i32 = 38;
pub const SO_DOMAIN_OPT: i32 = 39;
pub const IPPROTO_IP_LEVEL: i32 = 0;
pub const IP_TOS_OPT: i32 = 1;
pub const IP_TTL_OPT: i32 = 2;
pub const IP_MULTICAST_TTL_OPT: i32 = 33;
pub const IP_MULTICAST_LOOP_OPT: i32 = 34;
pub const IP_MCAST_JOIN_GROUP_OPT: i32 = 42;
pub const IP_MCAST_LEAVE_GROUP_OPT: i32 = 45;
pub const IP_RECVERR_OPT: i32 = 11;
pub const TCP_NODELAY_OPT: i32 = 1;
pub const TCP_MAXSEG_OPT: i32 = 2;
pub const TCP_KEEPIDLE_OPT: i32 = 4;
pub const TCP_KEEPINTVL_OPT: i32 = 5;
pub const TCP_KEEPCNT_OPT: i32 = 6;

pub const LINUX_ENOPROTOOPT: u32 = 92;
pub const LINUX_EPROTONOSUPPORT: u32 = 93;
pub const LINUX_ESOCKTNOSUPPORT: u32 = 94;
pub const LINUX_EOPNOTSUPP: u32 = 95;
pub const LINUX_EAFNOSUPPORT: u32 = 97;

pub const LINUX_PERSONALITY_QUERY: usize = 0xffff_ffff;
pub const PER_LINUX: usize = 0;
pub const PERSONALITY_PER_MASK: usize = 0x00ff;
pub const PERSONALITY_MAX_KNOWN_DOMAIN: usize = 0x0010;
pub const PERSONALITY_UNAME26: usize = 0x002_0000;
pub const PERSONALITY_ADDR_NO_RANDOMIZE: usize = 0x004_0000;
pub const PERSONALITY_FDPIC_FUNCPTRS: usize = 0x008_0000;
pub const PERSONALITY_MMAP_PAGE_ZERO: usize = 0x010_0000;
pub const PERSONALITY_ADDR_COMPAT_LAYOUT: usize = 0x020_0000;
pub const PERSONALITY_READ_IMPLIES_EXEC: usize = 0x040_0000;
pub const PERSONALITY_ADDR_LIMIT_32BIT: usize = 0x080_0000;
pub const PERSONALITY_SHORT_INODE: usize = 0x100_0000;
pub const PERSONALITY_WHOLE_SECONDS: usize = 0x200_0000;
pub const PERSONALITY_STICKY_TIMEOUTS: usize = 0x400_0000;
pub const PERSONALITY_ADDR_LIMIT_3GB: usize = 0x800_0000;
pub const PERSONALITY_KNOWN_FLAGS: usize = PERSONALITY_UNAME26
    | PERSONALITY_ADDR_NO_RANDOMIZE
    | PERSONALITY_FDPIC_FUNCPTRS
    | PERSONALITY_MMAP_PAGE_ZERO
    | PERSONALITY_ADDR_COMPAT_LAYOUT
    | PERSONALITY_READ_IMPLIES_EXEC
    | PERSONALITY_ADDR_LIMIT_32BIT
    | PERSONALITY_SHORT_INODE
    | PERSONALITY_WHOLE_SECONDS
    | PERSONALITY_STICKY_TIMEOUTS
    | PERSONALITY_ADDR_LIMIT_3GB;

#[cfg(target_arch = "riscv64")]
pub const AUX_PLATFORM: &str = "riscv64";
#[cfg(target_arch = "loongarch64")]
pub const AUX_PLATFORM: &str = "loongarch64";

const _: () = {
    assert!(SIGCHLD_NUM == 17);
    assert!(KERNEL_SIGSET_BYTES == 8);
    assert!(ST_MODE_TYPE_MASK == 0o170000);
    assert!(SYSV_IPC_CREAT == 0o1000);
    assert!(RTC_RD_TIME == 0x8024_7009);
    assert!(LINUX_EAFNOSUPPORT == 97);
    assert!(PER_LINUX == 0);
};
