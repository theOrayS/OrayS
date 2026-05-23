use axerrno::LinuxError;
use linux_raw_sys::general;
use memory_addr::PAGE_SIZE_4K;
use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;

use super::UserProcess;
use super::fd_table::{FdEntry, MemoryFileEntry, PathEntry};
use super::linux_abi::{
    DEFAULT_GROUP_CONTENT, DEFAULT_PASSWD_CONTENT, ETC_GROUP_PATH, ETC_PASSWD_PATH,
    PROC_SELF_MAPS_PATH, USER_ASPACE_BASE, USER_STACK_SIZE, USER_STACK_TOP,
};
use super::memory_map::{align_down, align_up};
use super::runtime_paths::normalize_path;
use super::task_registry::user_thread_entry_by_process_pid;

fn proc_self_maps_content(process: &UserProcess) -> Vec<u8> {
    let exec_path = process.exec_path();
    let brk = *process.brk.lock();
    let text_start = USER_ASPACE_BASE;
    let text_end = text_start + PAGE_SIZE_4K;
    let heap_start = align_down(brk.start, PAGE_SIZE_4K);
    let heap_end = align_up(brk.end.max(brk.start + PAGE_SIZE_4K), PAGE_SIZE_4K);
    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top - USER_STACK_SIZE;
    format!(
        "{text_start:08x}-{text_end:08x} r-xp 00000000 00:00 0 {exec_path}\n\
         {heap_start:08x}-{heap_end:08x} rw-p 00000000 00:00 0 [heap]\n\
         {stack_base:08x}-{stack_top:08x} rw-p 00000000 00:00 0 [stack]\n"
    )
    .into_bytes()
}

pub(super) fn is_proc_self_maps_path(path: &str) -> bool {
    normalize_path("/", path).as_deref() == Some(PROC_SELF_MAPS_PATH)
}

pub(super) fn synthetic_file_is_writable_open(flags: u32) -> bool {
    let access = flags & general::O_ACCMODE;
    access == general::O_WRONLY
        || access == general::O_RDWR
        || flags & (general::O_TRUNC | general::O_CREAT) != 0
}

pub(super) fn proc_self_maps_is_writable_open(flags: u32) -> bool {
    synthetic_file_is_writable_open(flags)
}

pub(super) fn proc_self_maps_fd_entry(process: &UserProcess) -> FdEntry {
    FdEntry::MemoryFile(MemoryFileEntry {
        path: PROC_SELF_MAPS_PATH.into(),
        data: Arc::new(proc_self_maps_content(process)),
        offset: 0,
    })
}

pub(super) fn proc_self_maps_path_entry(process: &UserProcess) -> FdEntry {
    let content_len = proc_self_maps_content(process).len();
    FdEntry::Path(PathEntry::synthetic_file(PROC_SELF_MAPS_PATH, content_len))
}

fn proc_stat_target_process(process: &UserProcess, path: &str) -> Option<(i32, UserProcessStat)> {
    let normalized = normalize_path("/", path)?;
    let pid = if normalized == "/proc/self/stat" {
        process.pid()
    } else {
        let rest = normalized.strip_prefix("/proc/")?;
        let pid_text = rest.strip_suffix("/stat")?;
        pid_text.parse::<i32>().ok()?
    };
    if pid == process.pid() {
        return Some((pid, UserProcessStat::from(process)));
    }
    if let Some(entry) = process.child_thread_entry_by_pid(pid) {
        return Some((pid, UserProcessStat::from(entry.process.as_ref())));
    }
    user_thread_entry_by_process_pid(pid)
        .map(|entry| (pid, UserProcessStat::from(entry.process.as_ref())))
}

struct UserProcessStat {
    ppid: i32,
    pgid: i32,
    sid: i32,
    state: char,
    comm: String,
}

impl UserProcessStat {
    fn from(process: &UserProcess) -> Self {
        let exec_path = process.exec_path();
        let comm = exec_path
            .rsplit('/')
            .next()
            .filter(|name| !name.is_empty())
            .unwrap_or("user")
            .chars()
            .take(15)
            .collect();
        let state = if process
            .live_threads
            .load(core::sync::atomic::Ordering::Acquire)
            == 0
        {
            'Z'
        } else if process.is_child_wait_blocked() {
            'S'
        } else {
            'R'
        };
        Self {
            ppid: process.ppid(),
            pgid: process.pgid(),
            sid: process.sid(),
            state,
            comm,
        }
    }
}

