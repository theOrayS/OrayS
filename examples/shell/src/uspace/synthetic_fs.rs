use core::sync::atomic::Ordering;

use axerrno::LinuxError;
use linux_raw_sys::general;
use memory_addr::{VirtAddr, PAGE_SIZE_4K};
use std::string::{String, ToString};
use std::sync::Arc;
use std::vec::Vec;

use super::fd_table::{FdEntry, MemoryFileEntry, PathEntry, ProcPagemapEntry};
use super::linux_abi::{
    DEFAULT_GROUP_CONTENT, DEFAULT_PASSWD_CONTENT, ETC_GROUP_PATH, ETC_PASSWD_PATH,
    PROC_SELF_MAPS_PATH, USER_ASPACE_BASE, USER_STACK_SIZE, USER_STACK_TOP,
};
use super::memory_map::{align_down, align_up};
use super::runtime_paths::normalize_path;
use super::task_context::task_ext;
use super::task_registry::{
    user_thread_entries_by_process_pid, user_thread_entry_by_process_pid, user_thread_entry_by_tid,
};
use super::UserProcess;

const PROC_SELF_PAGEMAP_PATH: &str = "/proc/self/pagemap";

fn proc_maps_perms(prot: u32, shared: bool) -> String {
    let mut perms = String::new();
    perms.push(if prot & general::PROT_READ != 0 {
        'r'
    } else {
        '-'
    });
    perms.push(if prot & general::PROT_WRITE != 0 {
        'w'
    } else {
        '-'
    });
    perms.push(if prot & general::PROT_EXEC != 0 {
        'x'
    } else {
        '-'
    });
    perms.push(if shared { 's' } else { 'p' });
    perms
}

fn proc_self_maps_content(process: &UserProcess) -> Vec<u8> {
    let exec_path = process.exec_path();
    let brk = *process.brk.lock();
    let text_start = USER_ASPACE_BASE;
    let text_end = text_start + PAGE_SIZE_4K;
    let heap_start = align_down(brk.start, PAGE_SIZE_4K);
    let heap_end = align_up(brk.end.max(brk.start + PAGE_SIZE_4K), PAGE_SIZE_4K);
    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top - USER_STACK_SIZE;
    let mut content = format!(
        "{text_start:08x}-{text_end:08x} r-xp 00000000 00:00 0 {exec_path}\n\
         {heap_start:08x}-{heap_end:08x} rw-p 00000000 00:00 0 [heap]\n\
         {stack_base:08x}-{stack_top:08x} rw-p 00000000 00:00 0 [stack]\n"
    );
    for region in process.mmap_regions() {
        content.push_str(&format!(
            "{:08x}-{:08x} {} 00000000 00:00 0\n",
            region.start,
            region.end(),
            proc_maps_perms(region.prot, region.shared)
        ));
    }
    content.into_bytes()
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

fn proc_pagemap_target_path(process: &UserProcess, path: &str) -> Option<(String, i32)> {
    let normalized = normalize_path("/", path)?;
    if normalized == PROC_SELF_PAGEMAP_PATH {
        return Some((normalized, process.pid()));
    }
    let rest = normalized.strip_prefix("/proc/")?;
    let pid_text = rest.strip_suffix("/pagemap")?;
    pid_text.parse::<i32>().ok().map(|pid| (normalized, pid))
}

fn proc_pagemap_snapshot(target: &UserProcess, path: String) -> ProcPagemapEntry {
    let brk = *target.brk.lock();
    let text_start = USER_ASPACE_BASE;
    let text_end = text_start + PAGE_SIZE_4K;
    let heap_start = align_down(brk.start, PAGE_SIZE_4K);
    let heap_end = align_up(brk.end.max(brk.start + PAGE_SIZE_4K), PAGE_SIZE_4K);
    let stack_top = align_down(USER_STACK_TOP, PAGE_SIZE_4K);
    let stack_base = stack_top.saturating_sub(USER_STACK_SIZE);
    let mut ranges = Vec::new();
    ranges.push((text_start, text_end));
    ranges.push((heap_start, heap_end));
    ranges.push((stack_base, stack_top));
    for region in target.mmap_regions() {
        ranges.push((region.start, region.end()));
    }
    ranges.sort_by_key(|(start, _)| *start);

    let aspace = target.aspace.lock();
    let mut present_ranges = Vec::<(u64, u64)>::new();
    for (start, end) in ranges {
        let mut page = align_down(start, PAGE_SIZE_4K);
        let end = align_up(end.max(start), PAGE_SIZE_4K);
        while page < end {
            if aspace.page_table().query(VirtAddr::from(page)).is_ok() {
                push_present_pagemap_page(&mut present_ranges, (page / PAGE_SIZE_4K) as u64);
            }
            match page.checked_add(PAGE_SIZE_4K) {
                Some(next) => page = next,
                None => break,
            }
        }
    }
    let max_page = USER_STACK_TOP.div_ceil(PAGE_SIZE_4K) as u64;
    ProcPagemapEntry {
        path,
        present_ranges: Arc::new(present_ranges),
        offset: 0,
        size: max_page.saturating_mul(core::mem::size_of::<u64>() as u64),
    }
}

fn push_present_pagemap_page(ranges: &mut Vec<(u64, u64)>, page_index: u64) {
    if let Some((_, end)) = ranges.last_mut() {
        if *end == page_index {
            *end = end.saturating_add(1);
            return;
        }
        if *end > page_index {
            return;
        }
    }
    ranges.push((page_index, page_index.saturating_add(1)));
}

pub(super) fn proc_pagemap_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, pid) = proc_pagemap_target_path(process, path)?;
    if pid == process.pid() {
        return Some(FdEntry::ProcPagemap(proc_pagemap_snapshot(process, path)));
    }
    user_thread_entry_by_process_pid(pid)
        .map(|entry| FdEntry::ProcPagemap(proc_pagemap_snapshot(entry.process.as_ref(), path)))
}

