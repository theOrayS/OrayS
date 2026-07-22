//! Linux UAPI constants that do not depend on an OrayS backend.

/// `AT_CLKTCK` auxiliary vector value: clock ticks per second (USER_HZ).
pub const AUX_CLOCK_TICKS: usize = 100;
/// `SEEK_DATA` whence value for `lseek`.
pub const SEEK_DATA_WHENCE: u32 = 3;
/// `SEEK_HOLE` whence value for `lseek`.
pub const SEEK_HOLE_WHENCE: u32 = 4;

/// `SIGCHLD` signal number.
pub const SIGCHLD_NUM: isize = 17;
/// `SIGCONT` signal number.
pub const SIGCONT_NUM: i32 = 18;
/// `SIGINT` signal number.
pub const SIGINT_NUM: i32 = 2;
/// `SIGQUIT` signal number.
pub const SIGQUIT_NUM: i32 = 3;
/// `SIGILL` signal number.
pub const SIGILL_NUM: i32 = 4;
/// `SIGABRT` signal number.
pub const SIGABRT_NUM: i32 = 6;
/// `SIGFPE` signal number.
pub const SIGFPE_NUM: i32 = 8;
/// `SIGKILL` signal number.
pub const SIGKILL_NUM: i32 = 9;
/// `SIGSEGV` signal number.
pub const SIGSEGV_NUM: i32 = 11;
/// `SIGPIPE` signal number.
pub const SIGPIPE_NUM: i32 = 13;
/// `SIGALRM` signal number.
pub const SIGALRM_NUM: i32 = 14;
/// `SIGTERM` signal number.
pub const SIGTERM_NUM: i32 = 15;
/// `SIGSTOP` signal number.
pub const SIGSTOP_NUM: i32 = 19;
/// `SIGVTALRM` signal number.
pub const SIGVTALRM_NUM: i32 = 26;
/// `SIGPROF` signal number.
pub const SIGPROF_NUM: i32 = 27;
/// `SIGCANCEL` signal number used by Linux NPTL.
pub const SIGCANCEL_NUM: i32 = 33;
/// `SI_TKILL` `si_code` value identifying `tkill`/`tgkill` signals.
pub const SI_TKILL_CODE: i32 = -6;
/// `SA_NODEFER` `sigaction` flag.
pub const SA_NODEFER_FLAG: u64 = 0x4000_0000;
/// Size in bytes of the kernel `sigset_t` at the syscall boundary.
pub const KERNEL_SIGSET_BYTES: usize = core::mem::size_of::<u64>();
/// `SIG_BLOCK` `how` value for `rt_sigprocmask`.
pub const SIG_BLOCK_HOW: usize = 0;
/// `SIG_UNBLOCK` `how` value for `rt_sigprocmask`.
pub const SIG_UNBLOCK_HOW: usize = 1;
/// `SIG_SETMASK` `how` value for `rt_sigprocmask`.
pub const SIG_SETMASK_HOW: usize = 2;
/// `SS_DISABLE` `sigaltstack` flag disabling the alternate signal stack.
pub const SS_DISABLE: i32 = 2;

/// `RLIMIT_STACK` resource number.
pub const RLIMIT_STACK_RESOURCE: u32 = 3;
/// `RLIMIT_FSIZE` resource number.
pub const RLIMIT_FSIZE_RESOURCE: u32 = 1;
/// `RLIMIT_NOFILE` resource number.
pub const RLIMIT_NOFILE_RESOURCE: u32 = 7;
/// Maximum descriptor count of a Linux `fd_set`.
pub const FD_SETSIZE: usize = 1024;
/// Maximum iovec count accepted by vectored IO syscalls.
pub const IOV_MAX: usize = 1024;

