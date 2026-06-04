use axerrno::LinuxError;
use axhal::context::TrapFrame;
use axhal::trap::{register_trap_handler, SYSCALL};
use linux_raw_sys::general;

use super::credentials::{
    sys_capget, sys_capset, sys_getgroups, sys_getresgid, sys_getresuid, sys_setfsgid,
    sys_setfsuid, sys_setgid, sys_setgroups, sys_setregid, sys_setresgid, sys_setresuid,
    sys_setreuid, sys_setuid,
};
use super::fd_pipe::sys_pipe2;
use super::fd_socket::{
    sys_accept_bridge, sys_bind_bridge, sys_connect_bridge, sys_getpeername_bridge,
    sys_getsockname_bridge, sys_getsockopt_bridge, sys_listen_bridge, sys_recvfrom_bridge,
    sys_recvmsg_bridge, sys_sendmsg_bridge, sys_sendto_bridge, sys_setsockopt_bridge,
    sys_shutdown_bridge, sys_socket_bridge, sys_socketpair_bridge,
};
use super::fd_table::{
    sys_chdir, sys_close, sys_close_range, sys_copy_file_range, sys_dup, sys_dup3,
    sys_epoll_create1, sys_epoll_ctl, sys_epoll_pwait, sys_epoll_pwait2, sys_eventfd2,
    sys_fadvise64, sys_fallocate, sys_fchdir, sys_fcntl, sys_flock, sys_fsync, sys_ftruncate,
    sys_getcwd, sys_getdents64, sys_ioctl, sys_linkat, sys_lseek, sys_memfd_create, sys_mkdirat,
    sys_mknodat, sys_openat, sys_pread64, sys_preadv, sys_preadv2, sys_pwrite64, sys_pwritev,
    sys_pwritev2, sys_read, sys_readahead, sys_readv, sys_renameat2, sys_sendfile, sys_signalfd4,
    sys_splice, sys_timerfd_create, sys_timerfd_gettime, sys_timerfd_settime, sys_unlinkat,
    sys_write, sys_writev,
};
use super::futex::sys_futex;
use super::linux_abi::neg_errno;
use super::memory_map::{
    sys_brk, sys_madvise, sys_mincore, sys_mlock, sys_mlockall, sys_mmap, sys_mprotect, sys_mremap,
    sys_msync, sys_munlock, sys_munlockall, sys_munmap,
};
use super::memory_policy::{sys_get_mempolicy, sys_mbind, sys_set_mempolicy};
use super::metadata::{
    sys_faccessat, sys_fchmod, sys_fchmodat, sys_fchown, sys_fchownat, sys_fgetxattr,
    sys_flistxattr, sys_fremovexattr, sys_fsetxattr, sys_fstat, sys_fstatfs, sys_getxattr,
    sys_lgetxattr, sys_listxattr, sys_llistxattr, sys_lremovexattr, sys_lsetxattr, sys_newfstatat,
    sys_readlinkat, sys_removexattr, sys_setxattr, sys_statfs, sys_statx, sys_symlinkat,
    sys_truncate, sys_umask, sys_utimensat,
};
use super::mount_abi::{sys_mount, sys_umount2};
use super::process_abi::{sys_getpgid, sys_getsid, sys_personality, sys_setpgid, sys_setsid};
use super::process_lifecycle::{
    sys_clone, sys_execve, sys_exit, sys_exit_group, sys_wait4, sys_waitid,
    terminate_current_thread_for_exit_group,
};
use super::resource_sched::{
    sys_getpriority, sys_ioprio_get, sys_ioprio_set, sys_prlimit64, sys_sched_get_priority_max,
    sys_sched_get_priority_min, sys_sched_getaffinity, sys_sched_getattr, sys_sched_getparam,
    sys_sched_getscheduler, sys_sched_rr_get_interval, sys_sched_setaffinity, sys_sched_setattr,
    sys_sched_setparam, sys_sched_setscheduler, sys_sched_yield, sys_setpriority,
};
#[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
use super::resource_sched::{sys_getrlimit, sys_setrlimit};
#[cfg(not(any(
    target_arch = "riscv64",
    target_arch = "aarch64",
    target_arch = "loongarch64"
)))]
use super::select_fdset::sys_poll;
use super::select_fdset::{sys_ppoll, sys_pselect6};
use super::signal_abi::{
    sys_kill, sys_rt_sigaction, sys_rt_sigpending, sys_rt_sigprocmask, sys_rt_sigreturn,
    sys_rt_sigsuspend, sys_rt_sigtimedwait, sys_sigaltstack, sys_tgkill, sys_tkill,
};
use super::system_info::{
    sys_getcpu, sys_getrusage, sys_prctl, sys_setdomainname, sys_sethostname, sys_sysinfo,
    sys_syslog, sys_uname,
};
use super::sysv_shm::{sys_shmat, sys_shmctl, sys_shmdt, sys_shmget};
use super::task_context::{
    current_process, set_current_user_pc, sys_get_robust_list, sys_set_robust_list,
    sys_set_tid_address, user_pc,
};
use super::time_abi::{
    sys_adjtimex, sys_clock_adjtime, sys_clock_getres, sys_clock_gettime, sys_clock_nanosleep,
    sys_clock_settime, sys_getitimer, sys_gettimeofday, sys_nanosleep, sys_setitimer,
    sys_timer_create, sys_timer_delete, sys_timer_getoverrun, sys_timer_gettime, sys_timer_settime,
    sys_times,
};
use super::user_memory::sys_getrandom;