fn proc_pid_stat_content(process: &UserProcess, path: &str) -> Option<(String, Vec<u8>)> {
    let normalized = normalize_path("/", path)?;
    let (pid, stat) = proc_stat_target_process(process, normalized.as_str())?;
    let content = format!(
        "{pid} ({}) {} {} {} {} 0 -1 0 0 0 0 0 0 0 0 20 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n",
        stat.comm, stat.state, stat.ppid, stat.pgid, stat.sid
    );
    Some((normalized, content.into_bytes()))
}

pub(super) fn proc_pid_stat_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_pid_stat_content(process, path)?;
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path,
        data: Arc::new(data),
        offset: 0,
    }))
}

pub(super) fn proc_pid_stat_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_pid_stat_content(process, path)?;
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        data.len(),
    )))
}

fn proc_pid_status_content(process: &UserProcess, path: &str) -> Option<(String, Vec<u8>)> {
    let normalized = normalize_path("/", path)?;
    let (pid, stat) = if normalized == "/proc/self/status" {
        (process.pid(), UserProcessStat::from(process))
    } else {
        let rest = normalized.strip_prefix("/proc/")?;
        let pid_text = rest.strip_suffix("/status")?;
        let pid = pid_text.parse::<i32>().ok()?;
        if pid == process.pid() {
            (pid, UserProcessStat::from(process))
        } else if let Some(entry) = process.child_thread_entry_by_pid(pid) {
            (pid, UserProcessStat::from(entry.process.as_ref()))
        } else {
            let entry = user_thread_entry_by_process_pid(pid)?;
            (pid, UserProcessStat::from(entry.process.as_ref()))
        }
    };
    let groups = process
        .groups()
        .into_iter()
        .map(|gid| gid.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let groups = if groups.is_empty() {
        process.gid().to_string()
    } else {
        groups
    };
    let content = format!(
        "Name:\t{}\n\
         State:\t{} ({})\n\
         Tgid:\t{pid}\n\
         Pid:\t{pid}\n\
         PPid:\t{}\n\
         Uid:\t{}\t{}\t{}\t{}\n\
         Gid:\t{}\t{}\t{}\t{}\n\
         Groups:\t{groups}\n",
        stat.comm,
        stat.state,
        stat.state,
        stat.ppid,
        process.real_uid(),
        process.uid(),
        process.saved_uid(),
        process.fs_uid(),
        process.real_gid(),
        process.gid(),
        process.saved_gid(),
        process.fs_gid(),
    );
    Some((normalized, content.into_bytes()))
}

pub(super) fn proc_pid_status_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_pid_status_content(process, path)?;
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path,
        data: Arc::new(data),
        offset: 0,
    }))
}

pub(super) fn proc_pid_status_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_pid_status_content(process, path)?;
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        data.len(),
    )))
}

pub(super) fn proc_exe_link_target(process: &UserProcess, path: &str) -> Option<String> {
    let pid_path = format!("/proc/{}/exe", process.pid());
    (path == "/proc/self/exe" || path == pid_path).then(|| process.exec_path())
}

pub(super) fn synthetic_userdb_content(path: &str) -> Option<(&'static str, &'static [u8])> {
    match normalize_path("/", path).as_deref() {
        Some(ETC_PASSWD_PATH) => Some((ETC_PASSWD_PATH, DEFAULT_PASSWD_CONTENT)),
        Some(ETC_GROUP_PATH) => Some((ETC_GROUP_PATH, DEFAULT_GROUP_CONTENT)),
        _ => None,
    }
}

pub(super) fn synthetic_userdb_fd_entry(path: &'static str, data: &'static [u8]) -> FdEntry {
    FdEntry::MemoryFile(MemoryFileEntry {
        path: path.into(),
        data: Arc::new(data.to_vec()),
        offset: 0,
    })
}

pub(super) fn synthetic_userdb_path_entry(path: &'static str, data: &'static [u8]) -> FdEntry {
    FdEntry::Path(PathEntry::synthetic_file(path, data.len()))
}

pub(super) fn dev_shm_host_path(path: &str) -> Option<String> {
    let normalized = normalize_path("/", path)?;
    let rel = normalized.strip_prefix("/dev/shm/")?;
    if rel.is_empty() {
        return None;
    }
    Some(format!("/tmp/shm/{rel}"))
}

pub(super) fn ensure_dev_shm_dir() -> Result<(), LinuxError> {
    ensure_host_dir("/tmp")?;
    ensure_host_dir("/tmp/shm")
}

fn ensure_host_dir(path: &str) -> Result<(), LinuxError> {
    if axfs::api::metadata(path).is_ok() {
        return Ok(());
    }
    axfs::api::create_dir(path).map_err(LinuxError::from)
}