/// `S_IFDIR` file-type bits of `st_mode`.
pub const ST_MODE_DIR: u32 = 0o040000;
/// `S_IFREG` file-type bits of `st_mode`.
pub const ST_MODE_FILE: u32 = 0o100000;
/// `S_IFLNK` file-type bits of `st_mode`.
pub const ST_MODE_LNK: u32 = 0o120000;
/// `S_IFCHR` file-type bits of `st_mode`.
pub const ST_MODE_CHR: u32 = 0o020000;
/// `S_IFBLK` file-type bits of `st_mode`.
pub const ST_MODE_BLK: u32 = 0o060000;
/// `S_IFIFO` file-type bits of `st_mode`.
pub const ST_MODE_FIFO: u32 = 0o010000;
/// `S_IFSOCK` file-type bits of `st_mode`.
pub const ST_MODE_SOCKET: u32 = 0o140000;
/// `S_IFMT` file-type mask of `st_mode`.
pub const ST_MODE_TYPE_MASK: u32 = 0o170000;
/// Permission bits mask of a file mode (`07777`).
pub const FILE_MODE_PERMISSION_MASK: u32 = 0o7777;
/// `S_ISUID` set-user-ID mode bit.
pub const FILE_MODE_SET_UID: u32 = 0o4000;
/// `S_ISGID` set-group-ID mode bit.
pub const FILE_MODE_SET_GID: u32 = 0o2000;
/// `S_ISVTX` sticky mode bit.
pub const FILE_MODE_STICKY: u32 = 0o1000;
/// `S_IXGRP` group-execute mode bit.
pub const FILE_MODE_GROUP_EXECUTE: u32 = 0o0010;
/// Sentinel UID/GID value meaning "unchanged" for `chown`-family calls.
pub const CHOWN_ID_UNCHANGED: u32 = u32::MAX;

/// `statfs` magic of the ext4 filesystem.
pub const EXT4_SUPER_MAGIC: i64 = 0xef53;
/// `statfs` magic of ramfs.
pub const RAMFS_MAGIC: i64 = 0x8584_58f6;
/// `statfs` magic of tmpfs.
pub const TMPFS_MAGIC: i64 = 0x0102_1994;
/// `statfs` magic of procfs.
pub const PROC_SUPER_MAGIC: i64 = 0x9fa0;
/// `statfs` magic of sysfs.
pub const SYSFS_MAGIC: i64 = 0x6265_6572;
/// `statfs` magic of devfs.
pub const DEVFS_MAGIC: i64 = 0x1373;
/// `statfs` magic of pipefs.
pub const PIPEFS_MAGIC: i64 = 0x5049_5045;

/// `IPC_PRIVATE` key value for System V IPC creation.
pub const SYSV_IPC_PRIVATE: i32 = 0;
/// `IPC_CREAT` flag for System V IPC `get` calls.
pub const SYSV_IPC_CREAT: i32 = 0o1000;
/// `IPC_EXCL` flag for System V IPC `get` calls.
pub const SYSV_IPC_EXCL: i32 = 0o2000;
/// `IPC_RMID` control command.
pub const SYSV_IPC_RMID: i32 = 0;
/// `IPC_SET` control command.
pub const SYSV_IPC_SET: i32 = 1;
/// `IPC_STAT` control command.
pub const SYSV_IPC_STAT: i32 = 2;
/// `IPC_INFO` control command.
pub const SYSV_IPC_INFO: i32 = 3;
/// `SHM_HUGETLB` `shmget` flag.
pub const SYSV_SHM_HUGETLB: i32 = 0o4000;
/// `SHM_RDONLY` `shmat` flag.
pub const SYSV_SHM_RDONLY: i32 = 0o10000;
/// `SHM_RND` `shmat` flag.
pub const SYSV_SHM_RND: i32 = 0o20000;
/// `SHM_REMAP` `shmat` flag.
pub const SYSV_SHM_REMAP: i32 = 0o40000;
/// `SHM_EXEC` `shmat` flag.
pub const SYSV_SHM_EXEC: i32 = 0o100000;
/// `SHM_LOCK` `shmctl` command.
pub const SYSV_SHM_LOCK: i32 = 11;
/// `SHM_UNLOCK` `shmctl` command.
pub const SYSV_SHM_UNLOCK: i32 = 12;
/// `SHM_STAT` `shmctl` command.
pub const SYSV_SHM_STAT: i32 = 13;
/// `SHM_INFO` `shmctl` command.
pub const SYSV_SHM_INFO: i32 = 14;
/// `SHM_STAT_ANY` `shmctl` command.
pub const SYSV_SHM_STAT_ANY: i32 = 15;
/// `SHM_LOCKED` state bit reported in `shm_perm.mode`.
pub const SYSV_SHM_LOCKED: u32 = 0o2000;