#[cfg(target_arch = "loongarch64")]
const LOONGARCH_LEGACY_GETRLIMIT: u32 = 163;
#[cfg(target_arch = "loongarch64")]
const LOONGARCH_LEGACY_SETRLIMIT: u32 = 164;

#[register_trap_handler(SYSCALL)]
fn user_syscall(tf: &TrapFrame, syscall_num: usize) -> isize {
    let Some(process) = current_process() else {
        return neg_errno(LinuxError::ENOSYS);
    };
    set_current_user_pc(user_pc(tf));
    match syscall_num as u32 {
        general::__NR_exit | general::__NR_exit_group => {}
        _ => {
            if process.eval_watchdog_expired() {
                process.request_exit_group(137);
            }
            if let Some(code) = process.pending_exit_group() {
                user_trace!(
                    "user-exit-group-syscall: tid={} code={code} syscall={} sp={:#x} ra={:#x} pc={:#x}",
                    current_tid(),
                    syscall_num,
                    tf.regs.sp,
                    tf.regs.ra,
                    user_pc(tf),
                );
                terminate_current_thread_for_exit_group(process.as_ref(), code);
            }
        }
    };
    match syscall_num as u32 {
        general::__NR_read => sys_read(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_pread64 => sys_pread64(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3()),
        general::__NR_write => sys_write(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_pwrite64 => {
            sys_pwrite64(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_writev => sys_writev(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_readv => sys_readv(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_preadv => sys_preadv(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3()),
        general::__NR_pwritev => sys_pwritev(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3()),
        general::__NR_preadv2 => sys_preadv2(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_pwritev2 => sys_pwritev2(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_sendfile => {
            sys_sendfile(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_splice => sys_splice(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_readahead => sys_readahead(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_copy_file_range => sys_copy_file_range(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_statfs => sys_statfs(&process, tf.arg0(), tf.arg1()),
        general::__NR_fstatfs => sys_fstatfs(&process, tf.arg0(), tf.arg1()),
        general::__NR_sysinfo => sys_sysinfo(&process, tf.arg0()),
        general::__NR_getcwd => sys_getcwd(&process, tf.arg0(), tf.arg1()),
        general::__NR_chdir => sys_chdir(&process, tf.arg0()),
        general::__NR_openat => sys_openat(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3()),
        general::__NR_mkdirat => sys_mkdirat(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_mknodat => sys_mknodat(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3()),
        general::__NR_unlinkat => sys_unlinkat(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_symlinkat => sys_symlinkat(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_linkat => sys_linkat(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_mount => sys_mount(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_umount2 => sys_umount2(&process, tf.arg0(), tf.arg1()),
        general::__NR_pipe2 => sys_pipe2(&process, tf.arg0(), tf.arg1()),
        general::__NR_fallocate => {
            sys_fallocate(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_fadvise64 => {
            sys_fadvise64(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_truncate => sys_truncate(&process, tf.arg0(), tf.arg1()),
        general::__NR_ftruncate => sys_ftruncate(&process, tf.arg0(), tf.arg1()),
        general::__NR_fchmod => sys_fchmod(&process, tf.arg0(), tf.arg1()),
        general::__NR_setxattr => sys_setxattr(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_lsetxattr => sys_lsetxattr(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_fsetxattr => sys_fsetxattr(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_getxattr => {
            sys_getxattr(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_lgetxattr => {
            sys_lgetxattr(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_fgetxattr => {
            sys_fgetxattr(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_listxattr => sys_listxattr(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_llistxattr => sys_llistxattr(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_flistxattr => sys_flistxattr(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_removexattr => sys_removexattr(&process, tf.arg0(), tf.arg1()),
        general::__NR_lremovexattr => sys_lremovexattr(&process, tf.arg0(), tf.arg1()),
        general::__NR_fremovexattr => sys_fremovexattr(&process, tf.arg0(), tf.arg1()),
        general::__NR_fchmodat => sys_fchmodat(&process, tf.arg0(), tf.arg1(), tf.arg2(), 0),
        general::__NR_fchmodat2 => {
            sys_fchmodat(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_fchown => sys_fchown(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_fchownat => sys_fchownat(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_faccessat => sys_faccessat(&process, tf.arg0(), tf.arg1(), tf.arg2(), 0),
        general::__NR_faccessat2 => {
            sys_faccessat(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_utimensat => {
            sys_utimensat(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_renameat2 => sys_renameat2(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_close => sys_close(&process, tf.arg0()),
        general::__NR_close_range => sys_close_range(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_fsync | general::__NR_fdatasync => sys_fsync(&process, tf.arg0()),
        general::__NR_newfstatat => {
            sys_newfstatat(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_statx => sys_statx(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_fstat => sys_fstat(&process, tf.arg0(), tf.arg1()),
        general::__NR_getdents64 => sys_getdents64(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_lseek => sys_lseek(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_dup => sys_dup(&process, tf.arg0()),
        general::__NR_dup3 => sys_dup3(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_fcntl => sys_fcntl(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_memfd_create => sys_memfd_create(&process, tf.arg0(), tf.arg1()),
        general::__NR_flock => sys_flock(&process, tf.arg0(), tf.arg1()),
        general::__NR_fchdir => sys_fchdir(&process, tf.arg0()),
        general::__NR_readlinkat => {
            sys_readlinkat(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_socket => sys_socket_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_socketpair => {
            sys_socketpair_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_bind => sys_bind_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_listen => sys_listen_bridge(&process, tf.arg0(), tf.arg1()),
        general::__NR_accept => sys_accept_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2(), 0),
        general::__NR_accept4 => {
            sys_accept_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_connect => sys_connect_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_sendto => sys_sendto_bridge(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_sendmsg => sys_sendmsg_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_recvfrom => sys_recvfrom_bridge(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_recvmsg => sys_recvmsg_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_shutdown => sys_shutdown_bridge(&process, tf.arg0(), tf.arg1()),
        general::__NR_getsockname => {
            sys_getsockname_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2())
        }
        general::__NR_getpeername => {
            sys_getpeername_bridge(&process, tf.arg0(), tf.arg1(), tf.arg2())
        }
        general::__NR_setsockopt => sys_setsockopt_bridge(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_getsockopt => sys_getsockopt_bridge(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_pselect6 => sys_pselect6(
            &process,
            tf.arg0() as i32,
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_ppoll => sys_ppoll(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        #[cfg(not(any(
            target_arch = "riscv64",
            target_arch = "aarch64",
            target_arch = "loongarch64"
        )))]
        general::__NR_poll => sys_poll(&process, tf.arg0(), tf.arg1(), tf.arg2() as i32),
        general::__NR_eventfd2 => sys_eventfd2(&process, tf.arg0(), tf.arg1()),
        general::__NR_timerfd_create => sys_timerfd_create(&process, tf.arg0(), tf.arg1()),
        general::__NR_timerfd_settime => {
            sys_timerfd_settime(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_timerfd_gettime => sys_timerfd_gettime(&process, tf.arg0(), tf.arg1()),
        general::__NR_signalfd4 => {
            sys_signalfd4(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_epoll_create1 => sys_epoll_create1(&process, tf.arg0()),
        general::__NR_epoll_ctl => {
            sys_epoll_ctl(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_epoll_pwait => sys_epoll_pwait(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3() as isize,
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_epoll_pwait2 => sys_epoll_pwait2(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_ioctl => sys_ioctl(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_clock_gettime => sys_clock_gettime(&process, tf.arg0(), tf.arg1()),
        general::__NR_clock_settime => sys_clock_settime(&process, tf.arg0(), tf.arg1()),
        general::__NR_clock_getres => sys_clock_getres(&process, tf.arg0(), tf.arg1()),
        general::__NR_gettimeofday => sys_gettimeofday(&process, tf.arg0(), tf.arg1()),
        general::__NR_adjtimex => sys_adjtimex(&process, tf.arg0()),
        general::__NR_clock_adjtime => sys_clock_adjtime(&process, tf.arg0(), tf.arg1()),
        general::__NR_getrandom => sys_getrandom(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_getitimer => sys_getitimer(&process, tf.arg0() as i32, tf.arg1()),
        general::__NR_setitimer => sys_setitimer(&process, tf.arg0() as i32, tf.arg1(), tf.arg2()),
        general::__NR_timer_create => sys_timer_create(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_timer_settime => {
            sys_timer_settime(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_timer_gettime => sys_timer_gettime(&process, tf.arg0(), tf.arg1()),
        general::__NR_timer_getoverrun => sys_timer_getoverrun(&process, tf.arg0()),
        general::__NR_timer_delete => sys_timer_delete(&process, tf.arg0()),
        general::__NR_times => sys_times(&process, tf.arg0()),
        general::__NR_getrusage => sys_getrusage(&process, tf.arg0() as i32, tf.arg1()),
        general::__NR_setpriority => sys_setpriority(
            &process,
            tf.arg0() as u32,
            tf.arg1() as i32,
            tf.arg2() as i32,
        ),
        general::__NR_getpriority => sys_getpriority(&process, tf.arg0() as u32, tf.arg1() as i32),
        general::__NR_ioprio_set => sys_ioprio_set(
            &process,
            tf.arg0() as u32,
            tf.arg1() as i32,
            tf.arg2() as u32,
        ),
        general::__NR_ioprio_get => sys_ioprio_get(&process, tf.arg0() as u32, tf.arg1() as i32),
        general::__NR_uname => sys_uname(&process, tf.arg0()),
        general::__NR_nanosleep => sys_nanosleep(&process, tf.arg0(), tf.arg1()),
        general::__NR_clock_nanosleep => {
            sys_clock_nanosleep(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_sched_yield => sys_sched_yield(tf),
        general::__NR_sched_setparam => sys_sched_setparam(&process, tf.arg0() as i32, tf.arg1()),
        general::__NR_sched_getparam => sys_sched_getparam(&process, tf.arg0() as i32, tf.arg1()),
        general::__NR_sched_setscheduler => {
            sys_sched_setscheduler(&process, tf.arg0() as i32, tf.arg1() as i32, tf.arg2())
        }
        general::__NR_sched_getscheduler => sys_sched_getscheduler(&process, tf.arg0() as i32),
        general::__NR_sched_get_priority_max => sys_sched_get_priority_max(tf.arg0() as i32),
        general::__NR_sched_get_priority_min => sys_sched_get_priority_min(tf.arg0() as i32),
        general::__NR_sched_rr_get_interval => {
            sys_sched_rr_get_interval(&process, tf.arg0() as i32, tf.arg1())
        }
        general::__NR_sched_setattr => {
            sys_sched_setattr(&process, tf.arg0() as i32, tf.arg1(), tf.arg2())
        }
        general::__NR_sched_getattr => {
            sys_sched_getattr(&process, tf.arg0() as i32, tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_sched_setaffinity => {
            sys_sched_setaffinity(&process, tf.arg0() as i32, tf.arg1(), tf.arg2())
        }
        general::__NR_sched_getaffinity => {
            sys_sched_getaffinity(&process, tf.arg0() as i32, tf.arg1(), tf.arg2())
        }
        general::__NR_syslog => sys_syslog(&process, tf.arg0() as i32, tf.arg1(), tf.arg2()),
        general::__NR_getcpu => sys_getcpu(&process, tf.arg0(), tf.arg1()),
        general::__NR_gettid => axtask::current().id().as_u64() as isize,
        general::__NR_brk => sys_brk(&process, tf.arg0()),
        general::__NR_shmget => sys_shmget(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_shmat => sys_shmat(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_shmdt => sys_shmdt(&process, tf, tf.arg0()),
        general::__NR_shmctl => sys_shmctl(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_mmap => sys_mmap(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_madvise => sys_madvise(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_mincore => sys_mincore(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_mprotect => sys_mprotect(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_msync => sys_msync(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_mremap => sys_mremap(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_munmap => sys_munmap(&process, tf, tf.arg0(), tf.arg1()),
        general::__NR_mbind => sys_mbind(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_get_mempolicy => sys_get_mempolicy(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_set_mempolicy => sys_set_mempolicy(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_mlock => sys_mlock(&process, tf.arg0(), tf.arg1()),
        general::__NR_mlock2 => {
            if tf.arg2() == 0 {
                sys_mlock(&process, tf.arg0(), tf.arg1())
            } else {
                0
            }
        }
        general::__NR_munlock => sys_munlock(&process, tf.arg0(), tf.arg1()),
        general::__NR_mlockall => sys_mlockall(&process, tf.arg0()),
        general::__NR_munlockall => sys_munlockall(&process),
        general::__NR_set_tid_address => sys_set_tid_address(tf, tf.arg0()),
        general::__NR_set_robust_list => sys_set_robust_list(tf.arg0(), tf.arg1()),
        general::__NR_get_robust_list => {
            sys_get_robust_list(&process, tf.arg0() as i32, tf.arg1(), tf.arg2())
        }
        general::__NR_futex => sys_futex(
            &process,
            tf,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
            tf.arg5(),
        ),
        general::__NR_getuid => process.real_uid() as isize,
        general::__NR_geteuid => process.uid() as isize,
        general::__NR_getgid => process.real_gid() as isize,
        general::__NR_getegid => process.gid() as isize,
        general::__NR_setuid => sys_setuid(&process, tf.arg0()),
        general::__NR_setgid => sys_setgid(&process, tf.arg0()),
        general::__NR_setreuid => sys_setreuid(&process, tf.arg0(), tf.arg1()),
        general::__NR_setregid => sys_setregid(&process, tf.arg0(), tf.arg1()),
        general::__NR_setresuid => sys_setresuid(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_getresuid => sys_getresuid(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_setresgid => sys_setresgid(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_getresgid => sys_getresgid(&process, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_setfsuid => sys_setfsuid(&process, tf.arg0()),
        general::__NR_setfsgid => sys_setfsgid(&process, tf.arg0()),
        general::__NR_getgroups => sys_getgroups(&process, tf.arg0(), tf.arg1()),
        general::__NR_setgroups => sys_setgroups(&process, tf.arg0(), tf.arg1()),
        general::__NR_umask => sys_umask(&process, tf.arg0()),
        general::__NR_personality => sys_personality(&process, tf.arg0()),
        general::__NR_prctl => sys_prctl(
            &process,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_sethostname => sys_sethostname(&process, tf.arg0(), tf.arg1()),
        general::__NR_setdomainname => sys_setdomainname(&process, tf.arg0(), tf.arg1()),
        general::__NR_setpgid => sys_setpgid(&process, tf.arg0(), tf.arg1()),
        general::__NR_getpgid => sys_getpgid(&process, tf.arg0()),
        general::__NR_getsid => sys_getsid(&process, tf.arg0()),
        general::__NR_setsid => sys_setsid(&process),
        general::__NR_kill => sys_kill(&process, tf.arg0() as i32, tf.arg1() as i32),
        general::__NR_tkill => sys_tkill(&process, tf.arg0() as i32, tf.arg1() as i32),
        general::__NR_tgkill => sys_tgkill(
            &process,
            tf.arg0() as i32,
            tf.arg1() as i32,
            tf.arg2() as i32,
        ),
        general::__NR_rt_sigtimedwait => {
            sys_rt_sigtimedwait(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_rt_sigsuspend => sys_rt_sigsuspend(&process, tf.arg0(), tf.arg1()),
        general::__NR_rt_sigpending => sys_rt_sigpending(&process, tf.arg0(), tf.arg1()),
        general::__NR_rt_sigaction => {
            sys_rt_sigaction(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_rt_sigreturn => sys_rt_sigreturn(&process),
        general::__NR_sigaltstack => sys_sigaltstack(&process, tf.arg0(), tf.arg1()),
        general::__NR_rt_sigprocmask => {
            sys_rt_sigprocmask(&process, tf.arg0(), tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_prlimit64 => sys_prlimit64(
            &process,
            tf.arg0() as i32,
            tf.arg1() as u32,
            tf.arg2(),
            tf.arg3(),
        ),
        #[cfg(target_arch = "riscv64")]
        general::__NR_getrlimit => sys_getrlimit(&process, tf.arg0() as u32, tf.arg1()),
        #[cfg(target_arch = "riscv64")]
        general::__NR_setrlimit => sys_setrlimit(&process, tf.arg0() as u32, tf.arg1()),
        #[cfg(target_arch = "loongarch64")]
        LOONGARCH_LEGACY_GETRLIMIT => sys_getrlimit(&process, tf.arg0() as u32, tf.arg1()),
        #[cfg(target_arch = "loongarch64")]
        LOONGARCH_LEGACY_SETRLIMIT => sys_setrlimit(&process, tf.arg0() as u32, tf.arg1()),
        general::__NR_getpid => process.pid() as isize,
        general::__NR_getppid => process.ppid() as isize,
        general::__NR_capget => sys_capget(&process, tf.arg0(), tf.arg1()),
        general::__NR_capset => sys_capset(&process, tf.arg0(), tf.arg1()),
        #[cfg(not(target_arch = "loongarch64"))]
        general::__NR_clone => sys_clone(
            &process,
            tf,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        #[cfg(target_arch = "loongarch64")]
        general::__NR_clone => sys_clone(
            &process,
            tf,
            tf.arg0(),
            tf.arg1(),
            tf.arg2(),
            tf.arg4(),
            tf.arg3(),
        ),
        general::__NR_execve => sys_execve(&process, tf, tf.arg0(), tf.arg1(), tf.arg2()),
        general::__NR_wait4 => {
            sys_wait4(&process, tf.arg0() as i32, tf.arg1(), tf.arg2(), tf.arg3())
        }
        general::__NR_waitid => sys_waitid(
            &process,
            tf.arg0() as u32,
            tf.arg1() as i32,
            tf.arg2(),
            tf.arg3(),
            tf.arg4(),
        ),
        general::__NR_exit => sys_exit(process.as_ref(), tf, tf.arg0() as i32),
        general::__NR_exit_group => sys_exit_group(process.as_ref(), tf, tf.arg0() as i32),
        _ => neg_errno(LinuxError::ENOSYS),
    }
}