pub(super) fn proc_pagemap_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, pid) = proc_pagemap_target_path(process, path)?;
    if pid != process.pid() && user_thread_entry_by_process_pid(pid).is_none() {
        return None;
    }
    let max_page = USER_STACK_TOP.div_ceil(PAGE_SIZE_4K);
    Some(FdEntry::Path(PathEntry::synthetic_file(
        path.as_str(),
        max_page.saturating_mul(core::mem::size_of::<u64>()),
    )))
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
    locked_mmap_kb: usize,
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
        let state = if process.live_threads.load(Ordering::Acquire) == 0 {
            'Z'
        } else if process.is_child_wait_blocked()
            || process.is_syscall_wait_blocked()
            || process_has_futex_waiter(process)
        {
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
            locked_mmap_kb: process.locked_mmap_kb(),
        }
    }
}

fn process_has_futex_waiter(process: &UserProcess) -> bool {
    user_thread_entries_by_process_pid(process.pid())
        .into_iter()
        .any(|entry| {
            task_ext(&entry.task)
                .map(|ext| ext.futex_wait.load(Ordering::Acquire) != 0)
                .unwrap_or(false)
        })
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
         VmLck:\t{} kB\n\
         Uid:\t{}\t{}\t{}\t{}\n\
         Gid:\t{}\t{}\t{}\t{}\n\
         Groups:\t{groups}\n",
        stat.comm,
        stat.state,
        stat.state,
        stat.ppid,
        stat.locked_mmap_kb,
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

fn proc_comm_target_process(process: &UserProcess, path: &str) -> Option<(String, Vec<u8>)> {
    let normalized = normalize_path("/", path)?;
    let process_comm_content = |target: &UserProcess| {
        let mut content = target.prctl_name();
        content.push('\n');
        Some((normalized.clone(), content.into_bytes()))
    };
    if normalized == "/proc/self/comm" {
        return process_comm_content(process);
    }
    let task_rest = if let Some(rest) = normalized.strip_prefix("/proc/self/task/") {
        Some(rest)
    } else if let Some(rest) = normalized.strip_prefix("/proc/") {
        if let Some((pid_text, rest)) = rest.split_once("/task/") {
            let pid = pid_text.parse::<i32>().ok()?;
            if pid != process.pid() {
                let entry = user_thread_entry_by_process_pid(pid)?;
                if entry.process.pid() != pid {
                    return None;
                }
            }
            Some(rest)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(rest) = task_rest {
        let tid_text = rest.strip_suffix("/comm")?;
        let tid = tid_text.parse::<i32>().ok()?;
        let target_name = if tid == process.pid() {
            Some(process.prctl_name())
        } else if let Some(entry) = user_thread_entry_by_tid(tid) {
            (entry.process.pid() == process.pid()).then(|| entry.process.prctl_name())
        } else {
            None
        }?;
        let mut content = target_name;
        content.push('\n');
        return Some((normalized, content.into_bytes()));
    }
    if let Some(rest) = normalized.strip_prefix("/proc/") {
        if let Some(pid_text) = rest.strip_suffix("/comm") {
            let pid = pid_text.parse::<i32>().ok()?;
            if pid == process.pid() {
                return process_comm_content(process);
            }
            let entry = user_thread_entry_by_process_pid(pid)?;
            return process_comm_content(entry.process.as_ref());
        }
    }
    None
}

pub(super) fn proc_comm_fd_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_comm_target_process(process, path)?;
    Some(FdEntry::MemoryFile(MemoryFileEntry {
        path,
        data: Arc::new(data),
        offset: 0,
    }))
}

pub(super) fn proc_comm_path_entry(process: &UserProcess, path: &str) -> Option<FdEntry> {
    let (path, data) = proc_comm_target_process(process, path)?;
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