/// `O_PATH` open flag.
pub const O_PATH_FLAG: u32 = 0o10000000;
/// `O_NOFOLLOW` open flag.
pub const O_NOFOLLOW_FLAG: u32 = 0o0400000;
/// `CLOSE_RANGE_UNSHARE` flag for `close_range`.
pub const CLOSE_RANGE_UNSHARE: u32 = 1 << 1;
/// `CLOSE_RANGE_CLOEXEC` flag for `close_range`.
pub const CLOSE_RANGE_CLOEXEC: u32 = 1 << 2;

/// `AF_UNIX` address family value.
pub const AF_UNIX_DOMAIN: i32 = 1;
/// `EACCES` errno value.
pub const LINUX_EACCES: u32 = 13;
/// `X_OK` mode bit for `access`/`faccessat`.
pub const ACCESS_X_OK: usize = 1;
/// `W_OK` mode bit for `access`/`faccessat`.
pub const ACCESS_W_OK: usize = 2;
/// `R_OK` mode bit for `access`/`faccessat`.
pub const ACCESS_R_OK: usize = 4;
/// Mask of the valid `access` mode bits.
pub const ACCESS_MODE_MASK: usize = ACCESS_X_OK | ACCESS_W_OK | ACCESS_R_OK;
/// `RTC_RD_TIME` ioctl request code.
pub const RTC_RD_TIME: u32 = 0x8024_7009;

/// `SOL_SOCKET` socket option level.
pub const SOL_SOCKET_LEVEL: i32 = 1;
/// `SO_DEBUG` socket option.
pub const SO_DEBUG_OPT: i32 = 1;
/// `SO_REUSEADDR` socket option.
pub const SO_REUSEADDR_OPT: i32 = 2;
/// `SO_TYPE` socket option.
pub const SO_TYPE_OPT: i32 = 3;
/// `SO_ERROR` socket option.
pub const SO_ERROR_OPT: i32 = 4;
/// `SO_DONTROUTE` socket option.
pub const SO_DONTROUTE_OPT: i32 = 5;
/// `SO_BROADCAST` socket option.
pub const SO_BROADCAST_OPT: i32 = 6;
/// `SO_SNDBUF` socket option.
pub const SO_SNDBUF_OPT: i32 = 7;
/// `SO_RCVBUF` socket option.
pub const SO_RCVBUF_OPT: i32 = 8;
/// `SO_KEEPALIVE` socket option.
pub const SO_KEEPALIVE_OPT: i32 = 9;
/// `SO_OOBINLINE` socket option.
pub const SO_OOBINLINE_OPT: i32 = 10;
/// `SO_NO_CHECK` socket option.
pub const SO_NO_CHECK_OPT: i32 = 11;
/// `SO_PRIORITY` socket option.
pub const SO_PRIORITY_OPT: i32 = 12;
/// `SO_LINGER` socket option.
pub const SO_LINGER_OPT: i32 = 13;
/// `SO_REUSEPORT` socket option.
pub const SO_REUSEPORT_OPT: i32 = 15;
/// `SO_PASSCRED` socket option.
pub const SO_PASSCRED_OPT: i32 = 16;
/// `SO_PEERCRED` socket option.
pub const SO_PEERCRED_OPT: i32 = 17;
/// `SO_RCVTIMEO` socket option.
pub const SO_RCVTIMEO_OPT: i32 = 20;
/// `SO_SNDTIMEO` socket option.
pub const SO_SNDTIMEO_OPT: i32 = 21;
/// `SO_ACCEPTCONN` socket option.
pub const SO_ACCEPTCONN_OPT: i32 = 30;
/// `SO_SNDBUFFORCE` socket option.
pub const SO_SNDBUFFORCE_OPT: i32 = 32;
/// `SO_RCVBUFFORCE` socket option.
pub const SO_RCVBUFFORCE_OPT: i32 = 33;
/// `SO_PROTOCOL` socket option.
pub const SO_PROTOCOL_OPT: i32 = 38;
/// `SO_DOMAIN` socket option.
pub const SO_DOMAIN_OPT: i32 = 39;
/// `IPPROTO_IP` protocol level value.
pub const IPPROTO_IP_LEVEL: i32 = 0;
/// `IP_TOS` IP option.
pub const IP_TOS_OPT: i32 = 1;
/// `IP_TTL` IP option.
pub const IP_TTL_OPT: i32 = 2;
/// `IP_MULTICAST_TTL` IP option.
pub const IP_MULTICAST_TTL_OPT: i32 = 33;
/// `IP_MULTICAST_LOOP` IP option.
pub const IP_MULTICAST_LOOP_OPT: i32 = 34;
/// `IP_ADD_MEMBERSHIP`-style multicast join option value used by Linux.
pub const IP_MCAST_JOIN_GROUP_OPT: i32 = 42;
/// `IP_DROP_MEMBERSHIP`-style multicast leave option value used by Linux.
pub const IP_MCAST_LEAVE_GROUP_OPT: i32 = 45;
/// `IP_RECVERR` IP option.
pub const IP_RECVERR_OPT: i32 = 11;
/// `TCP_NODELAY` TCP option.
pub const TCP_NODELAY_OPT: i32 = 1;
/// `TCP_MAXSEG` TCP option.
pub const TCP_MAXSEG_OPT: i32 = 2;
/// `TCP_KEEPIDLE` TCP option.
pub const TCP_KEEPIDLE_OPT: i32 = 4;
/// `TCP_KEEPINTVL` TCP option.
pub const TCP_KEEPINTVL_OPT: i32 = 5;
/// `TCP_KEEPCNT` TCP option.
pub const TCP_KEEPCNT_OPT: i32 = 6;

/// `ENOPROTOOPT` errno value.
pub const LINUX_ENOPROTOOPT: u32 = 92;
/// `EPROTONOSUPPORT` errno value.
pub const LINUX_EPROTONOSUPPORT: u32 = 93;
/// `ESOCKTNOSUPPORT` errno value.
pub const LINUX_ESOCKTNOSUPPORT: u32 = 94;
/// `EOPNOTSUPP` errno value.
pub const LINUX_EOPNOTSUPP: u32 = 95;
/// `EAFNOSUPPORT` errno value.
pub const LINUX_EAFNOSUPPORT: u32 = 97;

/// Sentinel persona value that queries instead of setting `personality`.
pub const LINUX_PERSONALITY_QUERY: usize = 0xffff_ffff;
/// `PER_LINUX` personality domain.
pub const PER_LINUX: usize = 0;
/// Mask of the personality domain bits.
pub const PERSONALITY_PER_MASK: usize = 0x00ff;
/// Highest personality domain this boundary recognizes.
pub const PERSONALITY_MAX_KNOWN_DOMAIN: usize = 0x0010;
/// `UNAME26` personality flag.
pub const PERSONALITY_UNAME26: usize = 0x002_0000;
/// `ADDR_NO_RANDOMIZE` personality flag.
pub const PERSONALITY_ADDR_NO_RANDOMIZE: usize = 0x004_0000;
/// `FDPIC_FUNCPTRS` personality flag.
pub const PERSONALITY_FDPIC_FUNCPTRS: usize = 0x008_0000;
/// `MMAP_PAGE_ZERO` personality flag.
pub const PERSONALITY_MMAP_PAGE_ZERO: usize = 0x010_0000;
/// `ADDR_COMPAT_LAYOUT` personality flag.
pub const PERSONALITY_ADDR_COMPAT_LAYOUT: usize = 0x020_0000;
/// `READ_IMPLIES_EXEC` personality flag.
pub const PERSONALITY_READ_IMPLIES_EXEC: usize = 0x040_0000;
/// `ADDR_LIMIT_32BIT` personality flag.
pub const PERSONALITY_ADDR_LIMIT_32BIT: usize = 0x080_0000;
/// `SHORT_INODE` personality flag.
pub const PERSONALITY_SHORT_INODE: usize = 0x100_0000;
/// `WHOLE_SECONDS` personality flag.
pub const PERSONALITY_WHOLE_SECONDS: usize = 0x200_0000;
/// `STICKY_TIMEOUTS` personality flag.
pub const PERSONALITY_STICKY_TIMEOUTS: usize = 0x400_0000;
/// `ADDR_LIMIT_3GB` personality flag.
pub const PERSONALITY_ADDR_LIMIT_3GB: usize = 0x800_0000;
/// Mask of every personality flag this boundary recognizes.
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
/// `AT_PLATFORM` auxiliary vector string for the current target.
pub const AUX_PLATFORM: &str = "riscv64";
#[cfg(target_arch = "loongarch64")]
/// `AT_PLATFORM` auxiliary vector string for the current target.
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
